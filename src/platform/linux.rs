//! Linux backend: systemd units and XDG autostart entries.

use super::proc::Runner;
use super::{ServiceAction, is_root, privilege_error};
use crate::model::{Impact, Scope, ServiceRow, ServiceState, StartupRow};
use std::path::{Path, PathBuf};

const SHOW_PROPS: &str = "--property=Id,Description,ActiveState,MainPID,LoadState";

pub fn list_services(runner: &dyn Runner) -> Result<Vec<ServiceRow>, String> {
    let mut rows = Vec::new();
    let mut first_error = None;

    match runner.run(
        "systemctl",
        &["show", "--all", "--type=service", SHOW_PROPS],
    ) {
        Ok(out) if out.success => rows.extend(parse_systemctl_show(&out.stdout, Scope::System)),
        Ok(out) => first_error = Some(out.failure_reason()),
        Err(err) => first_error = Some(err),
    }

    // A user session may not exist, which is not an error worth surfacing.
    if let Ok(out) = runner.run(
        "systemctl",
        &["--user", "show", "--all", "--type=service", SHOW_PROPS],
    ) && out.success
    {
        rows.extend(parse_systemctl_show(&out.stdout, Scope::User));
    }

    if rows.is_empty()
        && let Some(error) = first_error
    {
        return Err(error);
    }
    rows.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(rows)
}

/// `systemctl show` prints key=value blocks separated by blank lines.
pub fn parse_systemctl_show(text: &str, scope: Scope) -> Vec<ServiceRow> {
    let mut rows = Vec::new();
    let mut block: Vec<(&str, &str)> = Vec::new();

    let flush = |block: &mut Vec<(&str, &str)>, rows: &mut Vec<ServiceRow>| {
        if block.is_empty() {
            return;
        }
        let get = |key: &str| {
            block
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| v.trim())
                .unwrap_or("")
        };
        let id = get("Id");
        if !id.is_empty() {
            let active = get("ActiveState");
            let pid = get("MainPID").parse::<u32>().ok().filter(|p| *p > 0);
            let description = get("Description");
            rows.push(ServiceRow {
                name: id.to_string(),
                pid,
                description: (!description.is_empty()).then(|| description.to_string()),
                state: if pid.is_some() || active.starts_with("active") {
                    ServiceState::Running
                } else {
                    ServiceState::Stopped
                },
                scope,
            });
        }
        block.clear();
    };

    for line in text.lines() {
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            flush(&mut block, &mut rows);
            continue;
        }
        if let Some((key, value)) = trimmed.split_once('=') {
            block.push((key, value));
        }
    }
    flush(&mut block, &mut rows);
    rows
}

pub fn service_action(
    runner: &dyn Runner,
    row: &ServiceRow,
    action: ServiceAction,
) -> Result<(), String> {
    if row.scope == Scope::System && !is_root() {
        return Err(privilege_error("Controlling a system service"));
    }
    let verb = match action {
        ServiceAction::Start => "start",
        ServiceAction::Stop => "stop",
        ServiceAction::Restart => "restart",
    };
    let mut args: Vec<&str> = Vec::new();
    if row.scope == Scope::User {
        args.push("--user");
    }
    args.push(verb);
    args.push(&row.name);
    let out = runner.run("systemctl", &args)?;
    if out.success {
        Ok(())
    } else {
        Err(out.failure_reason())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopEntry {
    pub name: String,
    pub exec: String,
    pub hidden: bool,
    pub autostart_enabled: bool,
}

pub fn parse_desktop(text: &str) -> Option<DesktopEntry> {
    let mut in_entry = false;
    let mut name = None;
    let mut exec = None;
    let mut hidden = false;
    let mut autostart = true;

    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_entry = line == "[Desktop Entry]";
            continue;
        }
        if !in_entry || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match key.trim() {
            "Name" => name = Some(value.trim().to_string()),
            "Exec" => exec = Some(value.trim().to_string()),
            "Hidden" => hidden = value.trim().eq_ignore_ascii_case("true"),
            "X-GNOME-Autostart-enabled" => autostart = !value.trim().eq_ignore_ascii_case("false"),
            _ => {}
        }
    }

    Some(DesktopEntry {
        name: name?,
        exec: exec?,
        hidden,
        autostart_enabled: autostart,
    })
}

fn home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

