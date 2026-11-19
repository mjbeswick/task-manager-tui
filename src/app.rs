//! Application state. View state only, plus the last snapshot.

use crate::model::{
    AppUsage, ChartColor, ChartStyle, ProcKey, ProcRow, ProcSel, ServiceRow, SessionRow, Snapshot,
    StartupRow,
};
use crate::platform::ServiceAction;
use crate::sample::group::{self, ProcItem, SortDir, SortKey};
use crate::store::settings::Settings;
use crate::theme::{Theme, ThemeName};
use crate::ui::graph::GraphStore;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tab {
    Processes,
    Performance,
    AppHistory,
    Startup,
    Users,
    Details,
    Services,
}

impl Tab {
    pub const ALL: [Tab; 7] = [
        Tab::Processes,
        Tab::Performance,
        Tab::AppHistory,
        Tab::Startup,
        Tab::Users,
        Tab::Details,
        Tab::Services,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Tab::Processes => "Processes",
            Tab::Performance => "Performance",
            Tab::AppHistory => "App history",
            Tab::Startup => "Startup",
            Tab::Users => "Users",
            Tab::Details => "Details",
            Tab::Services => "Services",
        }
    }

    pub fn short_label(self) -> &'static str {
        match self {
            Tab::Processes => "Proc",
            Tab::Performance => "Perf",
            Tab::AppHistory => "History",
            Tab::Startup => "Startup",
            Tab::Users => "Users",
            Tab::Details => "Details",
            Tab::Services => "Services",
        }
    }

    pub fn index(self) -> usize {
        Self::ALL.iter().position(|tab| *tab == self).unwrap_or(0)
    }

    pub fn from_index(index: usize) -> Self {
        Self::ALL[index.min(Self::ALL.len() - 1)]
    }

    pub fn next(self) -> Self {
        Self::from_index((self.index() + 1) % Self::ALL.len())
    }

    pub fn previous(self) -> Self {
        Self::from_index((self.index() + Self::ALL.len() - 1) % Self::ALL.len())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    Cpu,
    Memory,
    Disk,
    Network,
    Gpu,
}

