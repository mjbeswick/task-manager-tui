//! Classification and the row list the Processes tab renders.

use crate::model::{DiskIo, Kind, ProcKey, ProcRow, ProcSel};
use std::collections::HashSet;
use std::path::Path;

/// Names that are system processes regardless of owner.
pub const SYSTEM_NAMES: &[&str] = &[
    "kernel_task",
    "launchd",
    "launchd_sim",
    "WindowServer",
    "loginwindow",
    "systemd",
    "kthreadd",
    "init",
    "systemd-journald",
    "systemd-logind",
    "systemd-udevd",
];

/// Which application names are graphical programs, gathered from desktop files.
#[derive(Debug, Default, Clone)]
pub struct Classifier {
    desktop_programs: HashSet<String>,
}

impl Classifier {
    pub fn new() -> Self {
        #[cfg(target_os = "linux")]
        {
            Self {
                desktop_programs: scan_desktop_programs(),
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            Self::default()
        }
    }

    pub fn has_desktop_program(&self, program: &str) -> bool {
        self.desktop_programs.contains(program)
    }

    pub fn classify(&self, name: &str, exe: Option<&str>, uid: Option<u32>, pid: u32) -> Kind {
        let desktop_match = exe
            .and_then(|path| Path::new(path).file_name())
            .and_then(|name| name.to_str())
            .is_some_and(|program| self.has_desktop_program(program));
        classify_parts(name, exe, uid, pid, desktop_match)
    }
}

pub fn classify_parts(
    name: &str,
    exe: Option<&str>,
    uid: Option<u32>,
    pid: u32,
    desktop_match: bool,
) -> Kind {
    if uid == Some(0) || pid == 1 || SYSTEM_NAMES.contains(&name) {
        return Kind::System;
    }
    if exe.is_some_and(is_app_bundle) || desktop_match {
        return Kind::App;
    }
    Kind::Background
}

/// A graphical application bundle, not an agent that happens to live inside one.
fn is_app_bundle(path: &str) -> bool {
    let bundled = path.contains(".app/") || path.contains(".appex/");
    let user_facing = path.starts_with("/Applications/")
        || path.starts_with("/System/Applications/")
        || (path.starts_with("/Users/") && path.contains("/Applications/"));
    bundled && user_facing
}

#[cfg(target_os = "linux")]
fn scan_desktop_programs() -> HashSet<String> {
    let mut programs = HashSet::new();
    let mut dirs = vec![
        std::path::PathBuf::from("/usr/share/applications"),
        std::path::PathBuf::from("/usr/local/share/applications"),
    ];
    if let Some(home) = std::env::var_os("HOME") {
        dirs.push(std::path::PathBuf::from(home).join(".local/share/applications"));
    }
    for dir in dirs {
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
            for line in text.lines() {
                if let Some(exec) = line.strip_prefix("Exec=")
                    && let Some(program) = exec.split_whitespace().next()
                {
                    let program = program.trim_matches('"').trim_matches('\'');
                    if let Some(name) = Path::new(program).file_name().and_then(|n| n.to_str()) {
                        programs.insert(name.to_string());
                    }
                }
            }
        }
    }
    programs
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortKey {
    Name,
    Pid,
    Status,
    User,
    Cpu,
    Memory,
    Disk,
    Network,
    Gpu,
}

impl SortKey {
    pub const PROCESS_KEYS: [SortKey; 7] = [
        SortKey::Name,
        SortKey::Status,
        SortKey::Cpu,
        SortKey::Memory,
        SortKey::Disk,
        SortKey::Network,
        SortKey::Gpu,
    ];
    pub const DETAIL_KEYS: [SortKey; 9] = [
        SortKey::Name,
        SortKey::Pid,
        SortKey::Status,
        SortKey::User,
        SortKey::Cpu,
        SortKey::Memory,
        SortKey::Disk,
        SortKey::Network,
        SortKey::Gpu,
    ];

    pub fn process_column(self) -> usize {
        Self::PROCESS_KEYS
            .iter()
            .position(|key| *key == self)
            .unwrap_or(0)
    }

    pub fn detail_column(self) -> usize {
        Self::DETAIL_KEYS
            .iter()
            .position(|key| *key == self)
            .unwrap_or(0)
    }

    pub fn from_process_column(index: usize) -> Self {
        Self::PROCESS_KEYS[index.min(Self::PROCESS_KEYS.len() - 1)]
    }

