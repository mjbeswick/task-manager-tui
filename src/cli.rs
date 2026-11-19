//! Command line flags.

use crate::app::Tab;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "taskmgr",
    version,
    about = "A Windows 10 Task Manager clone for Linux and macOS terminals"
)]
pub struct Cli {
    /// Sampling interval: high (0.5s), normal (1s), low (2s), or paused.
    #[arg(long, value_parser = parse_interval)]
    pub interval: Option<String>,

    /// Colour theme: light or dark.
    #[arg(long, value_parser = parse_theme)]
    pub theme: Option<String>,

    /// Use ASCII glyphs instead of box drawing and block characters.
    #[arg(long)]
    pub ascii: bool,

    /// Open this tab: processes, performance, app-history, startup, users, details, services.
    #[arg(long, value_parser = parse_tab)]
    pub tab: Option<Tab>,

    /// Render one frame, print it, and exit. Used by smoke tests.
    #[arg(long)]
    pub once: bool,
}

pub fn parse_tab(value: &str) -> Result<Tab, String> {
    let normalized = value.to_ascii_lowercase().replace('_', "-");
    match normalized.as_str() {
        "processes" => Ok(Tab::Processes),
        "performance" => Ok(Tab::Performance),
        "app-history" | "apphistory" => Ok(Tab::AppHistory),
        "startup" => Ok(Tab::Startup),
        "users" => Ok(Tab::Users),
        "details" => Ok(Tab::Details),
        "services" => Ok(Tab::Services),
        other => Err(format!(
            "unknown tab '{other}'; try one of processes, performance, app-history, startup, users, details, services"
        )),
    }
}

fn parse_interval(value: &str) -> Result<String, String> {
    match value.to_ascii_lowercase().as_str() {
        "high" | "normal" | "low" | "paused" => Ok(value.to_ascii_lowercase()),
        other => Err(format!(
            "unknown interval '{other}'; try high, normal, low, or paused"
        )),
    }
}

fn parse_theme(value: &str) -> Result<String, String> {
    match value.to_ascii_lowercase().as_str() {
        "light" | "dark" => Ok(value.to_ascii_lowercase()),
        other => Err(format!("unknown theme '{other}'; try light or dark")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tab_names_parse() {
        assert_eq!(parse_tab("processes").unwrap(), Tab::Processes);
        assert_eq!(parse_tab("app-history").unwrap(), Tab::AppHistory);
        assert_eq!(parse_tab("app_history").unwrap(), Tab::AppHistory);
        assert!(parse_tab("nope").unwrap_err().contains("unknown tab"));
    }

    #[test]
    fn flags_parse() {
        let cli = Cli::try_parse_from([
            "taskmgr",
            "--interval",
            "low",
            "--theme",
            "dark",
            "--ascii",
            "--tab",
            "services",
            "--once",
        ])
        .unwrap();
        assert_eq!(cli.interval.as_deref(), Some("low"));
        assert_eq!(cli.theme.as_deref(), Some("dark"));
        assert!(cli.ascii);
        assert_eq!(cli.tab, Some(Tab::Services));
        assert!(cli.once);
    }

    #[test]
    fn bad_values_are_rejected() {
        assert!(Cli::try_parse_from(["taskmgr", "--tab", "nope"]).is_err());
        assert!(Cli::try_parse_from(["taskmgr", "--interval", "fast"]).is_err());
        assert!(Cli::try_parse_from(["taskmgr", "--theme", "purple"]).is_err());
    }
}
