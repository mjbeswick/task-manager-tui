//! App history: CPU time, peak memory, and disk totals per application name,
//! persisted as versioned JSON with atomic replacement.

use crate::model::{AppUsage, Snapshot};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

pub const VERSION: u32 = 1;
const RETENTION_SECS: u64 = 30 * 86_400;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppRecord {
    pub cpu_ms: u64,
    pub peak_memory: u64,
    pub disk_read: u64,
    pub disk_written: u64,
    pub last_seen: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct HistoryFile {
    version: u32,
    apps: HashMap<String, AppRecord>,
}

#[derive(Debug, Default)]
pub struct History {
    pub path: Option<PathBuf>,
    pub apps: HashMap<String, AppRecord>,
    pub warning: Option<String>,
    dirty: bool,
    saved: bool,
}

impl History {
    pub fn data_path() -> Option<PathBuf> {
        crate::store::settings::project_dirs().map(|dirs| dirs.data_dir().join("history.json"))
    }

    pub fn load() -> Self {
        match Self::data_path() {
            Some(path) => Self::load_from(path),
            None => Self {
                warning: Some("no data directory is available; history will not persist".into()),
                ..Self::default()
            },
        }
    }

    pub fn load_from(path: PathBuf) -> Self {
        let text = match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                return Self {
                    path: Some(path),
                    ..Self::default()
                };
            }
            Err(err) => {
                return Self {
                    path: Some(path),
                    warning: Some(format!("cannot read history ({err}); starting empty")),
                    ..Self::default()
                };
            }
        };

        match serde_json::from_str::<HistoryFile>(&text) {
            Ok(file) if file.version == VERSION => {
                let mut history = Self {
                    path: Some(path),
                    apps: file.apps,
                    ..Self::default()
                };
                history.prune(now_epoch());
                history
            }
            Ok(file) => {
                let aside = path.with_extension(format!("json.v{}", file.version));
                let _ = std::fs::rename(&path, &aside);
                Self {
                    path: Some(path),
                    warning: Some(format!(
                        "history version {} is not supported; moved to {}",
                        file.version,
                        aside.display()
                    )),
                    ..Self::default()
                }
            }
            Err(_) => {
                let aside = path.with_extension("json.corrupt");
                let _ = std::fs::rename(&path, &aside);
                Self {
                    path: Some(path),
                    warning: Some(format!(
                        "history file was unreadable; moved to {}",
                        aside.display()
                    )),
                    ..Self::default()
                }
            }
        }
    }

    fn prune(&mut self, now: u64) {
        let cutoff = now.saturating_sub(RETENTION_SECS);
        self.apps.retain(|_, record| record.last_seen >= cutoff);
    }

    /// Add one sample's worth of usage. `logical` cores converts the normalised
    /// whole-system percentage back into core-seconds of CPU time.
    pub fn record(&mut self, snapshot: &Snapshot, dt: Duration, logical: usize, now: u64) {
        let dt_ms = dt.as_millis() as f64;
        let logical = logical.max(1) as f64;
        for process in &snapshot.processes {
            let entry = self.apps.entry(process.name.clone()).or_default();
            let core_share = (process.cpu as f64 / 100.0) * logical;
            entry.cpu_ms += (core_share * dt_ms).round() as u64;
            entry.peak_memory = entry.peak_memory.max(process.memory);
            entry.disk_read += (process.disk.read_bps * dt.as_secs_f64()).round() as u64;
            entry.disk_written += (process.disk.write_bps * dt.as_secs_f64()).round() as u64;
            entry.last_seen = now;
            self.dirty = true;
        }
    }

    pub fn rows(&self) -> Vec<AppUsage> {
        let mut rows: Vec<AppUsage> = self
            .apps
            .iter()
            .map(|(name, record)| AppUsage {
                name: name.clone(),
                cpu_ms: record.cpu_ms,
                peak_memory: record.peak_memory,
                disk_read: record.disk_read,
                disk_written: record.disk_written,
                last_seen: record.last_seen,
                network: None,
            })
            .collect();
        AppUsage::sort_by_cpu(&mut rows);
        rows
    }

    pub fn clear(&mut self) {
        self.apps.clear();
        self.dirty = true;
    }

    /// Remove one application's totals, keeping every other entry.
    pub fn remove(&mut self, name: &str) {
        if self.apps.remove(name).is_some() {
            self.dirty = true;
        }
    }

    pub fn flush(&mut self) -> Result<(), String> {
        let Some(path) = self.path.clone() else {
            return Err("no data directory is available".into());
        };
        if !self.dirty && self.saved {
            return Ok(());
        }
        let file = HistoryFile {
            version: VERSION,
            apps: self.apps.clone(),
        };
        let text = serde_json::to_string_pretty(&file)
            .map_err(|e| format!("cannot encode history: {e}"))?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("cannot create {parent:?}: {e}"))?;
        }
        let temp = path.with_extension("json.tmp");
        std::fs::write(&temp, text).map_err(|e| format!("cannot write {temp:?}: {e}"))?;
        std::fs::rename(&temp, &path).map_err(|e| format!("cannot replace {path:?}: {e}"))?;
        self.dirty = false;
        self.saved = true;
        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}

