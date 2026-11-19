//! One place that runs external commands. Never a shell, always an argument vector.

use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl CommandOutput {
    pub fn ok(stdout: impl Into<String>) -> Self {
        Self {
            success: true,
            stdout: stdout.into(),
            stderr: String::new(),
        }
    }

    /// The manager's own message, falling back to its exit status.
    pub fn failure_reason(&self) -> String {
        let stderr = self.stderr.trim();
        if !stderr.is_empty() {
            stderr.to_string()
        } else if !self.stdout.trim().is_empty() {
            self.stdout.trim().to_string()
        } else {
            "the command failed without a message".to_string()
        }
    }
}

pub trait Runner {
    fn run(&self, program: &str, args: &[&str]) -> Result<CommandOutput, String>;
}

/// Runs the real thing.
#[derive(Debug, Default, Clone, Copy)]
pub struct RealRunner;

impl Runner for RealRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<CommandOutput, String> {
        let output = Command::new(program)
            .args(args)
            .output()
            .map_err(|err| match err.kind() {
                std::io::ErrorKind::NotFound => format!("{program} is not installed"),
                _ => format!("could not run {program}: {err}"),
            })?;
        Ok(CommandOutput {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}

/// A runner for tests: records calls and returns canned output by program name.
#[derive(Debug, Default)]
pub struct RecordingRunner {
    pub calls: std::sync::Mutex<Vec<(String, Vec<String>)>>,
    pub responses: std::sync::Mutex<Vec<(String, Result<CommandOutput, String>)>>,
}

impl RecordingRunner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn respond(&self, program: &str, output: CommandOutput) {
        self.responses
            .lock()
            .unwrap()
            .push((program.to_string(), Ok(output)));
    }

    pub fn fail(&self, program: &str, error: &str) {
        self.responses
            .lock()
            .unwrap()
            .push((program.to_string(), Err(error.to_string())));
    }

    pub fn calls(&self) -> Vec<(String, Vec<String>)> {
        self.calls.lock().unwrap().clone()
    }
}

impl Runner for RecordingRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<CommandOutput, String> {
        self.calls.lock().unwrap().push((
            program.to_string(),
            args.iter().map(|a| a.to_string()).collect(),
        ));
        let responses = self.responses.lock().unwrap();
        if let Some((_, response)) = responses.iter().find(|(p, _)| p == program) {
            return response.clone();
        }
        Err(format!("{program} is not installed"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hostile_arguments_stay_single_arguments() {
        let runner = RealRunner;
        let out = runner
            .run("/usr/bin/printf", &["[%s]", "a b; rm -rf / \"quoted\""])
            .expect("printf runs");
        assert!(out.success);
        assert_eq!(out.stdout, "[a b; rm -rf / \"quoted\"]");
    }

    #[test]
    fn missing_program_is_an_error_not_a_panic() {
        let runner = RealRunner;
        let err = runner
            .run("definitely-not-a-real-binary-xyz", &[])
            .unwrap_err();
        assert!(err.contains("not installed"), "{err}");
    }

    #[test]
    fn recording_runner_records_argv() {
        let runner = RecordingRunner::new();
        runner.respond("launchctl", CommandOutput::ok("ok"));
        let out = runner
            .run("launchctl", &["disable", "gui/501/a b;c"])
            .unwrap();
        assert!(out.success);
        assert_eq!(
            runner.calls(),
            vec![(
                "launchctl".to_string(),
                vec!["disable".to_string(), "gui/501/a b;c".to_string()]
            )]
        );
    }

    #[test]
    fn failure_reason_prefers_stderr() {
        let out = CommandOutput {
            success: false,
            stdout: "noise".into(),
            stderr: "Job not found".into(),
        };
        assert_eq!(out.failure_reason(), "Job not found");
        let bare = CommandOutput {
            success: false,
            stdout: String::new(),
            stderr: String::new(),
        };
        assert!(bare.failure_reason().contains("without a message"));
    }
}
