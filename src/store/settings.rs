//! User settings: theme, update speed, and last tab. Plain TOML.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub theme: String,
    pub update_speed: String,
    pub last_tab: usize,
    pub ascii: bool,
    pub chart_style: String,
    pub chart_color: String,
    pub chart_gradient: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            update_speed: "normal".to_string(),
            last_tab: 0,
            ascii: false,
            chart_style: "filled".to_string(),
            chart_color: "per-resource".to_string(),
            chart_gradient: true,
        }
    }
}

impl Settings {
    pub fn interval(&self) -> Option<Duration> {
        match self.update_speed.as_str() {
            "high" => Some(Duration::from_millis(500)),
            "low" => Some(Duration::from_secs(2)),
            "paused" => None,
            _ => Some(Duration::from_secs(1)),
        }
    }

    pub fn set_speed(&mut self, speed: &str) {
        self.update_speed = speed.to_string();
    }

    pub fn faster_speed(&self) -> &'static str {
        match self.update_speed.as_str() {
            "paused" => "low",
            "low" => "normal",
            _ => "high",
        }
    }

    pub fn next_speed(&self) -> &'static str {
        match self.update_speed.as_str() {
            "high" => "normal",
            "normal" => "low",
            "low" => "paused",
            _ => "high",
        }
    }
}

pub fn project_dirs() -> Option<directories::ProjectDirs> {
    directories::ProjectDirs::from("", "", "task-manager-tui")
}

pub fn config_path() -> Option<PathBuf> {
    project_dirs().map(|dirs| dirs.config_dir().join("settings.toml"))
}

/// Load settings, falling back to defaults with a message when the file is unusable.
pub fn load_from(path: &Path) -> (Settings, Option<String>) {
    match std::fs::read_to_string(path) {
        Ok(text) => match toml::from_str::<Settings>(&text) {
            Ok(settings) => (settings, None),
            Err(err) => (
                Settings::default(),
                Some(format!("settings.toml is invalid ({err}); using defaults")),
            ),
        },
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => (Settings::default(), None),
        Err(err) => (
            Settings::default(),
            Some(format!("cannot read settings.toml ({err}); using defaults")),
        ),
    }
}

pub fn load() -> (Settings, Option<String>) {
    match config_path() {
        Some(path) => load_from(&path),
        None => (Settings::default(), None),
    }
}

pub fn save_to(settings: &Settings, path: &Path) -> Result<(), String> {
    let text =
        toml::to_string_pretty(settings).map_err(|e| format!("cannot encode settings: {e}"))?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("cannot create {parent:?}: {e}"))?;
    }
    std::fs::write(path, text).map_err(|e| format!("cannot write {path:?}: {e}"))
}

pub fn save(settings: &Settings) -> Result<(), String> {
    match config_path() {
        Some(path) => save_to(settings, &path),
        None => Err("no configuration directory is available".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_gives_defaults_without_complaint() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.toml");
        let (settings, warning) = load_from(&path);
        assert_eq!(settings, Settings::default());
        assert!(warning.is_none());
    }

    #[test]
    fn round_trip_keeps_values() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested/settings.toml");
        let settings = Settings {
            theme: "dark".into(),
            update_speed: "high".into(),
            last_tab: 3,
            ascii: true,
            chart_style: "line".into(),
            chart_color: "orange".into(),
            chart_gradient: false,
        };
        save_to(&settings, &path).unwrap();
        let (loaded, warning) = load_from(&path);
        assert!(warning.is_none());
        assert_eq!(loaded, settings);
    }

    #[test]
    fn invalid_toml_falls_back_and_warns() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.toml");
        std::fs::write(&path, "theme = [not valid").unwrap();
        let (settings, warning) = load_from(&path);
        assert_eq!(settings, Settings::default());
        assert!(warning.unwrap().contains("invalid"));
    }

    #[test]
    fn chart_settings_default_and_survive_old_files() {
        assert_eq!(Settings::default().chart_style, "filled");
        assert_eq!(Settings::default().chart_color, "per-resource");
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.toml");
        // A file written before chart settings existed.
        std::fs::write(&path, "theme = \"dark\"\nupdate_speed = \"low\"\n").unwrap();
        let (settings, warning) = load_from(&path);
        assert!(warning.is_none());
        assert_eq!(settings.chart_style, "filled");
        assert_eq!(settings.chart_color, "per-resource");
        // A file that already chose a colour keeps it.
        std::fs::write(&path, "chart_color = \"teal\"\n").unwrap();
        assert_eq!(load_from(&path).0.chart_color, "teal");
    }

    #[test]
    fn speed_cycles_and_maps_to_intervals() {
        let mut settings = Settings::default();
        assert_eq!(settings.interval(), Some(Duration::from_secs(1)));
        assert_eq!(settings.next_speed(), "low");
        settings.set_speed("low");
        assert_eq!(settings.interval(), Some(Duration::from_secs(2)));
        settings.set_speed("paused");
        assert_eq!(settings.interval(), None);
        settings.set_speed("high");
        assert_eq!(settings.interval(), Some(Duration::from_millis(500)));
    }
}
