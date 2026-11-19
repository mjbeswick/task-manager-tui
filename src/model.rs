//! Plain data every tab renders from. No platform handles, no live state.

use std::collections::HashMap;

/// A process is identified by its id and its start time. A reused id must never
/// inherit the history or selection of the process that held it before.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProcKey {
    pub pid: u32,
    pub start_time: u64,
}

impl ProcKey {
    pub fn new(pid: u32, start_time: u64) -> Self {
        Self { pid, start_time }
    }
}

/// A selected row on the Processes tab: a group header or an expanded process.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProcSel {
    Kind(Kind),
    Group(Kind, String),
    Proc(ProcKey),
}

/// The three groups Windows 10 uses on the Processes tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Kind {
    App,
    Background,
    System,
}

impl Kind {
    pub const ALL: [Kind; 3] = [Kind::App, Kind::Background, Kind::System];

    pub fn label(self) -> &'static str {
        match self {
            Kind::App => "Apps",
            Kind::Background => "Background processes",
            Kind::System => "System processes",
        }
    }
}

/// Meaning a row's status word carries, independent of any colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Normal,
    Good,
    Muted,
    Warn,
    Danger,
}

/// Windows 10 status labels, mapped from platform process states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Status {
    Running,
    Sleeping,
    Suspended,
    NotResponding,
    Stopped,
}

impl Status {
    pub fn label(self) -> &'static str {
        match self {
            Status::Running => "Running",
            Status::Sleeping => "Sleeping",
            Status::Suspended => "Suspended",
            Status::NotResponding => "Not responding",
            Status::Stopped => "Stopped",
        }
    }

    pub fn tone(self) -> Tone {
        match self {
            Status::NotResponding => Tone::Danger,
            Status::Suspended | Status::Stopped => Tone::Warn,
            Status::Running | Status::Sleeping => Tone::Normal,
        }
    }
}

/// How charts are drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartStyle {
    Filled,
    Line,
    Dots,
}

impl ChartStyle {
    pub const ALL: [ChartStyle; 3] = [ChartStyle::Filled, ChartStyle::Line, ChartStyle::Dots];

    pub fn label(self) -> &'static str {
        match self {
            ChartStyle::Filled => "Filled",
            ChartStyle::Line => "Line",
            ChartStyle::Dots => "Dots",
        }
    }

    pub fn next(self) -> Self {
        match self {
            ChartStyle::Filled => ChartStyle::Line,
            ChartStyle::Line => ChartStyle::Dots,
            ChartStyle::Dots => ChartStyle::Filled,
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            ChartStyle::Filled => "filled",
            ChartStyle::Line => "line",
            ChartStyle::Dots => "dots",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "filled" => Some(ChartStyle::Filled),
            "line" => Some(ChartStyle::Line),
            "dots" => Some(ChartStyle::Dots),
            _ => None,
        }
    }
}

/// Chart colours, independent of the light or dark window theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartColor {
    PerResource,
    Teal,
    Blue,
    Orange,
    Magenta,
    Mono,
    Theme,
}