pub fn now_epoch() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DiskIo, Kind, Priority, ProcKey, ProcRow, Status};

    fn snap(cpu: f32, memory: u64) -> Snapshot {
        let mut snapshot = Snapshot::default();
        snapshot.processes.push(ProcRow {
            key: ProcKey::new(42, 1),
            name: "Ghostty".into(),
            kind: Kind::App,
            status: Status::Running,
            cpu,
            memory,
            disk: DiskIo {
                read_bps: 100.0,
                write_bps: 50.0,
            },
            network: None,
            gpu: None,
            user: Some("michael".into()),
            uid: Some(501),
            parent_pid: None,
            exe: None,
            cmd: "Ghostty".into(),
            threads: Some(1),
            priority: Priority { nice: 0 },
            accumulated_cpu_ms: 0,
            run_time_secs: 10,
        });
        snapshot
    }

    #[test]
    fn cpu_time_accumulates_core_seconds() {
        let mut history = History::default();
        // 12.5% of an eight core machine is one full core.
        history.record(&snap(12.5, 100), Duration::from_secs(1), 8, 1_000);
        history.record(&snap(12.5, 100), Duration::from_secs(1), 8, 1_001);
        let record = history.apps.get("Ghostty").unwrap();
        assert_eq!(record.cpu_ms, 2_000);
        assert_eq!(record.peak_memory, 100);
        assert_eq!(record.disk_read, 200);
        assert_eq!(record.disk_written, 100);
        assert_eq!(record.last_seen, 1_001);
    }

    #[test]
    fn peak_memory_keeps_the_maximum() {
        let mut history = History::default();
        history.record(&snap(1.0, 500), Duration::from_secs(1), 8, 10);
        history.record(&snap(1.0, 100), Duration::from_secs(1), 8, 11);
        assert_eq!(history.apps.get("Ghostty").unwrap().peak_memory, 500);
    }

    #[test]
    fn save_and_reload_keeps_totals() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("history.json");
        let mut history = History::load_from(path.clone());
        history.record(&snap(10.0, 1024), Duration::from_secs(1), 8, now_epoch());
        history.flush().unwrap();
        let reloaded = History::load_from(path);
        assert_eq!(reloaded.apps.get("Ghostty").unwrap().cpu_ms, 800);
        assert!(reloaded.warning.is_none());
    }

    #[test]
    fn corrupt_file_is_moved_aside() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("history.json");
        std::fs::write(&path, "{ not json").unwrap();
        let history = History::load_from(path.clone());
        assert!(history.apps.is_empty());
        assert!(history.warning.unwrap().contains("unreadable"));
        assert!(dir.path().join("history.json.corrupt").exists());
        assert!(!path.exists());
    }

    #[test]
    fn unsupported_version_is_moved_aside() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("history.json");
        std::fs::write(&path, r#"{"version":99,"apps":{}}"#).unwrap();
        let history = History::load_from(path);
        assert!(history.apps.is_empty());
        assert!(history.warning.unwrap().contains("not supported"));
    }

    #[test]
    fn old_records_are_pruned() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("history.json");
        let now = now_epoch();
        let old = now - RETENTION_SECS - 10;
        let recent = now - 10;
        let text = format!(
            r#"{{"version":1,"apps":{{"Old":{{"cpu_ms":1,"peak_memory":0,"disk_read":0,"disk_written":0,"last_seen":{old}}},"New":{{"cpu_ms":2,"peak_memory":0,"disk_read":0,"disk_written":0,"last_seen":{recent}}}}}}}"#
        );
        std::fs::write(&path, text).unwrap();
        let history = History::load_from(path);
        assert!(!history.apps.contains_key("Old"));
        assert!(history.apps.contains_key("New"));
    }

    #[test]
    fn remove_one_keeps_the_others() {
        let mut history = History::default();
        history.apps.insert("Safari".into(), AppRecord::default());
        history.apps.insert("Ghostty".into(), AppRecord::default());
        history.remove("Safari");
        assert!(!history.apps.contains_key("Safari"));
        assert!(history.apps.contains_key("Ghostty"));
        assert!(history.is_dirty());
    }

    #[test]
    fn clear_empties_records_and_marks_dirty() {
        let mut history = History::default();
        history.record(&snap(1.0, 10), Duration::from_secs(1), 8, 1);
        history.clear();
        assert!(history.apps.is_empty());
        assert!(history.is_dirty());
        assert!(history.rows().is_empty());
    }

    #[test]
    fn rows_are_sorted_by_cpu_time() {
        let mut history = History::default();
        history.record(&snap(1.0, 10), Duration::from_secs(1), 8, 1);
        history.apps.insert(
            "Zebra".into(),
            AppRecord {
                cpu_ms: 999_999,
                ..Default::default()
            },
        );
        let rows = history.rows();
        assert_eq!(rows[0].name, "Zebra");
        assert_eq!(rows[1].name, "Ghostty");
    }
}