    pub fn from_detail_column(index: usize) -> Self {
        Self::DETAIL_KEYS[index.min(Self::DETAIL_KEYS.len() - 1)]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDir {
    Asc,
    Desc,
}

impl SortDir {
    pub fn flipped(self) -> Self {
        match self {
            SortDir::Asc => SortDir::Desc,
            SortDir::Desc => SortDir::Asc,
        }
    }
}

/// One rendered row. A group header or an expanded child process.
#[derive(Debug, Clone)]
pub enum ProcItem {
    Header {
        kind: Kind,
        count: usize,
        expanded: bool,
    },
    Group {
        kind: Kind,
        name: String,
        cpu: f32,
        memory: u64,
        disk: DiskIo,
        count: usize,
        expanded: bool,
    },
    Child {
        key: ProcKey,
        name: String,
        depth: u8,
    },
}

impl ProcItem {
    pub fn sel(&self) -> ProcSel {
        match self {
            ProcItem::Header { kind, .. } => ProcSel::Kind(*kind),
            ProcItem::Group { kind, name, .. } => ProcSel::Group(*kind, name.clone()),
            ProcItem::Child { key, .. } => ProcSel::Proc(*key),
        }
    }
}

struct GroupBucket<'a> {
    name: String,
    rows: Vec<&'a ProcRow>,
}

fn sort_rows(rows: &mut [&ProcRow], sort: SortKey, dir: SortDir) {
    rows.sort_by(|a, b| {
        let ordering = match sort {
            SortKey::Name => a.name.cmp(&b.name),
            SortKey::Pid => a.key.pid.cmp(&b.key.pid),
            SortKey::Status => a.status.cmp(&b.status),
            SortKey::User => a.user.cmp(&b.user),
            SortKey::Cpu => a.cpu.total_cmp(&b.cpu),
            SortKey::Memory => a.memory.cmp(&b.memory),
            SortKey::Disk => a.disk.total_bps().total_cmp(&b.disk.total_bps()),
            SortKey::Network => a
                .network
                .unwrap_or(0.0)
                .total_cmp(&b.network.unwrap_or(0.0)),
            SortKey::Gpu => a.gpu.unwrap_or(0.0).total_cmp(&b.gpu.unwrap_or(0.0)),
        };
        let ordering = match dir {
            SortDir::Asc => ordering,
            SortDir::Desc => ordering.reverse(),
        };
        ordering.then_with(|| a.name.cmp(&b.name))
    });
}

/// Keep the previous block order while values refresh. A block is a kind header
/// or a group header with its children, so grouping can never interleave.
pub fn keep_order(old: &[ProcItem], fresh: Vec<ProcItem>) -> Vec<ProcItem> {
    use std::collections::HashMap;
    let previous: HashMap<ProcSel, usize> = old
        .iter()
        .enumerate()
        .map(|(index, item)| (item.sel(), index))
        .collect();

    let mut blocks: Vec<Vec<ProcItem>> = Vec::new();
    for item in fresh {
        match &item {
            ProcItem::Child { .. } => {
                if let Some(last) = blocks.last_mut() {
                    last.push(item);
                } else {
                    blocks.push(vec![item]);
                }
            }
            _ => blocks.push(vec![item]),
        }
    }
    blocks.sort_by_key(|block| {
        block
            .first()
            .and_then(|item| previous.get(&item.sel()).copied())
            .unwrap_or(usize::MAX)
    });
    blocks.into_iter().flatten().collect()
}

/// Build the visible rows: groups in Windows 10 order, children when expanded.
pub fn build(
    rows: &[ProcRow],
    sort: SortKey,
    dir: SortDir,
    collapsed_kinds: &HashSet<Kind>,
    expanded_groups: &HashSet<(Kind, String)>,
    filter: &str,
) -> Vec<ProcItem> {
    let needle = filter.trim().to_lowercase();
    let mut out = Vec::new();

    for kind in Kind::ALL {
        let of_kind: Vec<&ProcRow> = rows.iter().filter(|r| r.kind == kind).collect();
        if of_kind.is_empty() {
            continue;
        }
        let mut groups: Vec<GroupBucket> = Vec::new();
        for row in of_kind.iter().copied() {
            match groups.iter_mut().find(|g| g.name == row.name) {
                Some(group) => group.rows.push(row),
                None => groups.push(GroupBucket {
                    name: row.name.clone(),
                    rows: vec![row],
                }),
            }
        }
        if groups.is_empty() {
            continue;
        }

        // Keep only groups with at least one matching child.
        groups.retain(|bucket| {
            needle.is_empty()
                || bucket.name.to_lowercase().contains(&needle)
                || bucket
                    .rows
                    .iter()
                    .any(|r| r.name.to_lowercase().contains(&needle))
        });
        if groups.is_empty() {
            continue;
        }

        let kind_expanded = !collapsed_kinds.contains(&kind);
        out.push(ProcItem::Header {
            kind,
            count: of_kind.len(),
            expanded: kind_expanded,
        });
        if !kind_expanded {
            continue;
        }

        let aggregate = |bucket: &GroupBucket| {
            (
                bucket.rows.iter().map(|r| r.cpu).sum::<f32>(),
                bucket.rows.iter().map(|r| r.memory).sum::<u64>(),
                bucket.rows.iter().map(|r| r.disk.total_bps()).sum::<f64>(),
            )
        };

        // Order the remaining groups.
        groups.sort_by(|a, b| {
            let (cpu_a, mem_a, disk_a) = aggregate(a);
            let (cpu_b, mem_b, disk_b) = aggregate(b);
            let ordering = match sort {
                SortKey::Name => a.name.cmp(&b.name),
                SortKey::Cpu => cpu_a.total_cmp(&cpu_b),
                SortKey::Memory => mem_a.cmp(&mem_b),
                SortKey::Disk => disk_a.total_cmp(&disk_b),
                SortKey::Pid
                | SortKey::Status
                | SortKey::User
                | SortKey::Network
                | SortKey::Gpu => std::cmp::Ordering::Equal,
            };
            match dir {
                SortDir::Asc => ordering,
                SortDir::Desc => ordering.reverse(),
            }
            .then_with(|| a.name.cmp(&b.name))
        });

        for bucket in groups {
            let (cpu, memory, _) = aggregate(&bucket);
            let disk = DiskIo {
                read_bps: bucket.rows.iter().map(|r| r.disk.read_bps).sum(),
                write_bps: bucket.rows.iter().map(|r| r.disk.write_bps).sum(),
            };
            let is_expanded = expanded_groups.contains(&(kind, bucket.name.clone()));
            out.push(ProcItem::Group {
                kind,
                name: bucket.name.clone(),
                cpu,
                memory,
                disk,
                count: bucket.rows.len(),
                expanded: is_expanded,
            });
            if is_expanded {
                let group_matches =
                    needle.is_empty() || bucket.name.to_lowercase().contains(&needle);
                let mut children: Vec<&ProcRow> = bucket
                    .rows
                    .iter()
                    .copied()
                    .filter(|row| group_matches || row.name.to_lowercase().contains(&needle))
                    .collect();
                sort_rows(&mut children, sort, dir);
                for row in children {
                    out.push(ProcItem::Child {
                        key: row.key,
                        name: row.name.clone(),
                        depth: 1,
                    });
                }
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Priority, Status};

    fn row(pid: u32, name: &str, kind: Kind, cpu: f32, mem: u64) -> ProcRow {
        ProcRow {
            key: ProcKey::new(pid, 1),
            name: name.into(),
            kind,
            status: Status::Running,
            cpu,
            memory: mem,
            disk: DiskIo::default(),
            network: None,
            gpu: None,
            user: Some("michael".into()),
            uid: Some(501),
            parent_pid: None,
            exe: None,
            cmd: name.into(),
            threads: Some(1),
            priority: Priority { nice: 0 },
            accumulated_cpu_ms: 0,
            run_time_secs: 1,
        }
    }

    #[test]
    fn classification_prefers_system_then_app() {
        assert_eq!(
            classify_parts("bash", None, Some(0), 10, false),
            Kind::System
        );
        assert_eq!(
            classify_parts("kernel_task", None, Some(501), 10, false),
            Kind::System
        );
        assert_eq!(
            classify_parts("launchd", None, Some(501), 1, false),
            Kind::System
        );
        assert_eq!(
            classify_parts(
                "Safari",
                Some("/Applications/Safari.app/Contents/MacOS/Safari"),
                Some(501),
                99,
                false
            ),
            Kind::App
        );
        assert_eq!(
            classify_parts("firefox", Some("/usr/bin/firefox"), Some(501), 99, true),
            Kind::App
        );
        assert_eq!(
            classify_parts("node", Some("/usr/bin/node"), Some(501), 99, false),
            Kind::Background
        );
    }

    #[test]
    fn groups_are_in_windows_order() {
        let rows = vec![
            row(1, "node", Kind::Background, 5.0, 10),
            row(2, "Safari", Kind::App, 6.0, 20),
            row(3, "launchd", Kind::System, 1.0, 30),
        ];
        let items = build(
            &rows,
            SortKey::Cpu,
            SortDir::Desc,
            &HashSet::new(),
            &HashSet::new(),
            "",
        );
        let headers: Vec<_> = items
            .iter()
            .filter_map(|i| match i {
                ProcItem::Header { kind, .. } => Some(*kind),
                _ => None,
            })
            .collect();
        assert_eq!(headers, vec![Kind::App, Kind::Background, Kind::System]);
    }

    #[test]
    fn same_name_aggregates_into_one_row() {
        let rows = vec![
            row(1, "Chrome", Kind::App, 2.0, 100),
            row(2, "Chrome", Kind::App, 3.0, 200),
        ];
        let items = build(
            &rows,
            SortKey::Cpu,
            SortDir::Desc,
            &HashSet::new(),
            &HashSet::new(),
            "",
        );
        assert_eq!(items.len(), 2, "header and one grouped row");
        match &items[1] {
            ProcItem::Group {
                cpu, memory, count, ..
            } => {
                assert_eq!(*cpu, 5.0);
                assert_eq!(*memory, 300);
                assert_eq!(*count, 2);
            }
            _ => panic!("expected a group"),
        }
    }

    #[test]
    fn expanding_shows_sorted_children() {
        let rows = vec![
            row(1, "Chrome", Kind::App, 2.0, 100),
            row(2, "Chrome", Kind::App, 9.0, 200),
        ];
        let mut expanded = HashSet::new();
        expanded.insert((Kind::App, "Chrome".to_string()));
        let items = build(
            &rows,
            SortKey::Cpu,
            SortDir::Desc,
            &HashSet::new(),
            &expanded,
            "",
        );
        assert_eq!(items.len(), 4, "header, group, two children");
        match &items[2] {
            ProcItem::Child { key, .. } => assert_eq!(key.pid, 2),
            _ => panic!("expected child first by cpu"),
        }
    }

    #[test]
    fn filter_keeps_matching_groups_and_children() {
        let rows = vec![
            row(1, "Chrome", Kind::App, 2.0, 100),
            row(2, "Safari", Kind::App, 9.0, 200),
            row(3, "node", Kind::Background, 1.0, 10),
        ];
        let items = build(
            &rows,
            SortKey::Name,
            SortDir::Asc,
            &HashSet::new(),
            &HashSet::new(),
            "chrom",
        );
        let groups: Vec<_> = items
            .iter()
            .filter_map(|i| match i {
                ProcItem::Group { name, .. } => Some(name.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(groups, vec!["Chrome"]);
        assert!(items.iter().all(|item| !matches!(
            item,
            ProcItem::Group { .. } | ProcItem::Child { .. } | ProcItem::Header { .. }
        ) || match item {
            ProcItem::Group { name, .. } | ProcItem::Child { name, .. } => name == "Chrome",
            ProcItem::Header { .. } => true,
        }));
    }

    #[test]
    fn keep_order_preserves_blocks_and_appends_new_ones() {
        let old_rows = vec![
            row(1, "alpha", Kind::App, 10.0, 10),
            row(2, "beta", Kind::App, 2.0, 20),
        ];
        let old = build(
            &old_rows,
            SortKey::Cpu,
            SortDir::Desc,
            &HashSet::new(),
            &HashSet::new(),
            "",
        );
        // beta becomes the busiest, and a new app appears.
        let fresh_rows = vec![
            row(1, "alpha", Kind::App, 1.0, 10),
            row(2, "beta", Kind::App, 90.0, 20),
            row(3, "gamma", Kind::App, 3.0, 30),
        ];
        let fresh = build(
            &fresh_rows,
            SortKey::Cpu,
            SortDir::Desc,
            &HashSet::new(),
            &HashSet::new(),
            "",
        );
        let ordered = keep_order(&old, fresh);
        let names: Vec<_> = ordered
            .iter()
            .filter_map(|item| match item {
                ProcItem::Group { name, .. } => Some(name.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(names, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn filter_with_no_match_is_empty() {
        let rows = vec![row(1, "Chrome", Kind::App, 2.0, 100)];
        assert!(
            build(
                &rows,
                SortKey::Name,
                SortDir::Asc,
                &HashSet::new(),
                &HashSet::new(),
                "zzz"
            )
            .is_empty()
        );
    }

    #[test]
    fn group_sorting_is_stable_for_equal_values() {
        let rows = vec![
            row(1, "beta", Kind::Background, 5.0, 10),
            row(2, "alpha", Kind::Background, 5.0, 10),
        ];
        let items = build(
            &rows,
            SortKey::Cpu,
            SortDir::Desc,
            &HashSet::new(),
            &HashSet::new(),
            "",
        );
        let names: Vec<_> = items
            .iter()
            .filter_map(|i| match i {
                ProcItem::Group { name, .. } => Some(name.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(names, vec!["alpha", "beta"]);
    }
}
