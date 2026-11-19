//! macOS backend: launchd for services and startup agents.

use super::proc::Runner;
use super::{ServiceAction, current_uid, privilege_error};
use crate::model::{Scope, ServiceRow, ServiceState, StartupRow};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn list_services(runner: &dyn Runner) -> Result<Vec<ServiceRow>, String> {
    let out = runner.run("launchctl", &["list"])?;
    if !out.success {
        return Err(out.failure_reason());
    }
    Ok(parse_launchctl_list(&out.stdout))
}

/// `launchctl list` prints `PID<TAB>Status<TAB>Label`, with a dash for no pid.
pub fn parse_launchctl_list(text: &str) -> Vec<ServiceRow> {
    let mut rows = Vec::new();
    for line in text.lines() {
        let mut fields = line.split('\t');
        let pid_field = match fields.next() {
            Some(value) => value.trim(),
            None => continue,
        };
        if pid_field.is_empty() || pid_field == "PID" || pid_field.starts_with("PID") {
            continue;
        }
        let _status = fields.next();
        let Some(label) = fields.next().map(str::trim) else {
            continue;
        };
        if label.is_empty() {
            continue;
        }
        let pid = pid_field.parse::<u32>().ok();
        rows.push(ServiceRow {
            name: label.to_string(),
            pid,
            description: None,
            state: if pid.is_some() {
                ServiceState::Running
            } else {
                ServiceState::Stopped
            },
            scope: Scope::User,
        });
    }
    rows.sort_by(|a, b| a.name.cmp(&b.name));
    rows
}

pub fn service_action(
    runner: &dyn Runner,
    row: &ServiceRow,
    action: ServiceAction,
) -> Result<(), String> {
    if row.scope == Scope::System {
        return Err(privilege_error("Controlling a system launch daemon"));
    }
    let target = format!("gui/{}/{}", current_uid(), row.name);
    let args: Vec<&str> = match action {
        ServiceAction::Start => vec!["kickstart", &target],
        ServiceAction::Stop => vec!["kill", "SIGTERM", &target],
        ServiceAction::Restart => vec!["kickstart", "-k", &target],
    };
    let out = runner.run("launchctl", &args)?;
    if out.success {
        Ok(())
    } else {
        Err(out.failure_reason())
    }
}

pub fn parse_print_disabled(text: &str) -> HashMap<String, bool> {
    let mut map = HashMap::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('"') {
            continue;
        }
        let Some(end) = trimmed[1..].find('"') else {
            continue;
        };
        let label = &trimmed[1..1 + end];
        let rest = &trimmed[1 + end + 1..];
        if let Some((_, value)) = rest.split_once("=>") {
            map.insert(label.to_string(), value.trim().starts_with("true"));
        }
    }
    map
}

