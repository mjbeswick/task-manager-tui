//! Platform backends. One interface, two implementations.

pub mod proc;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
use linux as imp;
#[cfg(target_os = "macos")]
use macos as imp;

use crate::model::{Scope, ServiceRow, SessionRow, SessionStatus, Signal, StartupRow};
use std::collections::HashMap;
use std::process::Command;
#[cfg(target_os = "linux")]
use std::process::Stdio;

pub use proc::{CommandOutput, RealRunner, RecordingRunner, Runner};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceAction {
    Start,
    Stop,
    Restart,
}

impl ServiceAction {
    pub fn label(self) -> &'static str {
        match self {
            ServiceAction::Start => "Start",
            ServiceAction::Stop => "Stop",
            ServiceAction::Restart => "Restart",
        }
    }

    pub fn past_tense(self) -> &'static str {
        match self {
            ServiceAction::Start => "started",
            ServiceAction::Stop => "stopped",
            ServiceAction::Restart => "restarted",
        }
    }
}

/// A privileged operation that runs as one elevated command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElevatedAction {
    Service {
        name: String,
        scope: Scope,
        action: ServiceAction,
    },
    Signal {
        pids: Vec<u32>,
        signal: Signal,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElevationError {
    PasswordRequired,
    Failed(String),
}

impl ElevatedAction {
    /// Program and argv for the action, before any elevation wrapper.
    pub fn command(&self) -> (String, Vec<String>) {
        match self {
            ElevatedAction::Signal { pids, signal } => {
                let mut args = vec!["-s".to_string(), signal.name().to_string()];
                args.extend(pids.iter().map(|pid| pid.to_string()));
                ("kill".to_string(), args)
            }
            ElevatedAction::Service {
                name,
                scope,
                action,
            } => service_command(name, *scope, *action),
        }
    }

    pub fn describe(&self) -> String {
        match self {
            ElevatedAction::Signal { pids, signal } => format!(
                "Send SIG{} to {} process{}",
                signal.name(),
                pids.len(),
                if pids.len() == 1 { "" } else { "es" }
            ),
            ElevatedAction::Service { name, action, .. } => {
                format!("{} {name}", action.label())
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn service_command(name: &str, scope: Scope, action: ServiceAction) -> (String, Vec<String>) {
    let verb = match action {
        ServiceAction::Start => "start",
        ServiceAction::Stop => "stop",
        ServiceAction::Restart => "restart",
    };
    let mut args = Vec::new();
    if scope == Scope::User {
        args.push("--user".to_string());
    }
    args.push(verb.to_string());
    args.push(name.to_string());
    ("systemctl".to_string(), args)
}

#[cfg(target_os = "macos")]
fn service_command(name: &str, scope: Scope, action: ServiceAction) -> (String, Vec<String>) {
    let domain = if scope == Scope::System {
        "system".to_string()
    } else {
        format!("gui/{}", current_uid())
    };
    let target = format!("{domain}/{name}");
    let args = match action {
        ServiceAction::Start => vec!["kickstart".to_string(), target],
        ServiceAction::Stop => vec!["kill".to_string(), "SIGTERM".to_string(), target],
        ServiceAction::Restart => {
            vec!["kickstart".to_string(), "-k".to_string(), target]
        }
    };
    ("launchctl".to_string(), args)
}

/// Run one action with administrator rights. `password` is only used by the
/// Linux fallback and is never stored.
pub fn elevate(action: &ElevatedAction, password: Option<&str>) -> Result<(), ElevationError> {
    let (program, args) = action.command();
    #[cfg(target_os = "macos")]
    {
        let _ = password;
        run_macos_elevated(&program, &args)
    }
    #[cfg(target_os = "linux")]
    {
        run_linux_elevated(&program, &args, password)
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        let _ = (program, args, password);
        Err(ElevationError::Failed(
            "elevation is not supported on this platform".into(),
        ))
    }
}

#[cfg(target_os = "macos")]
fn run_macos_elevated(program: &str, args: &[String]) -> Result<(), ElevationError> {
    let command = std::iter::once(program.to_string())
        .chain(args.iter().cloned())
        .map(|part| format!("'{}'", part.replace('\'', "'\\''")))
        .collect::<Vec<_>>()
        .join(" ");
    let escaped = command.replace('\\', "\\\\").replace('"', "\\\"");
    let script = format!("do shell script \"{escaped}\" with administrator privileges");
    let output = Command::new("osascript")
        .args(["-e", &script])
        .output()
        .map_err(|err| ElevationError::Failed(format!("cannot start osascript: {err}")))?;
    finish(output)
}

#[cfg(target_os = "linux")]
fn run_linux_elevated(
    program: &str,
    args: &[String],
    password: Option<&str>,
) -> Result<(), ElevationError> {
    if let Some(password) = password {
        return run_with_password(program, args, password);
    }
    if pkexec_available() {
        let output = Command::new("pkexec")
            .arg(program)
            .args(args)
            .output()
            .map_err(|err| ElevationError::Failed(format!("cannot start pkexec: {err}")))?;
        if output.status.success() {
            return Ok(());
        }
        let message = String::from_utf8_lossy(&output.stderr);
        if message.contains("No authentication agent") {
            return Err(ElevationError::PasswordRequired);
        }
        return Err(ElevationError::Failed(clean_error(&message, output.status)));
    }
    let output = Command::new("sudo")
        .arg("-n")
        .arg("--")
        .arg(program)
        .args(args)
        .output()
        .map_err(|err| ElevationError::Failed(format!("cannot start sudo: {err}")))?;
    if output.status.success() {
        return Ok(());
    }
    let message = String::from_utf8_lossy(&output.stderr);
    if message.contains("password is required")
        || message.contains("no tty")
        || message.contains("terminal is required")
        || message.contains("askpass")
    {
        Err(ElevationError::PasswordRequired)
    } else {
        Err(ElevationError::Failed(clean_error(&message, output.status)))
    }
}

#[cfg(target_os = "linux")]
fn pkexec_available() -> bool {
    std::path::Path::new("/usr/bin/pkexec").exists() || std::path::Path::new("/bin/pkexec").exists()
}

#[cfg(target_os = "linux")]
fn run_with_password(program: &str, args: &[String], password: &str) -> Result<(), ElevationError> {
    use std::io::Write;
    let mut child = Command::new("sudo")
        .arg("-S")
        .arg("-p")
        .arg("")
        .arg("--")
        .arg(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| ElevationError::Failed(format!("cannot start sudo: {err}")))?;
    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(format!("{password}\n").as_bytes())
            .map_err(|err| ElevationError::Failed(format!("cannot write to sudo: {err}")))?;
    }
    let output = child
        .wait_with_output()
        .map_err(|err| ElevationError::Failed(format!("cannot wait for sudo: {err}")))?;
    finish(output)
}

fn finish(output: std::process::Output) -> Result<(), ElevationError> {
    if output.status.success() {
        Ok(())
    } else {
        let message = String::from_utf8_lossy(&output.stderr);
        Err(ElevationError::Failed(clean_error(&message, output.status)))
    }
}

fn clean_error(message: &str, status: std::process::ExitStatus) -> String {
    let text = message.trim();
    if text.is_empty() {
        format!("elevated command exited with {status}")
    } else {
        text.to_string()
    }
}

pub fn list_services(runner: &dyn Runner) -> Result<Vec<ServiceRow>, String> {
    imp::list_services(runner)
}

pub fn service_action(
    runner: &dyn Runner,
    row: &ServiceRow,
    action: ServiceAction,
) -> Result<(), String> {
    imp::service_action(runner, row, action)
}

pub fn list_startup(runner: &dyn Runner) -> Result<Vec<StartupRow>, String> {
    imp::list_startup(runner)
}

pub fn set_startup(runner: &dyn Runner, row: &StartupRow, enabled: bool) -> Result<(), String> {
    imp::set_startup(runner, row, enabled)
}

/// Send one POSIX signal. Returns the operating system's error unchanged so the
/// caller can explain a refusal.
pub fn signal(pid: u32, signal: Signal) -> std::io::Result<()> {
    // SAFETY: kill takes an integer pid and signal and cannot violate memory safety.
    let result = unsafe { libc::kill(pid as libc::pid_t, signal.number()) };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

/// Change a process's scheduling priority. Negative values may need root.
pub fn set_priority(pid: u32, nice: i32) -> std::io::Result<()> {
    // SAFETY: setpriority takes integers and cannot violate memory safety.
    let result = unsafe { libc::setpriority(libc::PRIO_PROCESS, pid as libc::id_t, nice) };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

pub fn current_uid() -> u32 {
    // SAFETY: getuid has no preconditions and cannot fail.
    unsafe { libc::getuid() }
}

pub fn is_root() -> bool {
    current_uid() == 0
}

pub fn privilege_error(what: &str) -> String {
    format!(
        "{what} needs administrator rights. Re-run taskmgr with sudo, or use the system's own service manager."
    )
}

/// Map user names to numeric ids, used to attribute processes to sessions.
pub fn user_ids() -> HashMap<String, u32> {
    let mut map = HashMap::new();
    if let Ok(users) = sysinfo::Users::new_with_refreshed_list() {
        for user in &users {
            map.insert(user.name().to_string(), **user.id());
        }
    }
    map
}

/// Our controlling terminal, for marking the current session.
pub fn current_tty(runner: &dyn Runner) -> Option<String> {
    let pid = std::process::id().to_string();
    let out = runner.run("ps", &["-o", "tty=", "-p", &pid]).ok()?;
    let tty = out.stdout.trim().trim_start_matches("/dev/").to_string();
    if tty.is_empty() || tty == "?" || tty == "??" {
        None
    } else {
        Some(tty)
    }
}

pub fn list_sessions(runner: &dyn Runner) -> Result<Vec<SessionRow>, String> {
    let out = runner.run("who", &[])?;
    if !out.success {
        return Err(out.failure_reason());
    }
    let ids = user_ids();
    let tty = current_tty(runner);
    Ok(parse_who(&out.stdout, &ids, tty.as_deref()))
}

/// Parse `who` output. The time column has spaces, so only the first two tokens
/// and an optional parenthesised host at the end are meaningful.
pub fn parse_who(
    text: &str,
    ids: &HashMap<String, u32>,
    current_tty: Option<&str>,
) -> Vec<SessionRow> {
    let mut rows = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        let Some(user) = parts.next() else { continue };
        let Some(tty) = parts.next() else { continue };
        if tty.starts_with('-') {
            continue;
        }
        let host = line
            .rsplit_once('(')
            .and_then(|(_, rest)| rest.strip_suffix(')'))
            .unwrap_or("")
            .to_string();
        let tty_name = tty.trim_start_matches("/dev/").to_string();
        let status = if tty_name == "console"
            || tty_name.starts_with("tty")
            || tty_name.starts_with("pts")
        {
            SessionStatus::Active
        } else {
            SessionStatus::Disconnected
        };
        rows.push(SessionRow {
            user: user.to_string(),
            tty: tty_name.clone(),
            host,
            status,
            uid: ids.get(user).copied().unwrap_or(0),
            cpu: 0.0,
            memory: 0,
            disk: Default::default(),
            current: current_tty.is_some_and(|current| current == tty_name),
            others: 0,
        });
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids() -> HashMap<String, u32> {
        HashMap::from([("michael".to_string(), 501), ("guest".to_string(), 502)])
    }

    #[test]
    fn who_parses_sessions_with_hosts() {
        let rows = parse_who(
            include_str!("../../tests/fixtures/who.txt"),
            &ids(),
            Some("ttys000"),
        );
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].user, "michael");
        assert_eq!(rows[0].tty, "ttys000");
        assert!(rows[0].current);
        assert_eq!(rows[0].status, SessionStatus::Active);
        assert_eq!(rows[1].host, "michael-mba");
        assert_eq!(rows[2].host, "192.168.1.4");
        assert_eq!(rows[2].uid, 502);
        assert!(!rows[2].current);
    }

    #[test]
    fn who_handles_empty_output() {
        assert!(parse_who("", &ids(), None).is_empty());
    }

    #[test]
    fn signal_ends_a_child_process() {
        let child = std::process::Command::new("sleep")
            .arg("30")
            .spawn()
            .expect("spawn sleep");
        let pid = child.id();
        signal(pid, Signal::Term).expect("signal delivered");
        let mut child = child;
        let mut waited = None;
        for _ in 0..100 {
            if let Some(status) = child.try_wait().unwrap() {
                waited = Some(status);
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        assert!(waited.is_some(), "the child should have exited");
    }

    #[test]
    fn elevated_signal_builds_a_kill_argv() {
        let action = ElevatedAction::Signal {
            pids: vec![12, 34],
            signal: Signal::Term,
        };
        assert_eq!(
            action.command(),
            (
                "kill".to_string(),
                vec![
                    "-s".to_string(),
                    "TERM".to_string(),
                    "12".to_string(),
                    "34".to_string()
                ]
            )
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn elevated_system_service_uses_systemctl() {
        let action = ElevatedAction::Service {
            name: "nginx.service".into(),
            scope: Scope::System,
            action: ServiceAction::Stop,
        };
        assert_eq!(
            action.command(),
            (
                "systemctl".to_string(),
                vec!["stop".to_string(), "nginx.service".to_string()]
            )
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn elevated_system_service_uses_the_system_domain() {
        let action = ElevatedAction::Service {
            name: "com.example.job".into(),
            scope: Scope::System,
            action: ServiceAction::Restart,
        };
        assert_eq!(
            action.command(),
            (
                "launchctl".to_string(),
                vec![
                    "kickstart".to_string(),
                    "-k".to_string(),
                    "system/com.example.job".to_string()
                ]
            )
        );
    }

    #[test]
    fn signal_reports_a_missing_process() {
        let error = signal(4_000_000, Signal::Term).unwrap_err();
        assert_eq!(error.raw_os_error(), Some(libc::ESRCH));
    }
}