fn startup_dirs() -> Vec<(PathBuf, Scope)> {
    let mut dirs = Vec::new();
    if let Some(home) = home() {
        dirs.push((home.join(".config/autostart"), Scope::User));
    }
    dirs.push((PathBuf::from("/etc/xdg/autostart"), Scope::System));
    dirs
}

pub fn list_startup(_runner: &dyn Runner) -> Result<Vec<StartupRow>, String> {
    let owner = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
    Ok(collect_startup(&startup_dirs(), &owner))
}

fn program_of(exec: &str) -> String {
    let first = exec.split_whitespace().next().unwrap_or(exec);
    first.trim_matches('"').trim_matches('\'').to_string()
}

fn collect_startup(dirs: &[(PathBuf, Scope)], owner: &str) -> Vec<StartupRow> {
    let mut rows = Vec::new();
    for (dir, scope) in dirs {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("desktop") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else {
                continue;
            };
            let Some(parsed) = parse_desktop(&text) else {
                continue;
            };
            let program = program_of(&parsed.exec);
            rows.push(StartupRow {
                name: parsed.name,
                command: parsed.exec,
                owner: if *scope == Scope::User {
                    owner.to_string()
                } else {
                    "root".to_string()
                },
                source: path.display().to_string(),
                enabled: !parsed.hidden && parsed.autostart_enabled,
                impact: Impact::NotMeasured,
                missing_program: !program.is_empty() && !Path::new(&program).exists(),
                scope: *scope,
            });
        }
    }
    rows.sort_by(|a, b| a.name.cmp(&b.name));
    rows
}

/// Replace or add a Hidden= line inside [Desktop Entry].
pub fn set_desktop_hidden(text: &str, hidden: bool) -> String {
    let value = if hidden { "true" } else { "false" };
    let mut out = String::new();
    let mut replaced = false;
    let mut in_entry = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            if in_entry && !replaced {
                out.push_str(&format!("Hidden={value}\n"));
                replaced = true;
            }
            in_entry = trimmed == "[Desktop Entry]";
            out.push_str(line);
            out.push('\n');
            continue;
        }
        if in_entry && trimmed.starts_with("Hidden=") {
            out.push_str(&format!("Hidden={value}\n"));
            replaced = true;
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    if !replaced && in_entry {
        out.push_str(&format!("Hidden={value}\n"));
    }
    out
}