fn disabled_map(runner: &dyn Runner) -> HashMap<String, bool> {
    let target = format!("gui/{}", current_uid());
    match runner.run("launchctl", &["print-disabled", &target]) {
        Ok(out) if out.success => parse_print_disabled(&out.stdout),
        _ => HashMap::new(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlistEntry {
    pub label: String,
    pub program: Option<String>,
    pub args: Vec<String>,
    pub disabled: bool,
}

fn next_string_after(xml: &str, from: usize) -> Option<(String, usize)> {
    let rest = &xml[from..];
    let start = rest.find("<string>")?;
    let value_start = from + start + "<string>".len();
    let end = xml[value_start..].find("</string>")?;
    Some((
        xml[value_start..value_start + end].to_string(),
        value_start + end,
    ))
}

fn key_position(xml: &str, key: &str) -> Option<usize> {
    xml.find(&format!("<key>{key}</key>"))
}

fn plist_string(xml: &str, key: &str) -> Option<String> {
    let at = key_position(xml, key)?;
    next_string_after(xml, at).map(|(value, _)| value)
}

fn plist_array(xml: &str, key: &str) -> Vec<String> {
    let Some(at) = key_position(xml, key) else {
        return Vec::new();
    };
    let Some(array_at) = xml[at..].find("<array>") else {
        return Vec::new();
    };
    let start = at + array_at + "<array>".len();
    let Some(array_end) = xml[start..].find("</array>") else {
        return Vec::new();
    };
    let body = &xml[start..start + array_end];
    let mut values = Vec::new();
    let mut cursor = 0;
    while let Some((value, next)) = next_string_after(body, cursor) {
        values.push(value);
        cursor = next;
    }
    values
}

fn plist_bool(xml: &str, key: &str) -> Option<bool> {
    let at = key_position(xml, key)?;
    let rest = &xml[at..];
    if rest
        .find("<true/>")
        .is_some_and(|t| rest.find("<false/>").is_none_or(|f| t < f))
    {
        Some(true)
    } else if rest.find("<false/>").is_some() {
        Some(false)
    } else {
        None
    }
}

pub fn parse_plist(xml: &str) -> Option<PlistEntry> {
    let label = plist_string(xml, "Label")?;
    let program = plist_string(xml, "Program");
    let args = plist_array(xml, "ProgramArguments");
    let disabled = plist_bool(xml, "Disabled").unwrap_or(false);
    Some(PlistEntry {
        label,
        program,
        args,
        disabled,
    })
}

fn home() -> Result<PathBuf, String> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| "HOME is not set".to_string())
}

fn startup_dirs() -> Vec<(PathBuf, Scope)> {
    let mut dirs = Vec::new();
    if let Ok(home) = home() {
        dirs.push((home.join("Library/LaunchAgents"), Scope::User));
    }
    dirs.push((PathBuf::from("/Library/LaunchAgents"), Scope::System));
    dirs.push((PathBuf::from("/Library/LaunchDaemons"), Scope::System));
    dirs
}

pub fn list_startup(runner: &dyn Runner) -> Result<Vec<StartupRow>, String> {
    let disabled = disabled_map(runner);
    let owner = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
    Ok(collect_startup(&startup_dirs(), &disabled, &owner))
}

fn collect_startup(
    dirs: &[(PathBuf, Scope)],
    disabled: &HashMap<String, bool>,
    owner: &str,
) -> Vec<StartupRow> {
    let mut rows = Vec::new();
    for (dir, scope) in dirs {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("plist") {
                continue;
            }
            let Ok(xml) = std::fs::read_to_string(&path) else {
                continue;
            };
            let Some(parsed) = parse_plist(&xml) else {
                continue;
            };
            let program = parsed
                .program
                .clone()
                .or_else(|| parsed.args.first().cloned())
                .unwrap_or_default();
            let command = if parsed.args.is_empty() {
                program.clone()
            } else {
                parsed.args.join(" ")
            };
            let missing_program = !program.is_empty() && !Path::new(&program).exists();
            let enabled =
                !parsed.disabled && !disabled.get(&parsed.label).copied().unwrap_or(false);
            rows.push(StartupRow {
                name: parsed.label,
                command,
                owner: if *scope == Scope::User {
                    owner.to_string()
                } else {
                    "root".to_string()
                },
                source: path.display().to_string(),
                enabled,
                impact: crate::model::Impact::NotMeasured,
                missing_program,
                scope: *scope,
            });
        }
    }
    rows.sort_by(|a, b| a.name.cmp(&b.name));
    rows
}