impl ChartColor {
    pub const ALL: [ChartColor; 7] = [
        ChartColor::PerResource,
        ChartColor::Teal,
        ChartColor::Blue,
        ChartColor::Orange,
        ChartColor::Magenta,
        ChartColor::Mono,
        ChartColor::Theme,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ChartColor::PerResource => "Per resource",
            ChartColor::Teal => "Teal",
            ChartColor::Blue => "Blue",
            ChartColor::Orange => "Orange",
            ChartColor::Magenta => "Magenta",
            ChartColor::Mono => "Mono",
            ChartColor::Theme => "Theme",
        }
    }

    pub fn next(self) -> Self {
        let index = Self::ALL.iter().position(|c| *c == self).unwrap_or(0);
        Self::ALL[(index + 1) % Self::ALL.len()]
    }

    pub fn key(self) -> &'static str {
        match self {
            ChartColor::PerResource => "per-resource",
            ChartColor::Teal => "teal",
            ChartColor::Blue => "blue",
            ChartColor::Orange => "orange",
            ChartColor::Magenta => "magenta",
            ChartColor::Mono => "mono",
            ChartColor::Theme => "theme",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "per-resource" => Some(ChartColor::PerResource),
            "teal" => Some(ChartColor::Teal),
            "blue" => Some(ChartColor::Blue),
            "orange" => Some(ChartColor::Orange),
            "magenta" => Some(ChartColor::Magenta),
            "mono" => Some(ChartColor::Mono),
            "theme" => Some(ChartColor::Theme),
            _ => None,
        }
    }

    /// Resolve against the active theme and the series being drawn, so dark mode
    /// keeps contrast and Per resource gives each resource its Windows 10 colour.
    pub fn color(
        self,
        theme: &crate::theme::Theme,
        resource: crate::app::ResourceChoice,
    ) -> ratatui::style::Color {
        use crate::app::ResourceChoice as R;
        use ratatui::style::Color;
        let dark = crate::theme::is_dark(theme.bg);
        match self {
            ChartColor::PerResource => match (resource, dark) {
                (R::Cpu, false) => Color::Rgb(0x11, 0x7D, 0xBB),
                (R::Memory, false) => Color::Rgb(0x8B, 0x12, 0xAE),
                (R::Disk, false) => Color::Rgb(0x3E, 0x87, 0x0A),
                (R::Network, false) => Color::Rgb(0xA7, 0x4F, 0x01),
                (R::Gpu, false) => Color::Rgb(0x1E, 0x8A, 0x73),
                (R::Cpu, true) => Color::Rgb(0x4C, 0xB0, 0xF0),
                (R::Memory, true) => Color::Rgb(0xC7, 0x74, 0xE8),
                (R::Disk, true) => Color::Rgb(0x7E, 0xD0, 0x50),
                (R::Network, true) => Color::Rgb(0xF0, 0x95, 0x4A),
                (R::Gpu, true) => Color::Rgb(0x4C, 0xC2, 0xA8),
            },
            ChartColor::Teal => theme.graph_fill,
            ChartColor::Blue => Color::Rgb(0x2B, 0x88, 0xC4),
            ChartColor::Orange => Color::Rgb(0xD1, 0x6B, 0x1F),
            ChartColor::Magenta => Color::Rgb(0xB0, 0x4A, 0xA8),
            ChartColor::Mono => theme.text,
            ChartColor::Theme => theme.graph_fill,
        }
    }
}

/// What each platform can actually report. Drives dashes and disabled actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capabilities {
    pub per_process_network: bool,
    pub per_process_gpu: bool,
    pub affinity: bool,
    pub handles: bool,
    pub compressed_memory: bool,
    pub link_speed: bool,
    pub disk_active: bool,
}

