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

use crate::model::{ServiceRow, SessionRow, SessionStatus, StartupRow};
use std::collections::HashMap;

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

/// Send SIGTERM, or SIGKILL when `force` is set. Returns the operating system's
/// error unchanged so the caller can explain a refusal.
pub fn signal(pid: u32, force: bool) -> std::io::Result<()> {
    let signal = if force { libc::SIGKILL } else { libc::SIGTERM };
    // SAFETY: kill takes an integer pid and signal and cannot violate memory safety.
    let result = unsafe { libc::kill(pid as libc::pid_t, signal) };
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
        signal(pid, false).expect("signal delivered");
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
    fn signal_reports_a_missing_process() {
        let error = signal(4_000_000, false).unwrap_err();
        assert_eq!(error.raw_os_error(), Some(libc::ESRCH));
    }
}