pub fn set_startup(runner: &dyn Runner, row: &StartupRow, enabled: bool) -> Result<(), String> {
    if row.scope == Scope::System {
        return Err(privilege_error("Changing a system launch daemon"));
    }
    let target = format!("gui/{}/{}", current_uid(), row.name);
    let verb = if enabled { "enable" } else { "disable" };
    let out = runner.run("launchctl", &[verb, &target])?;
    if out.success {
        Ok(())
    } else {
        Err(out.failure_reason())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::RecordingRunner;
    use crate::platform::proc::CommandOutput;

    #[test]
    fn launchctl_list_parses_running_and_stopped_jobs() {
        let rows = parse_launchctl_list(include_str!("../../tests/fixtures/launchctl_list.txt"));
        assert_eq!(rows.len(), 3);
        let mds = rows
            .iter()
            .find(|r| r.name == "com.apple.metadata.mds")
            .unwrap();
        assert_eq!(mds.pid, Some(312));
        assert_eq!(mds.state, ServiceState::Running);
        let backup = rows
            .iter()
            .find(|r| r.name == "com.example.backup")
            .unwrap();
        assert_eq!(backup.pid, None);
        assert_eq!(backup.state, ServiceState::Stopped);
        assert_eq!(backup.description, None);
    }

    #[test]
    fn print_disabled_parses_values() {
        let map = parse_print_disabled(include_str!(
            "../../tests/fixtures/launchctl_print_disabled.txt"
        ));
        assert_eq!(map.get("com.example.backup"), Some(&true));
        assert_eq!(map.get("com.example.flaky"), Some(&false));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn plist_parses_label_and_arguments() {
        let entry = parse_plist(include_str!("../../tests/fixtures/rectangle.plist")).unwrap();
        assert_eq!(entry.label, "com.knollsoft.Rectangle");
        assert_eq!(entry.args.len(), 2);
        assert_eq!(
            entry.args[0],
            "/Applications/Rectangle.app/Contents/MacOS/Rectangle"
        );
        assert!(!entry.disabled);
    }

    #[test]
    fn plist_parses_missing_program() {
        let entry =
            parse_plist(include_str!("../../tests/fixtures/missing_program.plist")).unwrap();
        assert_eq!(entry.program.as_deref(), Some("/opt/old/tool"));
        assert!(entry.args.is_empty());
    }

    #[test]
    fn startup_collects_agents_and_marks_disabled() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("com.knollsoft.Rectangle.plist"),
            include_str!("../../tests/fixtures/rectangle.plist"),
        )
        .unwrap();
        std::fs::write(
            dir.path().join("gone.plist"),
            include_str!("../../tests/fixtures/missing_program.plist"),
        )
        .unwrap();
        let disabled = HashMap::from([("com.knollsoft.Rectangle".to_string(), true)]);
        let rows = collect_startup(
            &[(dir.path().to_path_buf(), Scope::User)],
            &disabled,
            "michael",
        );
        assert_eq!(rows.len(), 2);
        let rect = rows
            .iter()
            .find(|r| r.name == "com.knollsoft.Rectangle")
            .unwrap();
        assert!(!rect.enabled);
        assert_eq!(
            rect.missing_program,
            !Path::new("/Applications/Rectangle.app/Contents/MacOS/Rectangle").exists()
        );
        let gone = rows.iter().find(|r| r.name == "com.example.gone").unwrap();
        assert!(gone.missing_program);
    }

    #[test]
    fn startup_actions_use_the_user_domain() {
        let runner = RecordingRunner::new();
        runner.respond("launchctl", CommandOutput::ok(""));
        let row = StartupRow {
            name: "com.example.app".into(),
            command: "/bin/true".into(),
            owner: "michael".into(),
            source: "/x.plist".into(),
            enabled: false,
            impact: crate::model::Impact::NotMeasured,
            missing_program: false,
            scope: Scope::User,
        };
        set_startup(&runner, &row, true).unwrap();
        set_startup(&runner, &row, false).unwrap();
        let calls = runner.calls();
        assert_eq!(calls[0].1[0], "enable");
        assert!(calls[0].1[1].starts_with("gui/"));
        assert!(calls[0].1[1].ends_with("/com.example.app"));
        assert_eq!(calls[1].1[0], "disable");
    }

    #[test]
    fn system_scope_is_refused_before_running_anything() {
        let runner = RecordingRunner::new();
        let row = StartupRow {
            name: "com.apple.thing".into(),
            command: "/bin/true".into(),
            owner: "root".into(),
            source: "/Library/LaunchDaemons/x.plist".into(),
            enabled: true,
            impact: crate::model::Impact::NotMeasured,
            missing_program: false,
            scope: Scope::System,
        };
        let err = set_startup(&runner, &row, false).unwrap_err();
        assert!(err.contains("administrator"));
        assert!(runner.calls().is_empty());
    }

    #[test]
    fn service_action_builds_kickstart_and_kill() {
        let runner = RecordingRunner::new();
        runner.respond("launchctl", CommandOutput::ok(""));
        let row = ServiceRow {
            name: "homebrew.mxcl.nginx".into(),
            pid: None,
            description: None,
            state: ServiceState::Stopped,
            scope: Scope::User,
        };
        service_action(&runner, &row, ServiceAction::Start).unwrap();
        service_action(&runner, &row, ServiceAction::Stop).unwrap();
        service_action(&runner, &row, ServiceAction::Restart).unwrap();
        let calls = runner.calls();
        assert_eq!(calls[0].1[0], "kickstart");
        assert_eq!(calls[1].1[0], "kill");
        assert_eq!(calls[1].1[1], "SIGTERM");
        assert_eq!(calls[2].1[0], "kickstart");
        assert_eq!(calls[2].1[1], "-k");
    }
}