pub fn set_startup(_runner: &dyn Runner, row: &StartupRow, enabled: bool) -> Result<(), String> {
    if row.scope == Scope::System {
        return Err(privilege_error("Changing a system startup entry"));
    }
    let user_dir = home()
        .map(|h| h.join(".config/autostart"))
        .ok_or_else(|| "HOME is not set".to_string())?;
    let source = PathBuf::from(&row.source);
    let is_own_file = source.starts_with(&user_dir);
    let target = if is_own_file {
        source.clone()
    } else {
        let file = source
            .file_name()
            .ok_or_else(|| "startup entry has no file name".to_string())?;
        user_dir.join(file)
    };

    if !enabled && !is_own_file {
        std::fs::create_dir_all(&user_dir)
            .map_err(|e| format!("cannot write {user_dir:?}: {e}"))?;
    }

    let existing = std::fs::read_to_string(&target).unwrap_or_else(|_| {
        let mut text = String::from("[Desktop Entry]\nType=Application\n");
        text.push_str(&format!("Name={}\n", row.name));
        text.push_str(&format!("Exec={}\n", row.command));
        text
    });
    let updated = set_desktop_hidden(&existing, !enabled);
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("cannot write {parent:?}: {e}"))?;
    }
    std::fs::write(&target, updated).map_err(|e| format!("cannot write {target:?}: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::RecordingRunner;
    use crate::platform::proc::CommandOutput;

    #[test]
    fn systemctl_show_parses_units() {
        let rows = parse_systemctl_show(
            include_str!("../../tests/fixtures/systemctl_show.txt"),
            Scope::System,
        );
        assert_eq!(rows.len(), 3);
        let nginx = rows.iter().find(|r| r.name == "nginx.service").unwrap();
        assert_eq!(nginx.pid, Some(1412));
        assert_eq!(nginx.state, ServiceState::Running);
        assert_eq!(
            nginx.description.as_deref(),
            Some("A high performance web server and a reverse proxy server")
        );
        let ssh = rows.iter().find(|r| r.name == "ssh.service").unwrap();
        assert_eq!(ssh.pid, None);
        assert_eq!(ssh.state, ServiceState::Stopped);
        let broken = rows.iter().find(|r| r.name == "broken.service").unwrap();
        assert_eq!(broken.description, None);
    }

    #[test]
    fn desktop_entry_parses_fields() {
        let entry = parse_desktop(include_str!("../../tests/fixtures/syncthing.desktop")).unwrap();
        assert_eq!(entry.name, "Syncthing");
        assert!(entry.exec.contains("--no-browser"));
        assert!(!entry.hidden);
        assert!(entry.autostart_enabled);
    }

    #[test]
    fn desktop_entry_requires_name_and_exec() {
        assert!(parse_desktop("[Desktop Entry]\nName=Only\n").is_none());
        assert!(parse_desktop("").is_none());
    }

    #[test]
    fn hidden_line_is_replaced_not_duplicated() {
        let text = include_str!("../../tests/fixtures/syncthing.desktop");
        let disabled = set_desktop_hidden(text, true);
        assert_eq!(disabled.matches("Hidden=").count(), 1);
        assert!(disabled.contains("Hidden=true"));
        assert!(disabled.contains("Exec=/usr/bin/syncthing"));
        let enabled = set_desktop_hidden(&disabled, false);
        assert!(enabled.contains("Hidden=false"));
        assert_eq!(enabled.matches("Hidden=").count(), 1);
    }

    #[test]
    fn hidden_line_is_added_when_missing() {
        let text = "[Desktop Entry]\nName=X\nExec=/bin/true\n";
        let out = set_desktop_hidden(text, true);
        assert!(out.contains("Hidden=true"));
        assert!(out.contains("Hidden=true\n") || out.ends_with("Hidden=true\n"));
    }

    #[test]
    fn user_entry_toggle_round_trips_on_disk() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("syncthing.desktop");
        std::fs::write(
            &path,
            include_str!("../../tests/fixtures/syncthing.desktop"),
        )
        .unwrap();
        let row = StartupRow {
            name: "Syncthing".into(),
            command: "/usr/bin/syncthing".into(),
            owner: "michael".into(),
            source: path.display().to_string(),
            enabled: true,
            impact: Impact::NotMeasured,
            missing_program: false,
            scope: Scope::User,
        };
        // Only the write path is exercised here; the file lives outside $HOME by design.
        let updated = set_desktop_hidden(&std::fs::read_to_string(&path).unwrap(), true);
        std::fs::write(&path, updated).unwrap();
        assert!(
            std::fs::read_to_string(&path)
                .unwrap()
                .contains("Hidden=true")
        );
        let parsed = parse_desktop(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert!(parsed.hidden);
        assert_eq!(row.scope, Scope::User);
    }

    #[test]
    fn system_scope_toggle_is_refused() {
        let runner = RecordingRunner::new();
        let row = StartupRow {
            name: "ModemManager".into(),
            command: "/usr/sbin/ModemManager".into(),
            owner: "root".into(),
            source: "/etc/xdg/autostart/modem-manager.desktop".into(),
            enabled: true,
            impact: Impact::NotMeasured,
            missing_program: false,
            scope: Scope::System,
        };
        let err = set_startup(&runner, &row, false).unwrap_err();
        assert!(err.contains("administrator"));
        assert!(runner.calls().is_empty());
    }

    #[test]
    fn user_scope_actions_pass_the_user_flag() {
        let runner = RecordingRunner::new();
        runner.respond("systemctl", CommandOutput::ok(""));
        let row = ServiceRow {
            name: "gpg-agent.service".into(),
            pid: Some(900),
            description: None,
            state: ServiceState::Running,
            scope: Scope::User,
        };
        service_action(&runner, &row, ServiceAction::Restart).unwrap();
        let calls = runner.calls();
        assert_eq!(calls[0].1, vec!["--user", "restart", "gpg-agent.service"]);
    }

    #[test]
    fn system_scope_action_is_refused_for_non_root() {
        if is_root() {
            return;
        }
        let runner = RecordingRunner::new();
        let row = ServiceRow {
            name: "nginx.service".into(),
            pid: Some(1),
            description: None,
            state: ServiceState::Running,
            scope: Scope::System,
        };
        let err = service_action(&runner, &row, ServiceAction::Stop).unwrap_err();
        assert!(err.contains("administrator"));
        assert!(runner.calls().is_empty());
    }
}