impl Capabilities {
    pub fn for_current_platform() -> Self {
        #[cfg(target_os = "macos")]
        {
            Self {
                per_process_network: false,
                per_process_gpu: false,
                affinity: false,
                handles: false,
                compressed_memory: false,
                link_speed: false,
                disk_active: false,
            }
        }
        #[cfg(target_os = "linux")]
        {
            Self {
                per_process_network: false,
                per_process_gpu: false,
                affinity: false,
                handles: true,
                compressed_memory: true,
                link_speed: false,
                disk_active: false,
            }
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            Self {
                per_process_network: false,
                per_process_gpu: false,
                affinity: false,
                handles: false,
                compressed_memory: false,
                link_speed: false,
                disk_active: false,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DiskIo {
    pub read_bps: f64,
    pub write_bps: f64,
}

impl DiskIo {
    pub fn total_bps(self) -> f64 {
        self.read_bps + self.write_bps
    }

    pub fn io_bps(self) -> f64 {
        self.total_bps()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Priority {
    pub nice: i32,
}

impl Priority {
    /// The unix nice range accepted by `renice`.
    pub const MIN: i32 = -20;
    pub const MAX: i32 = 19;

    /// Windows 10 priority levels map onto the unix nice range.
    pub const LEVELS: [(&'static str, i32); 5] = [
        ("High", -10),
        ("Above normal", -5),
        ("Normal", 0),
        ("Below normal", 5),
        ("Low", 10),
    ];

    /// Keep a typed value inside the range every target accepts.
    pub fn clamp(nice: i32) -> i32 {
        nice.clamp(Self::MIN, Self::MAX)
    }

    pub fn level_index(self) -> usize {
        let mut best = 0;
        for (index, level) in Self::LEVELS.iter().enumerate() {
            if (level.1 - self.nice).abs() < (Self::LEVELS[best].1 - self.nice).abs() {
                best = index;
            }
        }
        best
    }

    pub fn label(self) -> &'static str {
        let mut best = Self::LEVELS[0];
        for level in Self::LEVELS {
            if (level.1 - self.nice).abs() < (best.1 - self.nice).abs() {
                best = level;
            }
        }
        best.0
    }
}

/// POSIX signals the interface can send. The numbers are the same on Linux and
/// macOS, the only targets this crate builds for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Term,
    Kill,
    Int,
    Hup,
    Stop,
    Cont,
}

impl Signal {
    pub const ALL: [Signal; 6] = [
        Signal::Term,
        Signal::Kill,
        Signal::Int,
        Signal::Hup,
        Signal::Stop,
        Signal::Cont,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Signal::Term => "TERM",
            Signal::Kill => "KILL",
            Signal::Int => "INT",
            Signal::Hup => "HUP",
            Signal::Stop => "STOP",
            Signal::Cont => "CONT",
        }
    }

    pub fn number(self) -> i32 {
        match self {
            Signal::Hup => 1,
            Signal::Int => 2,
            Signal::Kill => 9,
            Signal::Term => 15,
            Signal::Cont => 18,
            Signal::Stop => 19,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Signal::Term => "Terminate",
            Signal::Kill => "Kill",
            Signal::Int => "Interrupt",
            Signal::Hup => "Hang up",
            Signal::Stop => "Stop",
            Signal::Cont => "Continue",
        }
    }

    /// Signals that can strand or abruptly destroy work get a second question.
    pub fn requires_confirm(self) -> bool {
        matches!(self, Signal::Kill | Signal::Stop)
    }
}

#[derive(Debug, Clone)]
pub struct ProcRow {
    pub key: ProcKey,
    pub name: String,
    pub kind: Kind,
    pub status: Status,
    /// Whole-system percentage: per-core values divided by the logical core count.
    pub cpu: f32,
    pub memory: u64,
    pub disk: DiskIo,
    pub network: Option<f64>,
    pub gpu: Option<f32>,
    pub user: Option<String>,
    pub uid: Option<u32>,
    pub parent_pid: Option<u32>,
    pub exe: Option<String>,
    pub cmd: String,
    pub threads: Option<usize>,
    pub priority: Priority,
    pub accumulated_cpu_ms: u64,
    pub run_time_secs: u64,
}

impl ProcRow {
    pub fn thread_count(&self) -> Option<usize> {
        self.threads
    }
}

#[derive(Debug, Clone, Default)]
pub struct CpuStat {
    pub total: f32,
    pub per_core: Vec<f32>,
    pub speed_mhz: u64,
    pub brand: String,
    pub logical: usize,
    pub physical: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct MemStat {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub swap_total: u64,
    pub swap_used: u64,
    pub compressed: Option<u64>,
    pub cached: Option<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct DiskStat {
    pub name: String,
    pub read_bps: f64,
    pub write_bps: f64,
    pub busy: f32,
    pub total: u64,
    pub available: u64,
}

impl DiskStat {
    pub fn io_bps(&self) -> f64 {
        self.read_bps + self.write_bps
    }
}

#[derive(Debug, Clone, Default)]
pub struct NetStat {
    pub name: String,
    pub rx_bps: f64,
    pub tx_bps: f64,
    pub link_speed: Option<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct GpuStat {
    pub name: String,
    pub usage: f32,
    pub used_memory: Option<u64>,
    pub total_memory: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SessionStatus {
    Active,
    #[default]
    Disconnected,
}

impl SessionStatus {
    pub fn label(self) -> &'static str {
        match self {
            SessionStatus::Active => "Active",
            SessionStatus::Disconnected => "Disconnected",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SessionRow {
    pub user: String,
    pub tty: String,
    pub host: String,
    pub status: SessionStatus,
    pub uid: u32,
    pub cpu: f32,
    pub memory: u64,
    pub disk: DiskIo,
    pub current: bool,
    /// Further sessions for this user, merged into this row.
    pub others: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ServiceState {
    Running,
    Stopped,
}

impl ServiceState {
    pub fn label(self) -> &'static str {
        match self {
            ServiceState::Running => "Running",
            ServiceState::Stopped => "Stopped",
        }
    }

    pub fn tone(self) -> Tone {
        match self {
            ServiceState::Running => Tone::Good,
            ServiceState::Stopped => Tone::Muted,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    User,
    System,
}

impl Scope {
    pub fn label(self) -> &'static str {
        match self {
            Scope::User => "user",
            Scope::System => "system",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServiceRow {
    pub name: String,
    pub pid: Option<u32>,
    pub description: Option<String>,
    pub state: ServiceState,
    pub scope: Scope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Impact {
    NotMeasured,
}

impl Impact {
    pub fn label(self) -> &'static str {
        match self {
            Impact::NotMeasured => "Not measured",
        }
    }
}

#[derive(Debug, Clone)]
pub struct StartupRow {
    pub name: String,
    pub command: String,
    pub owner: String,
    pub source: String,
    pub enabled: bool,
    pub impact: Impact,
    pub missing_program: bool,
    pub scope: Scope,
}

#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    pub cpu: CpuStat,
    pub memory: MemStat,
    pub disks: Vec<DiskStat>,
    pub networks: Vec<NetStat>,
    pub gpus: Vec<GpuStat>,
    pub processes: Vec<ProcRow>,
    pub sessions: Vec<SessionRow>,
    pub services: Vec<ServiceRow>,
    pub startup: Vec<StartupRow>,
    pub uptime_secs: u64,
    pub thread_count: Option<usize>,
    pub handle_count: Option<usize>,
    pub caps: Option<Capabilities>,
    pub errors: Vec<String>,
    pub services_error: Option<String>,
    pub startup_error: Option<String>,
    pub app_usage: Vec<AppUsage>,
    pub history_error: Option<String>,
}

impl Snapshot {
    pub fn process(&self, key: ProcKey) -> Option<&ProcRow> {
        self.processes.iter().find(|p| p.key == key)
    }

    pub fn by_pid(&self, pid: u32) -> Option<&ProcRow> {
        self.processes.iter().find(|p| p.key.pid == pid)
    }

    pub fn caps(&self) -> Capabilities {
        self.caps.unwrap_or_else(Capabilities::for_current_platform)
    }

    /// Fixture used by tests and golden snapshots. Two of each kind, stable values.
    pub fn fixture() -> Self {
        let proc = |pid: u32, name: &str, kind: Kind, cpu: f32, mem: u64| ProcRow {
            key: ProcKey::new(pid, 1_700_000_000 + pid as u64),
            name: name.to_string(),
            kind,
            status: Status::Running,
            cpu,
            memory: mem,
            disk: DiskIo {
                read_bps: 1024.0 * 12.0,
                write_bps: 1024.0 * 3.0,
            },
            network: None,
            gpu: None,
            user: Some("michael".into()),
            uid: Some(501),
            parent_pid: Some(1),
            exe: Some(format!("/usr/local/bin/{name}")),
            cmd: format!("{name} --flag"),
            threads: Some(4),
            priority: Priority { nice: 0 },
            accumulated_cpu_ms: 61_000,
            run_time_secs: 3600,
        };

        let processes = vec![
            proc(4242, "Ghostty", Kind::App, 2.1, 380 * 1024 * 1024),
            proc(4243, "Safari", Kind::App, 6.4, 1_240 * 1024 * 1024),
            proc(500, "WindowServer", Kind::System, 4.8, 1_200 * 1024 * 1024),
            proc(1, "launchd", Kind::System, 0.2, 12 * 1024 * 1024),
            proc(900, "node", Kind::Background, 12.5, 512 * 1024 * 1024),
            proc(901, "postgres", Kind::Background, 1.1, 96 * 1024 * 1024),
        ];
        let thread_count = processes
            .iter()
            .map(|p| p.threads.unwrap_or(0))
            .sum::<usize>();

        Self {
            cpu: CpuStat {
                total: 27.0,
                per_core: vec![41.0, 18.0, 33.0, 16.0, 27.0, 12.0, 55.0, 14.0],
                speed_mhz: 3204,
                brand: "Apple M1".into(),
                logical: 8,
                physical: Some(8),
            },
            memory: MemStat {
                total: 16 * 1024 * 1024 * 1024,
                used: 10 * 1024 * 1024 * 1024,
                available: 6 * 1024 * 1024 * 1024,
                swap_total: 2 * 1024 * 1024 * 1024,
                swap_used: 256 * 1024 * 1024,
                compressed: None,
                cached: Some(2 * 1024 * 1024 * 1024),
            },
            disks: vec![DiskStat {
                name: "disk0".into(),
                read_bps: 4.2 * 1024.0 * 1024.0,
                write_bps: 1.1 * 1024.0 * 1024.0,
                busy: 12.0,
                total: 994 * 1024 * 1024 * 1024,
                available: 582 * 1024 * 1024 * 1024,
            }],
            networks: vec![NetStat {
                name: "en0".into(),
                rx_bps: 1.2 * 1024.0 * 1024.0,
                tx_bps: 240.0 * 1024.0,
                link_speed: None,
            }],
            gpus: Vec::new(),
            processes,
            sessions: vec![SessionRow {
                user: "michael".into(),
                tty: "ttys001".into(),
                host: "localhost".into(),
                status: SessionStatus::Active,
                uid: 501,
                cpu: 24.0,
                memory: 4 * 1024 * 1024 * 1024,
                disk: DiskIo {
                    read_bps: 1024.0 * 512.0,
                    write_bps: 1024.0 * 64.0,
                },
                current: true,
                others: 0,
            }],
            services: vec![
                ServiceRow {
                    name: "com.apple.metadata.mds".into(),
                    pid: Some(312),
                    description: None,
                    state: ServiceState::Running,
                    scope: Scope::System,
                },
                ServiceRow {
                    name: "homebrew.mxcl.nginx".into(),
                    pid: None,
                    description: Some("nginx web server".into()),
                    state: ServiceState::Stopped,
                    scope: Scope::User,
                },
            ],
            startup: vec![StartupRow {
                name: "Rectangle".into(),
                command: "/Applications/Rectangle.app/Contents/MacOS/Rectangle".into(),
                owner: "michael".into(),
                source: "~/Library/LaunchAgents/com.knollsoft.Rectangle.plist".into(),
                enabled: true,
                impact: Impact::NotMeasured,
                missing_program: false,
                scope: Scope::User,
            }],
            uptime_secs: 3 * 86_400 + 4 * 3600,
            thread_count: Some(thread_count),
            handle_count: Some(4_812),
            caps: None,
            errors: Vec::new(),
            services_error: None,
            startup_error: None,
            app_usage: vec![
                AppUsage {
                    name: "Safari".into(),
                    cpu_ms: 3_600_000,
                    peak_memory: 2 * 1024 * 1024 * 1024,
                    disk_read: 40 * 1024 * 1024,
                    disk_written: 8 * 1024 * 1024,
                    last_seen: 1_700_000_000,
                    network: None,
                },
                AppUsage {
                    name: "Ghostty".into(),
                    cpu_ms: 61_000,
                    peak_memory: 380 * 1024 * 1024,
                    disk_read: 1024 * 1024,
                    disk_written: 512 * 1024,
                    last_seen: 1_700_000_100,
                    network: None,
                },
            ],
            history_error: None,
        }
    }
}

/// Per-app totals used by the App history tab.
#[derive(Debug, Clone, Default)]
pub struct AppUsage {
    pub name: String,
    pub cpu_ms: u64,
    pub peak_memory: u64,
    pub disk_read: u64,
    pub disk_written: u64,
    pub last_seen: u64,
    pub network: Option<u64>,
}

impl AppUsage {
    pub fn sort_by_cpu(rows: &mut [AppUsage]) {
        rows.sort_by(|a, b| b.cpu_ms.cmp(&a.cpu_ms).then(a.name.cmp(&b.name)));
    }
}

/// Sessions grouped by uid, used by the Users tab.
pub fn sessions_by_uid(procs: &[ProcRow]) -> HashMap<Option<String>, (f32, u64, DiskIo)> {
    let mut map: HashMap<Option<String>, (f32, u64, DiskIo)> = HashMap::new();
    for p in procs {
        let entry = map.entry(p.user.clone()).or_default();
        entry.0 += p.cpu;
        entry.1 += p.memory;
        entry.2.read_bps += p.disk.read_bps;
        entry.2.write_bps += p.disk.write_bps;
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proc_key_distinguishes_reused_pids() {
        let old = ProcKey::new(900, 100);
        let new = ProcKey::new(900, 200);
        assert_ne!(old, new);
        let mut set = std::collections::HashSet::new();
        set.insert(old);
        assert!(!set.contains(&new));
        assert!(set.contains(&old));
    }

    #[test]
    fn status_tones_follow_meaning() {
        assert_eq!(Status::NotResponding.tone(), Tone::Danger);
        assert_eq!(Status::Suspended.tone(), Tone::Warn);
        assert_eq!(Status::Stopped.tone(), Tone::Warn);
        assert_eq!(Status::Running.tone(), Tone::Normal);
        assert_eq!(Status::Sleeping.tone(), Tone::Normal);
        assert_eq!(ServiceState::Running.tone(), Tone::Good);
        assert_eq!(ServiceState::Stopped.tone(), Tone::Muted);
    }

    #[test]
    fn signals_have_posix_numbers_and_unique_names() {
        assert_eq!(
            Signal::ALL.iter().map(|s| s.number()).collect::<Vec<_>>(),
            vec![15, 9, 2, 1, 19, 18]
        );
        let mut names: Vec<_> = Signal::ALL.iter().map(|s| s.name()).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), Signal::ALL.len());
    }

    #[test]
    fn priority_clamps_to_the_renice_range() {
        assert_eq!(Priority::clamp(-40), -20);
        assert_eq!(Priority::clamp(40), 19);
        assert_eq!(Priority::clamp(7), 7);
    }

    #[test]
    fn priority_labels_round_trip() {
        assert_eq!(Priority { nice: 0 }.label(), "Normal");
        assert_eq!(Priority { nice: -10 }.label(), "High");
        assert_eq!(Priority { nice: 9 }.label(), "Low");
    }

    #[test]
    fn fixture_has_every_kind() {
        let snap = Snapshot::fixture();
        for kind in Kind::ALL {
            assert!(
                snap.processes.iter().any(|p| p.kind == kind),
                "missing {kind:?}"
            );
        }
        assert_eq!(snap.thread_count, Some(24));
    }

    #[test]
    fn theme_chart_colour_follows_the_palette() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Nord, false);
        assert_eq!(
            ChartColor::Theme.color(&theme, crate::app::ResourceChoice::Cpu),
            theme.graph_fill
        );
    }

    #[test]
    fn chart_choices_cycle_and_parse() {
        assert_eq!(ChartStyle::Filled.next(), ChartStyle::Line);
        assert_eq!(ChartStyle::Dots.next(), ChartStyle::Filled);
        assert_eq!(ChartStyle::parse("dots"), Some(ChartStyle::Dots));
        assert_eq!(ChartStyle::parse("nope"), None);
        assert_eq!(ChartColor::Theme.next(), ChartColor::PerResource);
        assert_eq!(ChartColor::Mono.next(), ChartColor::Theme);
        assert_eq!(ChartColor::parse("orange"), Some(ChartColor::Orange));
        assert_eq!(ChartColor::parse("nope"), None);
        for colour in ChartColor::ALL {
            assert_eq!(ChartColor::parse(colour.key()), Some(colour));
        }
        for style in ChartStyle::ALL {
            assert_eq!(ChartStyle::parse(style.key()), Some(style));
        }
    }

    #[test]
    fn chart_colours_stay_visible() {
        use crate::app::ResourceChoice as R;
        use crate::theme::{DARK, LIGHT, contrast};
        for theme in [LIGHT, DARK] {
            for colour in ChartColor::ALL {
                for resource in [R::Cpu, R::Memory, R::Disk, R::Network, R::Gpu] {
                    let ratio = contrast(colour.color(&theme, resource), theme.surface);
                    assert!(
                        ratio >= 3.0,
                        "{:?} {colour:?} {resource:?} is {ratio}",
                        theme.name
                    );
                }
            }
        }
    }

    #[test]
    fn capabilities_match_the_target() {
        let caps = Capabilities::for_current_platform();
        assert!(!caps.per_process_network);
        assert!(!caps.per_process_gpu);
        assert!(!caps.affinity);
        assert!(!caps.link_speed);
        #[cfg(target_os = "linux")]
        {
            assert!(caps.handles);
            assert!(caps.compressed_memory);
        }
        #[cfg(target_os = "macos")]
        {
            assert!(!caps.handles);
            assert!(!caps.compressed_memory);
        }
    }

    #[test]
    fn sessions_aggregate_by_user() {
        let snap = Snapshot::fixture();
        let map = sessions_by_uid(&snap.processes);
        let (cpu, mem, _) = map.get(&Some("michael".to_string())).unwrap();
        assert!(*cpu > 27.0);
        assert!(*mem > 2 * 1024 * 1024 * 1024);
    }
}