impl Resource {
    pub const ALL: [Resource; 5] = [
        Resource::Cpu,
        Resource::Memory,
        Resource::Disk,
        Resource::Network,
        Resource::Gpu,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Resource::Cpu => "CPU",
            Resource::Memory => "Memory",
            Resource::Disk => "Disk",
            Resource::Network => "Network",
            Resource::Gpu => "GPU",
        }
    }

    pub fn next(self) -> Self {
        let index = Self::ALL.iter().position(|r| *r == self).unwrap_or(0);
        Self::ALL[(index + 1) % Self::ALL.len()]
    }

    pub fn previous(self) -> Self {
        let index = Self::ALL.iter().position(|r| *r == self).unwrap_or(0);
        Self::ALL[(index + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

/// How long a fresh sample leaves the row order alone after a move, so each
/// keypress moves one visible row instead of chasing a re-sorted list.
pub const NAV_FREEZE: std::time::Duration = std::time::Duration::from_millis(2_000);

#[derive(Debug, Default)]
pub struct ProcessTab {
    pub items: Vec<ProcItem>,
    pub selected: usize,
    pub key: Option<ProcSel>,
    pub scroll: usize,
    pub sort: SortKeyState,
    pub collapsed_kinds: HashSet<crate::model::Kind>,
    pub expanded_groups: HashSet<(crate::model::Kind, String)>,
    pub filter: String,
    pub more_details: bool,
    pub show_optional_columns: bool,
    pub last_navigation: Option<std::time::Instant>,
}

#[derive(Debug, Clone, Copy)]
pub struct SortKeyState {
    pub key: SortKey,
    pub dir: SortDir,
}

impl Default for SortKeyState {
    fn default() -> Self {
        Self {
            key: SortKey::Cpu,
            dir: SortDir::Desc,
        }
    }
}

impl SortKeyState {
    pub fn next_column(&mut self, keys: &[SortKey]) {
        let index = keys.iter().position(|k| *k == self.key).unwrap_or(0);
        self.key = keys[(index + 1) % keys.len()];
    }

    pub fn previous_column(&mut self, keys: &[SortKey]) {
        let index = keys.iter().position(|k| *k == self.key).unwrap_or(0);
        self.key = keys[(index + keys.len() - 1) % keys.len()];
    }

    pub fn flip(&mut self) {
        self.dir = self.dir.flipped();
    }
}

impl ProcessTab {
    pub fn refresh(&mut self, snapshot: &Snapshot) {
        let fresh = group::build(
            &snapshot.processes,
            self.sort.key,
            self.sort.dir,
            &self.collapsed_kinds,
            &self.expanded_groups,
            &self.filter,
        );
        self.items = if self.navigating() {
            group::keep_order(&self.items, fresh)
        } else {
            fresh
        };
        self.sync(snapshot);
    }

    /// True while the row order should hold still for the user.
    pub fn navigating(&self) -> bool {
        self.last_navigation
            .is_some_and(|at| at.elapsed() < NAV_FREEZE)
    }

    pub fn mark_navigation(&mut self) {
        self.last_navigation = Some(std::time::Instant::now());
    }

    /// Test hook: run a refresh with the freeze forced on or off.
    pub fn refresh_with(&mut self, snapshot: &Snapshot, freeze: bool) {
        let was = self.last_navigation;
        self.last_navigation = if freeze {
            Some(std::time::Instant::now())
        } else {
            None
        };
        self.refresh(snapshot);
        self.last_navigation = was;
    }

    fn sync(&mut self, snapshot: &Snapshot) {
        if self.items.is_empty() {
            self.selected = 0;
            self.key = None;
            self.scroll = 0;
            return;
        }
        if let Some(key) = self.key.clone() {
            if let Some(index) = self.items.iter().position(|item| item.sel() == key) {
                self.selected = index;
            } else {
                self.selected = self.selected.min(self.items.len() - 1);
                self.key = Some(self.items[self.selected].sel());
            }
        } else {
            self.selected = 0;
            self.key = Some(self.items[0].sel());
        }
        // Group rows carry no process, so nothing else to resolve.
        let _ = snapshot;
    }

    pub fn current(&self) -> Option<&ProcItem> {
        self.items.get(self.selected)
    }

    pub fn selected_key(&self) -> Option<ProcSel> {
        self.key
            .clone()
            .or_else(|| self.current().map(ProcItem::sel))
    }
}

#[derive(Debug, Default)]
pub struct DetailsTab {
    pub selected: usize,
    pub key: Option<ProcKey>,
    pub scroll: usize,
    pub sort: SortKeyState,
    pub filter: String,
    pub panel: bool,
}

#[derive(Debug, Default)]
pub struct PerformanceTab {
    pub resource: ResourceChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResourceChoice {
    #[default]
    Cpu,
    Memory,
    Disk,
    Network,
    Gpu,
}

impl ResourceChoice {
    pub fn next(self) -> Self {
        match self {
            ResourceChoice::Cpu => ResourceChoice::Memory,
            ResourceChoice::Memory => ResourceChoice::Disk,
            ResourceChoice::Disk => ResourceChoice::Network,
            ResourceChoice::Network => ResourceChoice::Gpu,
            ResourceChoice::Gpu => ResourceChoice::Cpu,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            ResourceChoice::Cpu => ResourceChoice::Gpu,
            ResourceChoice::Memory => ResourceChoice::Cpu,
            ResourceChoice::Disk => ResourceChoice::Memory,
            ResourceChoice::Network => ResourceChoice::Disk,
            ResourceChoice::Gpu => ResourceChoice::Network,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ResourceChoice::Cpu => "CPU",
            ResourceChoice::Memory => "Memory",
            ResourceChoice::Disk => "Disk",
            ResourceChoice::Network => "Network",
            ResourceChoice::Gpu => "GPU",
        }
    }
}

#[derive(Debug, Default)]
pub struct AppHistoryTab {
    pub selected: usize,
    pub key: Option<String>,
    pub scroll: usize,
    pub sort: SortKeyState,
}

#[derive(Debug, Default)]
pub struct StartupTab {
    pub selected: usize,
    pub key: Option<String>,
    pub scroll: usize,
    pub filter: String,
}

#[derive(Debug, Default)]
pub struct UsersTab {
    pub selected: usize,
    pub key: Option<u32>,
    pub scroll: usize,
    pub panel: bool,
}

#[derive(Debug, Default)]
pub struct ServicesTab {
    pub selected: usize,
    pub key: Option<String>,
    pub scroll: usize,
    pub filter: String,
}

#[derive(Debug, Clone)]
pub enum PendingAction {
    EndTask(ProcKey),
    EndTree(ProcKey),
    ClearHistory,
    Service(String, ServiceAction),
    Startup(String, bool),
    Priority(ProcKey, i32),
}

#[derive(Debug, Clone)]
pub enum Dialog {
    Confirm {
        title: String,
        message: String,
        confirm_label: String,
        action: PendingAction,
    },
    Error {
        title: String,
        message: String,
    },
    Help,
    Run {
        input: String,
    },
    Priority {
        key: ProcKey,
        name: String,
        selected: usize,
    },
    Appearance {
        selected: usize,
    },
    Properties {
        title: String,
        lines: Vec<String>,
    },
}

impl Dialog {
    pub fn title(&self) -> &str {
        match self {
            Dialog::Confirm { title, .. } => title,
            Dialog::Error { title, .. } => title,
            Dialog::Help => "Keyboard shortcuts",
            Dialog::Run { .. } => "Run a new task",
            Dialog::Priority { .. } => "Set priority",
            Dialog::Appearance { .. } => "Appearance",
            Dialog::Properties { title, .. } => title,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItem {
    Run,
    Exit,
    Theme,
    SpeedHigh,
    SpeedNormal,
    SpeedLow,
    SpeedPaused,
    Refresh,
    ToggleDetails,
    OptionalColumns,
    Appearance,
    SortNext,
    SortReverse,
    ExpandAll,
    CollapseAll,
    EndTask,
    ClearHistory,
    Help,
}

pub const MENUS: [(&str, &[(&str, MenuItem)]); 4] = [
    (
        "File",
        &[("Run new task", MenuItem::Run), ("Exit", MenuItem::Exit)],
    ),
    (
        "Options",
        &[
            ("Dark mode", MenuItem::Theme),
            ("Update speed: High", MenuItem::SpeedHigh),
            ("Update speed: Normal", MenuItem::SpeedNormal),
            ("Update speed: Low", MenuItem::SpeedLow),
            ("Update speed: Paused", MenuItem::SpeedPaused),
        ],
    ),
    (
        "View",
        &[
            ("Refresh now", MenuItem::Refresh),
            ("Fewer details", MenuItem::ToggleDetails),
            ("Optional columns", MenuItem::OptionalColumns),
            ("Appearance...", MenuItem::Appearance),
            ("Sort by next column", MenuItem::SortNext),
            ("Reverse sort", MenuItem::SortReverse),
            ("Expand all", MenuItem::ExpandAll),
            ("Collapse all", MenuItem::CollapseAll),
            ("Clear app history", MenuItem::ClearHistory),
        ],
    ),
    ("Help", &[("Keyboard shortcuts", MenuItem::Help)]),
];

#[derive(Debug, Clone, Copy, Default)]
pub struct MenuState {
    pub menu: usize,
    pub item: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    Quit,
    Run {
        program: String,
        args: Vec<String>,
    },
    Signal {
        key: ProcKey,
        force: bool,
        tree: bool,
    },
    Service {
        name: String,
        action: ServiceAction,
    },
    Startup {
        name: String,
        enabled: bool,
    },
    ClearHistory,
    Priority {
        key: ProcKey,
        nice: i32,
    },
    Refresh,
    SetInterval(Option<std::time::Duration>),
    PersistSettings,
}

pub struct App {
    pub tab: Tab,
    pub theme_name: ThemeName,
    pub chart_style: ChartStyle,
    pub chart_color: ChartColor,
    pub ascii: bool,
    pub theme: Theme,
    pub settings: Settings,
    pub snapshot: Snapshot,
    pub graphs: GraphStore,
    pub dialog: Option<Dialog>,
    pub menu: Option<MenuState>,
    pub quit: bool,
    pub process: ProcessTab,
    pub details: DetailsTab,
    pub performance: PerformanceTab,
    pub app_history: AppHistoryTab,
    pub startup: StartupTab,
    pub users: UsersTab,
    pub services: ServicesTab,
    pub filter_editing: bool,
    pub notices: Vec<String>,
    pub last_sample_at: Option<std::time::Instant>,
    pub sample_count: u64,
}

impl App {
    pub fn new(settings: Settings, ascii_flag: bool) -> (Self, Vec<String>) {
        let mut notices = Vec::new();
        let ascii = ascii_flag || settings.ascii;
        let theme_name = ThemeName::parse(&settings.theme).unwrap_or(ThemeName::Light);
        let chart_style = ChartStyle::parse(&settings.chart_style).unwrap_or(ChartStyle::Filled);
        let chart_color = ChartColor::parse(&settings.chart_color).unwrap_or(ChartColor::Teal);
        let mut app = Self {
            tab: Tab::from_index(settings.last_tab),
            theme_name,
            chart_style,
            chart_color,
            ascii,
            theme: Theme::from_name(theme_name, ascii),
            settings,
            snapshot: Snapshot::default(),
            graphs: GraphStore::default(),
            dialog: None,
            menu: None,
            quit: false,
            process: ProcessTab::default(),
            details: DetailsTab::default(),
            performance: PerformanceTab::default(),
            app_history: AppHistoryTab::default(),
            startup: StartupTab::default(),
            users: UsersTab::default(),
            services: ServicesTab::default(),
            filter_editing: false,
            notices: Vec::new(),
            last_sample_at: None,
            sample_count: 0,
        };
        app.process.sort = SortKeyState {
            key: SortKey::Cpu,
            dir: SortDir::Desc,
        };
        app.details.sort = SortKeyState {
            key: SortKey::Cpu,
            dir: SortDir::Desc,
        };
        app.app_history.sort = SortKeyState {
            key: SortKey::Cpu,
            dir: SortDir::Desc,
        };
        app.process.more_details = true;
        app.process.show_optional_columns = true;
        notices.append(&mut app.notices);
        (app, notices)
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn set_theme(&mut self, name: ThemeName) {
        self.theme_name = name;
        self.theme = Theme::from_name(name, self.ascii);
        self.settings.theme = match name {
            ThemeName::Light => "light".to_string(),
            ThemeName::Dark => "dark".to_string(),
        };
    }

    pub fn toggle_theme(&mut self) {
        self.set_theme(self.theme_name.toggle());
    }

    pub fn set_chart_style(&mut self, style: ChartStyle) {
        self.chart_style = style;
        self.settings.chart_style = style.key().to_string();
    }

    pub fn set_chart_color(&mut self, color: ChartColor) {
        self.chart_color = color;
        self.settings.chart_color = color.key().to_string();
    }

    pub fn apply_snapshot(&mut self, snapshot: Snapshot) {
        self.snapshot = snapshot;
        self.last_sample_at = Some(std::time::Instant::now());
        self.sample_count += 1;
        self.graphs.push(&self.snapshot);
        self.process.refresh(&self.snapshot);
        self.sync_details();
        self.sync_services();
        self.sync_startup();
        self.sync_users();
        self.sync_history();
    }

    pub fn filter(&self) -> &str {
        match self.tab {
            Tab::Processes => &self.process.filter,
            Tab::Details => &self.details.filter,
            Tab::Services => &self.services.filter,
            Tab::Startup => &self.startup.filter,
            _ => "",
        }
    }

    pub fn filter_mut(&mut self) -> Option<&mut String> {
        match self.tab {
            Tab::Processes => Some(&mut self.process.filter),
            Tab::Details => Some(&mut self.details.filter),
            Tab::Services => Some(&mut self.services.filter),
            Tab::Startup => Some(&mut self.startup.filter),
            _ => None,
        }
    }

    pub fn filter_push(&mut self, ch: char) {
        if let Some(filter) = self.filter_mut() {
            filter.push(ch);
            self.refresh_after_filter();
        }
    }

    pub fn filter_backspace(&mut self) {
        if let Some(filter) = self.filter_mut() {
            filter.pop();
            self.refresh_after_filter();
        }
    }

    pub fn filter_clear(&mut self) {
        if let Some(filter) = self.filter_mut() {
            filter.clear();
            self.filter_editing = false;
            self.refresh_after_filter();
        }
    }

    fn refresh_after_filter(&mut self) {
        match self.tab {
            Tab::Processes => self.process.refresh(&self.snapshot),
            Tab::Details => self.sync_details(),
            Tab::Services => self.sync_services(),
            Tab::Startup => self.sync_startup(),
            _ => {}
        }
    }

    pub fn details_rows(&self) -> Vec<&ProcRow> {
        let needle = self.details.filter.trim().to_lowercase();
        let mut rows: Vec<&ProcRow> = self
            .snapshot
            .processes
            .iter()
            .filter(|row| needle.is_empty() || row.name.to_lowercase().contains(&needle))
            .collect();
        let (sort, dir) = (self.details.sort.key, self.details.sort.dir);
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
            ordering.then_with(|| a.key.pid.cmp(&b.key.pid))
        });
        rows
    }

    fn sync_details(&mut self) {
        let rows = self.details_rows();
        let len = rows.len();
        if len == 0 {
            self.details.selected = 0;
            self.details.key = None;
            return;
        }
        let found = self
            .details
            .key
            .and_then(|key| rows.iter().position(|row| row.key == key));
        let selected = found.unwrap_or(self.details.selected.min(len - 1));
        let key = rows[selected].key;
        drop(rows);
        self.details.selected = selected;
        self.details.key = Some(key);
    }

    pub fn service_rows(&self) -> Vec<&ServiceRow> {
        let needle = self.services.filter.trim().to_lowercase();
        self.snapshot
            .services
            .iter()
            .filter(|row| {
                needle.is_empty()
                    || row.name.to_lowercase().contains(&needle)
                    || row
                        .description
                        .as_deref()
                        .is_some_and(|d| d.to_lowercase().contains(&needle))
            })
            .collect()
    }

    fn sync_services(&mut self) {
        let rows = self.service_rows();
        let len = rows.len();
        if len == 0 {
            self.services.selected = 0;
            self.services.key = None;
            return;
        }
        let found = self
            .services
            .key
            .as_ref()
            .and_then(|key| rows.iter().position(|row| &row.name == key));
        let selected = found.unwrap_or(self.services.selected.min(len - 1));
        let key = rows[selected].name.clone();
        drop(rows);
        self.services.selected = selected;
        self.services.key = Some(key);
    }

    pub fn startup_rows(&self) -> Vec<&StartupRow> {
        let needle = self.startup.filter.trim().to_lowercase();
        self.snapshot
            .startup
            .iter()
            .filter(|row| needle.is_empty() || row.name.to_lowercase().contains(&needle))
            .collect()
    }

    fn sync_startup(&mut self) {
        let rows = self.startup_rows();
        let len = rows.len();
        if len == 0 {
            self.startup.selected = 0;
            self.startup.key = None;
            return;
        }
        let found = self
            .startup
            .key
            .as_ref()
            .and_then(|key| rows.iter().position(|row| &row.name == key));
        let selected = found.unwrap_or(self.startup.selected.min(len - 1));
        let key = rows[selected].name.clone();
        drop(rows);
        self.startup.selected = selected;
        self.startup.key = Some(key);
    }

    pub fn user_rows(&self) -> Vec<&SessionRow> {
        self.snapshot.sessions.iter().collect()
    }

    fn sync_users(&mut self) {
        let rows = self.user_rows();
        let len = rows.len();
        if len == 0 {
            self.users.selected = 0;
            self.users.key = None;
            return;
        }
        let found = self
            .users
            .key
            .and_then(|key| rows.iter().position(|row| row.uid == key));
        let selected = found.unwrap_or(self.users.selected.min(len - 1));
        let key = rows[selected].uid;
        drop(rows);
        self.users.selected = selected;
        self.users.key = Some(key);
    }

    pub fn history_rows(&self) -> Vec<&AppUsage> {
        let (sort, dir) = (self.app_history.sort.key, self.app_history.sort.dir);
        let mut rows: Vec<&AppUsage> = self.snapshot.app_usage.iter().collect();
        rows.sort_by(|a, b| {
            let ordering = match sort {
                SortKey::Name => a.name.cmp(&b.name),
                SortKey::Memory => a.peak_memory.cmp(&b.peak_memory),
                SortKey::Disk => {
                    (a.disk_read + a.disk_written).cmp(&(b.disk_read + b.disk_written))
                }
                _ => a.cpu_ms.cmp(&b.cpu_ms),
            };
            let ordering = match dir {
                SortDir::Asc => ordering,
                SortDir::Desc => ordering.reverse(),
            };
            ordering.then_with(|| a.name.cmp(&b.name))
        });
        rows
    }

    fn sync_history(&mut self) {
        let rows = self.history_rows();
        let len = rows.len();
        if len == 0 {
            self.app_history.selected = 0;
            self.app_history.key = None;
            return;
        }
        let found = self
            .app_history
            .key
            .as_ref()
            .and_then(|key| rows.iter().position(|row| &row.name == key));
        let selected = found.unwrap_or(self.app_history.selected.min(len - 1));
        let key = rows[selected].name.clone();
        drop(rows);
        self.app_history.selected = selected;
        self.app_history.key = Some(key);
    }

    pub fn selected_process(&self) -> Option<ProcRow> {
        match self.tab {
            Tab::Details | Tab::Performance => {
                let key = self.details.key?;
                self.snapshot.process(key).cloned()
            }
            Tab::Processes => match self.process.selected_key()? {
                ProcSel::Proc(key) => self.snapshot.process(key).cloned(),
                ProcSel::Kind(_) => None,
                ProcSel::Group(kind, name) => self
                    .snapshot
                    .processes
                    .iter()
                    .find(|p| p.kind == kind && p.name == name)
                    .cloned(),
            },
            _ => None,
        }
    }

    pub fn selected_service(&self) -> Option<ServiceRow> {
        let rows = self.service_rows();
        rows.get(self.services.selected).map(|row| (*row).clone())
    }

    pub fn selected_startup(&self) -> Option<StartupRow> {
        let rows = self.startup_rows();
        rows.get(self.startup.selected).map(|row| (*row).clone())
    }

    pub fn selected_usage(&self) -> Option<AppUsage> {
        let rows = self.history_rows();
        rows.get(self.app_history.selected)
            .map(|row| (*row).clone())
    }

    pub fn selected_session(&self) -> Option<SessionRow> {
        let rows = self.user_rows();
        rows.get(self.users.selected).map(|row| (*row).clone())
    }

    /// Move the selection on the active tab. Returns false at the edges.
    pub fn move_selection(&mut self, delta: isize) -> bool {
        match self.tab {
            Tab::Processes => {
                let len = self.process.items.len();
                if move_index(&mut self.process.selected, len, delta) {
                    // Track the new row, otherwise the next sample restores the
                    // selection to the row the user just left.
                    self.process.key = self
                        .process
                        .items
                        .get(self.process.selected)
                        .map(ProcItem::sel);
                    self.process.mark_navigation();
                    true
                } else {
                    false
                }
            }
            Tab::Details => {
                let len = self.details_rows().len();
                if move_index(&mut self.details.selected, len, delta) {
                    if let Some(row) = self.details_rows().get(self.details.selected) {
                        self.details.key = Some(row.key);
                    }
                    true
                } else {
                    false
                }
            }
            Tab::Services => {
                let len = self.service_rows().len();
                if move_index(&mut self.services.selected, len, delta) {
                    if let Some(row) = self.service_rows().get(self.services.selected) {
                        self.services.key = Some(row.name.clone());
                    }
                    true
                } else {
                    false
                }
            }
            Tab::Startup => {
                let len = self.startup_rows().len();
                if move_index(&mut self.startup.selected, len, delta) {
                    if let Some(row) = self.startup_rows().get(self.startup.selected) {
                        self.startup.key = Some(row.name.clone());
                    }
                    true
                } else {
                    false
                }
            }
            Tab::Users => {
                let len = self.user_rows().len();
                if move_index(&mut self.users.selected, len, delta) {
                    if let Some(row) = self.user_rows().get(self.users.selected) {
                        self.users.key = Some(row.uid);
                    }
                    true
                } else {
                    false
                }
            }
            Tab::AppHistory => {
                let len = self.history_rows().len();
                if move_index(&mut self.app_history.selected, len, delta) {
                    if let Some(row) = self.history_rows().get(self.app_history.selected) {
                        self.app_history.key = Some(row.name.clone());
                    }
                    true
                } else {
                    false
                }
            }
            Tab::Performance => false,
        }
    }

    pub fn show_optional(&self) -> bool {
        self.process.show_optional_columns
    }

    pub fn toggle_expand(&mut self) {
        if self.tab != Tab::Processes {
            return;
        }
        let Some(key) = self.process.selected_key() else {
            return;
        };
        self.expand_key(key);
    }

    pub fn expand_key(&mut self, key: ProcSel) {
        match &key {
            ProcSel::Kind(kind) => {
                if !self.process.collapsed_kinds.remove(kind) {
                    self.process.collapsed_kinds.insert(*kind);
                }
            }
            ProcSel::Group(kind, name) => {
                let group = (*kind, name.clone());
                if !self.process.expanded_groups.remove(&group) {
                    self.process.expanded_groups.insert(group);
                }
            }
            ProcSel::Proc(_) => return,
        }
        self.process.refresh(&self.snapshot);
        self.process.key = Some(match self.process.current() {
            Some(item) => item.sel(),
            None => return,
        });
        if let Some(index) = self
            .process
            .items
            .iter()
            .position(|item| Some(item.sel()) == self.process.key)
        {
            self.process.selected = index;
        }
    }

    pub fn collapse_all(&mut self) {
        let mut kinds = HashSet::new();
        for item in &self.process.items {
            if let ProcItem::Header { kind, .. } = item {
                kinds.insert(*kind);
            }
        }
        self.process.collapsed_kinds = kinds;
        self.process.expanded_groups.clear();
        self.process.refresh(&self.snapshot);
    }

    pub fn expand_all(&mut self) {
        self.process.collapsed_kinds.clear();
        self.process.expanded_groups.clear();
        self.process.refresh(&self.snapshot);
    }

    pub fn sync_details_public(&mut self) {
        self.sync_details();
    }

    pub fn sync_services_public(&mut self) {
        self.sync_services();
    }

    pub fn sync_startup_public(&mut self) {
        self.sync_startup();
    }

    pub fn sync_users_public(&mut self) {
        self.sync_users();
    }

    pub fn sync_history_public(&mut self) {
        self.sync_history();
    }

    pub fn notice(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.notices.retain(|n| n != &message);
        self.notices.push(message);
        if self.notices.len() > 4 {
            self.notices.remove(0);
        }
    }
}

fn move_index(selected: &mut usize, len: usize, delta: isize) -> bool {
    if len == 0 {
        return false;
    }
    let next = (*selected as isize + delta).clamp(0, len as isize - 1) as usize;
    let moved = next != *selected;
    *selected = next;
    moved
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> App {
        let (mut app, _) = App::new(Settings::default(), false);
        app.apply_snapshot(Snapshot::fixture());
        app
    }

    #[test]
    fn tabs_cycle_and_wrap() {
        assert_eq!(Tab::Processes.next(), Tab::Performance);
        assert_eq!(Tab::Services.next(), Tab::Processes);
        assert_eq!(Tab::Processes.previous(), Tab::Services);
        assert_eq!(Tab::from_index(3), Tab::Startup);
        assert_eq!(Tab::Startup.index(), 3);
    }

    #[test]
    fn process_selection_survives_reordering() {
        let mut app = app();
        // Select the second group, then make it the top CPU consumer.
        app.process.selected = 1;
        app.process.key = Some(app.process.items[1].sel());
        let key = app.process.key.clone().unwrap();
        let mut snapshot = app.snapshot.clone();
        if let ProcSel::Group(kind, name) = &key {
            for process in &mut snapshot.processes {
                if process.kind == *kind && &process.name == name {
                    process.cpu = 99.0;
                }
            }
        }
        app.apply_snapshot(snapshot);
        assert_eq!(
            app.process.items[app.process.selected].sel(),
            key,
            "selection follows the process, not the index"
        );
    }

    #[test]
    fn process_selection_moves_when_key_disappears() {
        let mut app = app();
        app.process.selected = 2;
        app.process.key = Some(app.process.items[2].sel());
        let mut snapshot = app.snapshot.clone();
        snapshot
            .processes
            .retain(|p| p.key.pid != 1 && p.key.pid != 500);
        app.apply_snapshot(snapshot);
        assert!(!app.process.items.is_empty());
        assert!(app.process.selected < app.process.items.len());
        assert_eq!(
            app.process.key,
            Some(app.process.items[app.process.selected].sel())
        );
    }

    #[test]
    fn a_sample_does_not_undo_a_move() {
        let mut app = app();
        app.move_selection(1);
        let key = app.process.items[app.process.selected].sel();
        let mut snapshot = app.snapshot.clone();
        for process in &mut snapshot.processes {
            if process.name == "node" {
                process.cpu = 99.0;
            }
        }
        app.apply_snapshot(snapshot);
        assert_eq!(app.process.items[app.process.selected].sel(), key);
    }

    #[test]
    fn navigation_freezes_the_order_until_the_pause_ends() {
        let mut app = app();
        app.move_selection(1);
        let before: Vec<_> = app.process.items.iter().map(|item| item.sel()).collect();
        let mut snapshot = app.snapshot.clone();
        for process in &mut snapshot.processes {
            if process.name == "postgres" {
                process.cpu = 99.0;
            }
        }
        app.apply_snapshot(snapshot.clone());
        let during: Vec<_> = app.process.items.iter().map(|item| item.sel()).collect();
        assert_eq!(before, during, "order holds while navigating");
        app.process.refresh_with(&snapshot, false);
        let after: Vec<_> = app.process.items.iter().map(|item| item.sel()).collect();
        assert_ne!(during, after, "it re-sorts after the pause");
    }

    #[test]
    fn collapse_all_leaves_only_headers() {
        let mut app = app();
        app.collapse_all();
        assert!(
            app.process
                .items
                .iter()
                .all(|item| matches!(item, ProcItem::Header { .. })),
            "only kind headers remain"
        );
        app.expand_all();
        assert!(app.process.items.len() > 1);
    }

    #[test]
    fn collapsing_and_expanding_a_group_changes_the_row_count() {
        let mut app = app();
        let key = app
            .process
            .items
            .iter()
            .find(|item| matches!(item, ProcItem::Group { .. }))
            .map(ProcItem::sel)
            .unwrap();
        let collapsed_len = app.process.items.len();
        app.expand_key(key.clone());
        assert!(app.process.items.len() > collapsed_len, "expanded");
        app.expand_key(key.clone());
        assert_eq!(app.process.items.len(), collapsed_len, "collapsed again");
        assert!(app.process.items.iter().any(|item| item.sel() == key));
    }

    #[test]
    fn details_selection_follows_a_sort_change() {
        let mut app = app();
        app.tab = Tab::Details;
        app.details.key = Some(app.details_rows()[2].key);
        app.details.sort.key = SortKey::Name;
        app.details.sort.dir = SortDir::Asc;
        app.sync_details();
        let rows = app.details_rows();
        assert_eq!(rows[app.details.selected].key, app.details.key.unwrap());
    }

    #[test]
    fn services_filter_narrows_rows() {
        let mut app = app();
        app.tab = Tab::Services;
        for ch in "nginx".chars() {
            app.filter_push(ch);
        }
        assert_eq!(app.service_rows().len(), 1);
        app.filter_clear();
        assert_eq!(app.service_rows().len(), 2);
    }

    #[test]
    fn sessions_can_appear_and_disappear() {
        let mut app = app();
        app.tab = Tab::Users;
        let mut snapshot = app.snapshot.clone();
        let mut second = snapshot.sessions[0].clone();
        second.user = "guest".into();
        second.uid = 502;
        second.tty = "ttys003".into();
        snapshot.sessions.push(second);
        app.apply_snapshot(snapshot);
        assert_eq!(app.user_rows().len(), 2);
        app.users.selected = 1;
        app.users.key = Some(502);
        let mut snapshot = app.snapshot.clone();
        snapshot.sessions.retain(|row| row.uid != 502);
        app.apply_snapshot(snapshot);
        assert_eq!(app.user_rows().len(), 1);
        assert_eq!(app.users.selected, 0);
        assert_eq!(app.users.key, Some(501));
    }

    #[test]
    fn theme_toggle_updates_settings() {
        let mut app = app();
        assert_eq!(app.theme_name, ThemeName::Light);
        app.toggle_theme();
        assert_eq!(app.theme_name, ThemeName::Dark);
        assert_eq!(app.settings.theme, "dark");
    }

    #[test]
    fn filter_narrows_process_rows() {
        let mut app = app();
        app.tab = Tab::Processes;
        for ch in "saf".chars() {
            app.filter_push(ch);
        }
        let groups: Vec<_> = app
            .process
            .items
            .iter()
            .filter_map(|item| match item {
                ProcItem::Group { name, .. } => Some(name.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(groups, vec!["Safari"]);
        app.filter_clear();
        assert!(app.process.items.len() > 1);
    }
}
