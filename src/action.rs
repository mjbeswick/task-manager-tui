//! Actions to effects. The interface never signals a process itself; it returns
//! work for the main loop, which keeps this layer testable.

use crate::app::{App, Dialog, Effect, MENUS, MenuState, PendingAction, Tab};
use crate::input::Action;
use crate::model::{Kind, ProcKey, ProcRow, ProcSel, Signal, Status};
use crate::platform::{ElevatedAction, ServiceAction};
use crate::sample::group::{SortDir, SortKey};

impl App {
    pub fn dispatch(&mut self, action: Action) -> Vec<Effect> {
        let mut effects = Vec::new();

        if self.dialog.is_some() {
            return self.dispatch_dialog(action);
        }
        if self.menu.is_some() {
            return self.dispatch_menu(action);
        }
        if self.filter_editing {
            match action {
                Action::FilterChar(ch) => self.filter_push(ch),
                Action::FilterBackspace => self.filter_backspace(),
                Action::FilterClear => self.filter_clear(),
                Action::FilterDone => self.filter_editing = false,
                Action::Cancel => self.filter_clear(),
                _ => {}
            }
            return effects;
        }

        match action {
            Action::Quit => {
                self.quit = true;
                effects.push(Effect::Quit);
            }
            Action::NextTab => self.switch_tab(self.tab.next()),
            Action::PreviousTab => self.switch_tab(self.tab.previous()),
            Action::GoTab(index) => self.switch_tab(Tab::from_index(index)),
            Action::MoveUp => {
                self.move_selection(-1);
            }
            Action::MoveDown => {
                self.move_selection(1);
            }
            Action::PageUp => {
                self.move_selection(-10);
            }
            Action::PageDown => {
                self.move_selection(10);
            }
            Action::Top => {
                self.move_selection(-100_000);
            }
            Action::Bottom => {
                self.move_selection(100_000);
            }
            Action::Expand => match self.tab {
                Tab::Processes => self.toggle_expand(),
                _ => self.inspect(),
            },
            Action::Collapse => {
                if self.tab == Tab::Processes {
                    if let Some(ProcSel::Group(_, _)) = self.process.selected_key() {
                        self.toggle_expand();
                    } else {
                        self.move_selection(-1);
                    }
                }
            }
            Action::SortNext => self.cycle_sort(true),
            Action::SortPrevious => self.cycle_sort(false),
            Action::SortFlip => self.flip_sort(),
            Action::ToggleDetails => {
                if self.tab == Tab::Processes {
                    self.process.more_details = !self.process.more_details;
                }
            }
            Action::ToggleOptionalColumns => {
                self.process.show_optional_columns = !self.process.show_optional_columns;
            }
            Action::ToggleHints => {
                self.toggle_hints();
                effects.push(Effect::PersistSettings);
            }
            Action::ToggleTheme => {
                self.toggle_theme();
                effects.push(Effect::PersistSettings);
            }
            Action::Appearance => {
                self.dialog = Some(Dialog::Appearance { selected: 0 });
            }
            Action::SpeedFaster => {
                let next = self.settings.faster_speed();
                self.set_speed(next, &mut effects);
            }
            Action::SpeedSlower => {
                let next = self.settings.next_speed();
                self.set_speed(next, &mut effects);
            }
            Action::Refresh => effects.push(Effect::Refresh),
            Action::FilterStart => {
                if self.filter_mut().is_some() {
                    self.filter_editing = true;
                }
            }
            Action::FilterChar(_)
            | Action::FilterBackspace
            | Action::FilterClear
            | Action::FilterDone => {}
            Action::EndTask | Action::EndTree => {
                if self.tab == Tab::AppHistory {
                    if action == Action::EndTask {
                        self.end_selected_app();
                    }
                    return effects;
                }
                let tree = action == Action::EndTree;
                if let Some(row) = self.selected_process() {
                    effects.extend(self.confirm_end_task(&row, tree));
                } else {
                    self.notice("No process is selected on this tab.");
                }
            }
            Action::ToggleStartup => {
                if let Some(row) = self.selected_startup() {
                    effects.push(Effect::Startup {
                        name: row.name.clone(),
                        enabled: !row.enabled,
                    });
                }
            }
            Action::ServiceStart | Action::ServiceStop | Action::ServiceRestart => {
                let service_action = match action {
                    Action::ServiceStart => ServiceAction::Start,
                    Action::ServiceStop => ServiceAction::Stop,
                    _ => ServiceAction::Restart,
                };
                if let Some(row) = self.selected_service() {
                    let needs_admin =
                        row.scope == crate::model::Scope::System && !crate::platform::is_root();
                    if needs_admin {
                        let elevated = ElevatedAction::Service {
                            name: row.name.clone(),
                            scope: row.scope,
                            action: service_action,
                        };
                        self.dialog = Some(Dialog::Confirm {
                            title: format!("{} service", service_action.label()),
                            message: format!(
                                "{} {}? This system service needs administrator rights.",
                                service_action.label(),
                                row.name
                            ),
                            confirm_label: format!(
                                "Elevate and {}",
                                service_action.label().to_lowercase()
                            ),
                            action: PendingAction::Elevate(elevated),
                        });
                    } else if service_action == ServiceAction::Start {
                        effects.push(Effect::Service {
                            name: row.name.clone(),
                            action: service_action,
                        });
                    } else {
                        self.dialog = Some(Dialog::Confirm {
                            title: format!("{} service", service_action.label()),
                            message: format!(
                                "{} {}? Services that depend on it may also stop or restart.",
                                service_action.label(),
                                row.name
                            ),
                            confirm_label: service_action.label().to_string(),
                            action: PendingAction::Service(row.name.clone(), service_action),
                        });
                    }
                } else {
                    self.notice("No service is selected.");
                }
            }
            Action::Inspect => self.inspect(),
            Action::SetPriority => self.open_priority_dialog(),
            Action::OpenSignal => self.open_signal_dialog(),
            Action::ClearUsageEntry => {
                if self.tab == Tab::AppHistory {
                    self.confirm_clear_usage();
                }
            }
            Action::SetAffinity => {
                self.notice("Set affinity is not supported on Linux or macOS.");
            }
            Action::ShowProperties => self.show_properties(),
            Action::Help => self.dialog = Some(Dialog::Help),
            Action::DeleteHistory => {
                self.dialog = Some(Dialog::Confirm {
                    title: "Delete usage history".to_string(),
                    message: "Every application's accumulated CPU time, peak memory, and disk totals will be cleared."
                        .to_string(),
                    confirm_label: "Delete usage history".to_string(),
                    action: PendingAction::ClearHistory,
                });
            }
            Action::MenuOpen => self.menu = Some(MenuState::default()),
            Action::OpenMenu(index) => {
                self.menu = Some(MenuState {
                    menu: index.min(MENUS.len() - 1),
                    item: 0,
                })
            }
            Action::MenuClose
            | Action::MenuFirst
            | Action::MenuLast
            | Action::MenuLeft
            | Action::MenuRight
            | Action::MenuUp
            | Action::MenuDown
            | Action::MenuActivate => {}
            Action::Cancel => {
                if !self.filter().is_empty() {
                    self.filter_clear();
                }
            }
            Action::CycleBack
            | Action::Confirm
            | Action::RunSubmit
            | Action::RunChar(_)
            | Action::RunBackspace
            | Action::ToggleSignalTree
            | Action::PriorityChar(_)
            | Action::PriorityBackspace
            | Action::PriorityStep(_)
            | Action::PasswordChar(_)
            | Action::PasswordBackspace
            | Action::PasswordSubmit => {}
        }
        effects
    }

    /// A left click, which first dismisses an open menu unless the click is
    /// on the menu itself.
    pub fn mouse_click(&mut self, target: Option<crate::ui::hits::Target>) -> Vec<Effect> {
        if self.menu.is_some()
            && !matches!(
                target,
                Some(crate::ui::hits::Target::Menu(_))
                    | Some(crate::ui::hits::Target::MenuItem(..))
                    | Some(crate::ui::hits::Target::MenuPanel)
            )
        {
            self.menu = None;
            return Vec::new();
        }
        match target {
            Some(target) => self.click(target),
            None => Vec::new(),
        }
    }

    /// Handle a click or tap on a region recorded during the last frame.
    pub fn click(&mut self, target: crate::ui::hits::Target) -> Vec<Effect> {
        use crate::ui::hits::Target;
        match target {
            Target::Tab(tab) => {
                self.switch_tab(tab);
                Vec::new()
            }
            Target::Menu(index) => {
                self.menu = Some(MenuState {
                    menu: index.min(MENUS.len() - 1),
                    item: 0,
                });
                Vec::new()
            }
            Target::MenuItem(menu, item) => {
                self.menu = Some(MenuState { menu, item });
                self.dispatch(Action::MenuActivate)
            }
            Target::Row(index) => {
                self.select_row(index);
                Vec::new()
            }
            Target::Resource(resource) => {
                self.performance.resource = resource;
                Vec::new()
            }
            Target::DialogConfirm => self.dispatch(Action::Confirm),
            Target::DialogCancel | Target::DialogClose => self.dispatch(Action::Cancel),
            Target::DialogSignal(index) => {
                if let Some(Dialog::Signal { selected, .. }) = &mut self.dialog {
                    *selected = index.min(Signal::ALL.len() - 1);
                }
                Vec::new()
            }
            Target::AppearanceRow(index) => {
                if let Some(Dialog::Appearance { selected }) = &mut self.dialog {
                    *selected = index.min(3);
                }
                Vec::new()
            }
            Target::Header(key) => {
                self.sort_by(key);
                Vec::new()
            }
            Target::FilterField => self.dispatch(Action::FilterStart),
            Target::FilterClear => {
                self.filter_clear();
                Vec::new()
            }
            Target::MenuPanel
            | Target::ScrollZone(_)
            | Target::ScrollbarTrack(_)
            | Target::ScrollbarThumb(_) => Vec::new(),
        }
    }

    /// Sort by `key`, or reverse when it is already the sort column. Text
    /// columns start A to Z, figures start highest first.
    fn sort_by(&mut self, key: SortKey) {
        let state = match self.tab {
            Tab::Processes => &mut self.process.sort,
            Tab::Details => &mut self.details.sort,
            Tab::AppHistory => &mut self.app_history.sort,
            _ => return,
        };
        if state.key == key {
            state.flip();
        } else {
            state.key = key;
            state.dir = match key {
                SortKey::Name | SortKey::Status | SortKey::User | SortKey::Pid => SortDir::Asc,
                _ => SortDir::Desc,
            };
        }
        self.refresh_active();
    }

    /// Select a row by its visible index on the active tab.
    pub fn select_row(&mut self, index: usize) {
        match self.tab {
            Tab::Processes => {
                if let Some(item) = self.process.items.get(index) {
                    self.process.selected = index;
                    self.process.key = Some(item.sel());
                }
            }
            Tab::Details => {
                let key = self.details_rows().get(index).map(|row| row.key);
                if let Some(key) = key {
                    self.details.selected = index;
                    self.details.key = Some(key);
                }
            }
            Tab::Services => {
                let key = self.service_rows().get(index).map(|row| row.name.clone());
                if let Some(key) = key {
                    self.services.selected = index;
                    self.services.key = Some(key);
                }
            }
            Tab::Startup => {
                let key = self.startup_rows().get(index).map(|row| row.name.clone());
                if let Some(key) = key {
                    self.startup.selected = index;
                    self.startup.key = Some(key);
                }
            }
            Tab::Users => {
                let key = self.user_rows().get(index).map(|row| row.uid);
                if let Some(key) = key {
                    self.users.selected = index;
                    self.users.key = Some(key);
                }
            }
            Tab::AppHistory => {
                let key = self.history_rows().get(index).map(|row| row.name.clone());
                if let Some(key) = key {
                    self.app_history.selected = index;
                    self.app_history.key = Some(key);
                }
            }
            Tab::Performance => {}
        }
    }

    fn switch_tab(&mut self, tab: Tab) {
        self.tab = tab;
        self.filter_editing = false;
        self.settings.last_tab = tab.index();
        match tab {
            Tab::Processes => self.process.refresh(&self.snapshot),
            Tab::Details => self.sync_details_public(),
            Tab::Services => self.sync_services_public(),
            Tab::Startup => self.sync_startup_public(),
            Tab::Users => self.sync_users_public(),
            Tab::AppHistory => self.sync_history_public(),
            _ => {}
        }
    }

    fn set_speed(&mut self, speed: &str, effects: &mut Vec<Effect>) {
        self.settings.set_speed(speed);
        effects.push(Effect::SetInterval(self.settings.interval()));
        effects.push(Effect::PersistSettings);
    }

    fn sorted_keys(&self) -> Vec<SortKey> {
        match self.tab {
            Tab::Processes => SortKey::PROCESS_KEYS.to_vec(),
            Tab::Details => SortKey::DETAIL_KEYS.to_vec(),
            Tab::AppHistory => vec![SortKey::Name, SortKey::Cpu, SortKey::Memory, SortKey::Disk],
            _ => Vec::new(),
        }
    }

    fn cycle_sort(&mut self, forward: bool) {
        let keys = self.sorted_keys();
        if keys.is_empty() {
            return;
        }
        let state = match self.tab {
            Tab::Processes => &mut self.process.sort,
            Tab::Details => &mut self.details.sort,
            Tab::AppHistory => &mut self.app_history.sort,
            _ => return,
        };
        if forward {
            state.next_column(&keys);
        } else {
            state.previous_column(&keys);
        }
        self.refresh_active();
    }

    fn flip_sort(&mut self) {
        let state = match self.tab {
            Tab::Processes => &mut self.process.sort,
            Tab::Details => &mut self.details.sort,
            Tab::AppHistory => &mut self.app_history.sort,
            _ => return,
        };
        state.flip();
        self.refresh_active();
    }

    fn refresh_active(&mut self) {
        match self.tab {
            Tab::Processes => self.process.refresh(&self.snapshot),
            Tab::Details => self.sync_details_public(),
            Tab::AppHistory => self.sync_history_public(),
            _ => {}
        }
    }

    fn confirm_clear_usage(&mut self) {
        let Some(row) = self.selected_usage() else {
            self.notice("Select an application first.");
            return;
        };
        self.dialog = Some(Dialog::Confirm {
            title: "Clear usage history".to_string(),
            message: format!(
                "Clear {}'s accumulated CPU time, peak memory, and disk totals? This cannot be undone.",
                row.name
            ),
            confirm_label: "Clear entry".to_string(),
            action: PendingAction::ClearUsage(row.name),
        });
    }

    fn end_selected_app(&mut self) {
        let Some(row) = self.selected_usage() else {
            self.notice("Select an application first.");
            return;
        };
        let count = self.app_processes(&row.name).len();
        if count == 0 {
            self.notice(format!("{} has no running processes.", row.name));
            return;
        }
        self.dialog = Some(Dialog::Confirm {
            title: "End app".to_string(),
            message: format!(
                "End {}'s {count} running process{}? Unsaved work will be lost.",
                row.name,
                if count == 1 { "" } else { "es" }
            ),
            confirm_label: "End app".to_string(),
            action: PendingAction::EndApp(row.name),
        });
    }

    fn app_processes(&self, name: &str) -> Vec<ProcRow> {
        self.snapshot
            .processes
            .iter()
            .filter(|process| process.kind == Kind::App && process.name == name)
            .cloned()
            .collect()
    }

    fn open_priority_dialog(&mut self) {
        let Some(row) = self.selected_process() else {
            self.notice("Select a process row first.");
            return;
        };
        self.dialog = Some(Dialog::Priority {
            key: row.key,
            name: row.name,
            input: row.priority.nice.to_string(),
            editing: false,
            error: false,
        });
    }

    fn open_signal_dialog(&mut self) {
        let Some(row) = self.signal_target() else {
            self.notice("Select a process row first.");
            return;
        };
        self.dialog = Some(Dialog::Signal {
            key: row.key,
            name: row.name,
            pid: row.key.pid,
            selected: 0,
            tree: false,
        });
    }

    /// The selected process, or the first process under a selected kind heading.
    fn signal_target(&self) -> Option<ProcRow> {
        if let Some(row) = self.selected_process() {
            return Some(row);
        }
        let ProcSel::Kind(kind) = self.process.selected_key()? else {
            return None;
        };
        self.snapshot
            .processes
            .iter()
            .find(|process| process.kind == kind)
            .cloned()
    }

    fn signal_pids(&self, key: ProcKey, tree: bool) -> Vec<u32> {
        let mut pids = vec![key.pid];
        if tree {
            pids.extend(
                descendants(&self.snapshot.processes, key.pid)
                    .iter()
                    .map(|key| key.pid),
            );
        }
        pids
    }

    fn submit_signal(&mut self, key: ProcKey, signal: Signal, tree: bool) -> Vec<Effect> {
        let mut effects = Vec::new();
        let row = self.snapshot.process(key);
        let privileged = !crate::platform::is_root()
            && row.is_some_and(|row| {
                row.kind == Kind::System || row.uid != Some(crate::platform::current_uid())
            });
        if privileged {
            let name = row
                .map(|r| r.name.clone())
                .unwrap_or_else(|| format!("PID {}", key.pid));
            let tree_note = if tree { " and its process tree" } else { "" };
            let elevated = ElevatedAction::Signal {
                pids: self.signal_pids(key, tree),
                signal,
            };
            self.dialog = Some(Dialog::Confirm {
                title: format!("Send SIG{}", signal.name()),
                message: format!(
                    "Send SIG{} to {name} (PID {}){tree_note}? This process needs administrator rights.",
                    signal.name(),
                    key.pid
                ),
                confirm_label: format!("Elevate and send SIG{}", signal.name()),
                action: PendingAction::Elevate(elevated),
            });
            return effects;
        }
        if signal.requires_confirm() {
            let name = row
                .map(|r| r.name.clone())
                .unwrap_or_else(|| format!("PID {}", key.pid));
            let tree_note = if tree { " and its process tree" } else { "" };
            self.dialog = Some(Dialog::Confirm {
                title: format!("Send SIG{}", signal.name()),
                message: format!(
                    "Send SIG{} to {name} (PID {}){tree_note}? {}",
                    signal.name(),
                    key.pid,
                    if signal == Signal::Stop {
                        "Tracking continues until SIGCONT."
                    } else {
                        "Unsaved work may be lost."
                    }
                ),
                confirm_label: format!("Send SIG{}", signal.name()),
                action: PendingAction::Signal(key, signal, tree),
            });
        } else {
            effects.push(Effect::Signal { key, signal, tree });
        }
        effects
    }

    fn inspect(&mut self) {
        match self.tab {
            Tab::Processes => self.open_process_info(),
            Tab::Details => {
                self.details.panel = !self.details.panel;
            }
            Tab::Users => {
                self.users.panel = !self.users.panel;
            }
            _ => self.show_properties(),
        }
    }

    pub(crate) fn open_process_info(&mut self) {
        let Some(ProcSel::Proc(key)) = self.process.selected_key() else {
            self.notice("Select a process row for details.");
            return;
        };
        let Some(row) = self.snapshot.process(key) else {
            self.notice("That process has exited.");
            return;
        };
        self.dialog = Some(Dialog::ProcessInfo {
            key,
            name: row.name.clone(),
        });
    }

    fn show_properties(&mut self) {
        let lines = match self.tab {
            Tab::Services => self.selected_service().map(|row| {
                vec![
                    format!("Name: {}", row.name),
                    format!(
                        "Description: {}",
                        row.description.unwrap_or_else(|| "-".into())
                    ),
                    format!("Status: {}", row.state.label()),
                    format!(
                        "PID: {}",
                        row.pid.map(|p| p.to_string()).unwrap_or_else(|| "-".into())
                    ),
                    format!("Group: {}", row.scope.label()),
                ]
            }),
            Tab::Startup => self.selected_startup().map(|row| {
                vec![
                    format!("Name: {}", row.name),
                    format!("Command: {}", row.command),
                    format!("Owner: {}", row.owner),
                    format!("Source: {}", row.source),
                    format!(
                        "Status: {}",
                        if row.enabled { "Enabled" } else { "Disabled" }
                    ),
                    format!("Startup impact: {}", row.impact.label()),
                    format!(
                        "Program present: {}",
                        if row.missing_program { "no" } else { "yes" }
                    ),
                ]
            }),
            Tab::Details => self.selected_process().map(|row| process_lines(&row)),
            _ => self.selected_process().map(|row| process_lines(&row)),
        };
        if let Some(lines) = lines {
            self.dialog = Some(Dialog::Properties {
                title: "Properties".to_string(),
                lines,
            });
        }
    }

    fn confirm_end_task(&mut self, row: &ProcRow, tree: bool) -> Vec<Effect> {
        let mut effects = Vec::new();
        if row.status == Status::Stopped {
            self.notice("That process has already exited.");
            return effects;
        }
        let key = row.key;
        let privileged = !crate::platform::is_root()
            && (row.kind == Kind::System || row.uid != Some(crate::platform::current_uid()));
        let signal = if tree { Signal::Kill } else { Signal::Term };
        if privileged {
            let count = descendants(&self.snapshot.processes, key.pid).len() + 1;
            let elevated = ElevatedAction::Signal {
                pids: self.signal_pids(key, tree),
                signal,
            };
            self.dialog = Some(Dialog::Confirm {
                title: if tree { "End process tree" } else { "End task" }.to_string(),
                message: format!(
                    "End {}{}? This needs administrator rights.",
                    row.name,
                    if tree {
                        format!(
                            " and its {count} process{}",
                            if count == 1 { "" } else { "es" }
                        )
                    } else {
                        format!(" (PID {})", key.pid)
                    }
                ),
                confirm_label: "Elevate and end".to_string(),
                action: PendingAction::Elevate(elevated),
            });
            return effects;
        }
        if tree {
            let count = descendants(&self.snapshot.processes, key.pid).len() + 1;
            self.dialog = Some(Dialog::Confirm {
                title: "End process tree".to_string(),
                message: format!(
                    "End {} and its {count} process{}? Unsaved work will be lost.",
                    row.name,
                    if count == 1 { "" } else { "es" }
                ),
                confirm_label: "End process tree".to_string(),
                action: PendingAction::EndTree(key),
            });
            return effects;
        }
        effects.push(Effect::Signal {
            key,
            signal: Signal::Term,
            tree: false,
        });
        effects
    }

    fn dispatch_dialog(&mut self, action: Action) -> Vec<Effect> {
        let mut effects = Vec::new();
        match action {
            Action::Cancel => {
                self.dialog = None;
            }
            Action::RunChar(ch) => {
                if let Some(Dialog::Run { input }) = &mut self.dialog {
                    input.push(ch);
                }
            }
            Action::RunBackspace => {
                if let Some(Dialog::Run { input }) = &mut self.dialog {
                    input.pop();
                }
            }
            Action::RunSubmit => {
                if let Some(Dialog::Run { input }) = self.dialog.take() {
                    let parts = split_command_line(&input);
                    match parts.split_first() {
                        Some((program, args)) => effects.push(Effect::Run {
                            program: program.clone(),
                            args: args.to_vec(),
                        }),
                        None => self.notice("Nothing to run."),
                    }
                }
            }
            Action::PasswordChar(ch) => {
                if let Some(Dialog::Password { input, error, .. }) = &mut self.dialog {
                    input.push(ch);
                    *error = false;
                }
            }
            Action::PasswordBackspace => {
                if let Some(Dialog::Password { input, error, .. }) = &mut self.dialog {
                    input.pop();
                    *error = false;
                }
            }
            Action::PasswordSubmit => {
                if matches!(self.dialog, Some(Dialog::Password { .. })) {
                    let Some(Dialog::Password { action, input, .. }) = self.dialog.take() else {
                        return effects;
                    };
                    if input.is_empty() {
                        self.dialog = Some(Dialog::Password {
                            action,
                            input,
                            error: true,
                        });
                    } else {
                        effects.push(Effect::ElevatedWithPassword {
                            action,
                            password: input,
                        });
                    }
                }
            }
            Action::MoveUp | Action::MoveDown => {
                let delta = if action == Action::MoveUp { -1 } else { 1 };
                match &mut self.dialog {
                    Some(Dialog::Signal { selected, .. }) => {
                        let count = Signal::ALL.len() as isize;
                        *selected = (*selected as isize + delta).rem_euclid(count) as usize;
                    }
                    Some(Dialog::Appearance { selected }) => {
                        *selected = (*selected as isize + delta).rem_euclid(4) as usize;
                    }
                    _ => {}
                }
            }
            Action::ToggleSignalTree => {
                if let Some(Dialog::Signal { tree, .. }) = &mut self.dialog {
                    *tree = !*tree;
                }
            }
            Action::PriorityStep(delta) => {
                if let Some(Dialog::Priority {
                    input,
                    editing,
                    error,
                    ..
                }) = &mut self.dialog
                {
                    let current = input.trim().parse::<i32>().unwrap_or(0);
                    *input = crate::model::Priority::clamp(current + delta).to_string();
                    *editing = false;
                    *error = false;
                }
            }
            Action::PriorityChar(ch) => {
                if let Some(Dialog::Priority {
                    input,
                    editing,
                    error,
                    ..
                }) = &mut self.dialog
                {
                    if !*editing {
                        input.clear();
                        *editing = true;
                    }
                    if ch != '-' || input.is_empty() {
                        input.push(ch);
                    }
                    *error = false;
                }
            }
            Action::PriorityBackspace => {
                if let Some(Dialog::Priority {
                    input,
                    editing,
                    error,
                    ..
                }) = &mut self.dialog
                {
                    if !*editing {
                        input.clear();
                        *editing = true;
                    }
                    input.pop();
                    *error = false;
                }
            }
            Action::Confirm | Action::CycleBack
                if matches!(self.dialog, Some(Dialog::Appearance { .. })) =>
            {
                let forward = action == Action::Confirm;
                let selected = match &self.dialog {
                    Some(Dialog::Appearance { selected }) => *selected,
                    _ => 0,
                };
                match selected {
                    0 => {
                        if forward {
                            self.toggle_theme();
                        } else {
                            self.previous_theme();
                        }
                        effects.push(Effect::PersistSettings);
                    }
                    1 => {
                        let style = if forward {
                            self.chart_style.next()
                        } else {
                            crate::model::ChartStyle::ALL[(crate::model::ChartStyle::ALL
                                .iter()
                                .position(|s| *s == self.chart_style)
                                .unwrap_or(0)
                                + crate::model::ChartStyle::ALL.len()
                                - 1)
                                % crate::model::ChartStyle::ALL.len()]
                        };
                        self.set_chart_style(style);
                        effects.push(Effect::PersistSettings);
                    }
                    3 => {
                        let next = !self.chart_gradient;
                        self.set_chart_gradient(next);
                        effects.push(Effect::PersistSettings);
                    }
                    _ => {
                        let colors = crate::model::ChartColor::ALL;
                        let index = colors
                            .iter()
                            .position(|c| *c == self.chart_color)
                            .unwrap_or(0);
                        let next = if forward {
                            (index + 1) % colors.len()
                        } else {
                            (index + colors.len() - 1) % colors.len()
                        };
                        self.set_chart_color(colors[next]);
                        effects.push(Effect::PersistSettings);
                    }
                }
                return effects;
            }
            Action::Confirm => {
                if let Some(Dialog::Signal {
                    key,
                    selected,
                    tree,
                    ..
                }) = &self.dialog
                {
                    let signal = Signal::ALL[(*selected).min(Signal::ALL.len() - 1)];
                    let key = *key;
                    let tree = *tree;
                    self.dialog = None;
                    return self.submit_signal(key, signal, tree);
                }
                if let Some(Dialog::Priority { key, input, .. }) = &self.dialog {
                    let key = *key;
                    match input.trim().parse::<i32>() {
                        Ok(nice) => {
                            let nice = crate::model::Priority::clamp(nice);
                            self.dialog = None;
                            effects.push(Effect::Priority { key, nice });
                        }
                        Err(_) => {
                            if let Some(Dialog::Priority { error, .. }) = &mut self.dialog {
                                *error = true;
                            }
                        }
                    }
                    return effects;
                }
                if let Some(Dialog::Confirm { action, .. }) = self.dialog.take() {
                    match action {
                        PendingAction::EndTask(key) => effects.push(Effect::Signal {
                            key,
                            signal: Signal::Term,
                            tree: false,
                        }),
                        PendingAction::EndTree(key) => effects.push(Effect::Signal {
                            key,
                            signal: Signal::Kill,
                            tree: true,
                        }),
                        PendingAction::ClearHistory => effects.push(Effect::ClearHistory),
                        PendingAction::ClearUsage(name) => effects.push(Effect::ClearUsage(name)),
                        PendingAction::Elevate(action) => effects.push(Effect::Elevated(action)),
                        PendingAction::EndApp(name) => {
                            let processes = self.app_processes(&name);
                            let pids: std::collections::HashSet<u32> =
                                processes.iter().map(|row| row.key.pid).collect();
                            for row in processes {
                                if row.parent_pid.is_some_and(|parent| pids.contains(&parent)) {
                                    continue;
                                }
                                effects.push(Effect::Signal {
                                    key: row.key,
                                    signal: Signal::Term,
                                    tree: false,
                                });
                            }
                        }
                        PendingAction::Service(name, action) => {
                            effects.push(Effect::Service { name, action })
                        }
                        PendingAction::Startup(name, enabled) => {
                            effects.push(Effect::Startup { name, enabled })
                        }
                        PendingAction::Priority(key, nice) => {
                            effects.push(Effect::Priority { key, nice })
                        }
                        PendingAction::Signal(key, signal, tree) => {
                            effects.push(Effect::Signal { key, signal, tree })
                        }
                    }
                }
            }
            _ => {}
        }
        effects
    }

    fn dispatch_menu(&mut self, action: Action) -> Vec<Effect> {
        let mut state = self.menu.unwrap_or_default();
        match action {
            Action::MenuClose | Action::Cancel => {
                self.menu = None;
            }
            Action::MenuLeft => {
                state.menu = (state.menu + MENUS.len() - 1) % MENUS.len();
                state.item = 0;
                self.menu = Some(state);
            }
            Action::MenuRight => {
                state.menu = (state.menu + 1) % MENUS.len();
                state.item = 0;
                self.menu = Some(state);
            }
            Action::MenuUp => {
                let count = MENUS[state.menu].1.len();
                state.item = (state.item + count - 1) % count;
                self.menu = Some(state);
            }
            Action::MenuFirst => {
                state.item = 0;
                self.menu = Some(state);
            }
            Action::MenuLast => {
                state.item = MENUS[state.menu].1.len() - 1;
                self.menu = Some(state);
            }
            Action::MenuDown => {
                let count = MENUS[state.menu].1.len();
                state.item = (state.item + 1) % count;
                self.menu = Some(state);
            }
            Action::MenuActivate => {
                let item = MENUS[state.menu].1.get(state.item).map(|(_, item)| *item);
                self.menu = None;
                if let Some(item) = item {
                    return self.menu_item(item);
                }
            }
            _ => {
                self.menu = Some(state);
            }
        }
        Vec::new()
    }

    fn menu_item(&mut self, item: crate::app::MenuItem) -> Vec<Effect> {
        use crate::app::MenuItem;
        let mut effects = Vec::new();
        let action = match item {
            MenuItem::Run => {
                self.dialog = Some(Dialog::Run {
                    input: String::new(),
                });
                return effects;
            }
            MenuItem::Exit => Action::Quit,
            MenuItem::SpeedHigh => {
                self.set_speed("high", &mut effects);
                return effects;
            }
            MenuItem::SpeedNormal => {
                self.set_speed("normal", &mut effects);
                return effects;
            }
            MenuItem::SpeedLow => {
                self.set_speed("low", &mut effects);
                return effects;
            }
            MenuItem::SpeedPaused => {
                self.set_speed("paused", &mut effects);
                return effects;
            }
            MenuItem::Refresh => Action::Refresh,
            MenuItem::ToggleDetails => Action::ToggleDetails,
            MenuItem::OptionalColumns => Action::ToggleOptionalColumns,
            MenuItem::ToggleHints => Action::ToggleHints,
            MenuItem::Appearance => {
                self.dialog = Some(Dialog::Appearance { selected: 0 });
                return effects;
            }
            MenuItem::SortNext => Action::SortNext,
            MenuItem::SortReverse => Action::SortFlip,
            MenuItem::ExpandAll => {
                self.expand_all();
                return effects;
            }
            MenuItem::CollapseAll => {
                self.collapse_all();
                return effects;
            }
            MenuItem::EndTask => Action::EndTask,
            MenuItem::EndTree => Action::EndTree,
            MenuItem::Signal => Action::OpenSignal,
            MenuItem::Priority => Action::SetPriority,
            MenuItem::Properties => Action::ShowProperties,
            MenuItem::ClearHistory => Action::DeleteHistory,
            MenuItem::Help => Action::Help,
            MenuItem::About => {
                self.dialog = Some(Dialog::About);
                return effects;
            }
        };
        effects.extend(self.dispatch(action));
        effects
    }
}

pub fn process_lines(row: &ProcRow) -> Vec<String> {
    vec![
        format!("Name: {}", row.name),
        format!("PID: {}", row.key.pid),
        format!(
            "Command line: {}",
            if row.cmd.is_empty() { "-" } else { &row.cmd }
        ),
        format!(
            "Executable: {}",
            row.exe.clone().unwrap_or_else(|| "-".into())
        ),
        format!(
            "Parent: {}",
            row.parent_pid
                .map(|p| p.to_string())
                .unwrap_or_else(|| "-".into())
        ),
        format!("Owner: {}", row.user.clone().unwrap_or_else(|| "-".into())),
        format!(
            "Threads: {}",
            row.thread_count()
                .map(|t| t.to_string())
                .unwrap_or_else(|| "-".into())
        ),
        format!("Status: {}", row.status.label()),
        format!("Priority: {}", row.priority.label()),
        format!(
            "Affinity: {}",
            if crate::model::Capabilities::for_current_platform().affinity {
                "-"
            } else {
                "not supported on this platform"
            }
        ),
        format!(
            "Started: {}",
            crate::ui::format::duration(row.run_time_secs)
        ),
    ]
}

/// Collect every descendant of `pid` by walking parent links.
pub fn descendants(processes: &[ProcRow], pid: u32) -> Vec<ProcKey> {
    let mut found: Vec<ProcKey> = Vec::new();
    let mut frontier = vec![pid];
    while let Some(parent) = frontier.pop() {
        for process in processes.iter().filter(|p| p.parent_pid == Some(parent)) {
            if process.key.pid == pid || found.contains(&process.key) {
                continue;
            }
            found.push(process.key);
            frontier.push(process.key.pid);
        }
    }
    found
}

/// Split a command line into a program and arguments, honouring single and
/// double quotes. No shell is involved, so metacharacters stay literal.
pub fn split_command_line(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;

    for ch in input.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        match quote {
            Some(q) => {
                if ch == q {
                    quote = None;
                } else if ch == '\\' && q == '"' {
                    escaped = true;
                } else {
                    current.push(ch);
                }
            }
            None => match ch {
                '"' | '\'' => quote = Some(ch),
                '\\' => escaped = true,
                ch if ch.is_whitespace() => {
                    if !current.is_empty() {
                        parts.push(std::mem::take(&mut current));
                    }
                }
                ch => current.push(ch),
            },
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::App;
    use crate::input::Action;
    use crate::store::settings::Settings;

    fn app() -> App {
        let (mut app, _) = App::new(Settings::default(), false);
        app.apply_snapshot(crate::model::Snapshot::fixture());
        app
    }

    #[test]
    fn quit_sets_flag_and_effect() {
        let mut app = app();
        let effects = app.dispatch(Action::Quit);
        assert!(app.quit);
        assert_eq!(effects, vec![Effect::Quit]);
    }

    #[test]
    fn ending_an_owned_process_signals_immediately() {
        let mut app = app();
        app.tab = Tab::Details;
        // The fixture's uid 501 is only "us" on a default macOS account.
        let mut snapshot = crate::model::Snapshot::fixture();
        for p in &mut snapshot.processes {
            p.uid = Some(crate::platform::current_uid());
        }
        app.apply_snapshot(snapshot);
        let row = app
            .snapshot
            .processes
            .iter()
            .find(|p| p.kind == Kind::Background)
            .unwrap()
            .clone();
        app.details.key = Some(row.key);
        app.sync_details_public();
        let effects = app.dispatch(Action::EndTask);
        assert!(app.dialog.is_none());
        assert!(matches!(
            effects.as_slice(),
            [Effect::Signal { key, signal: Signal::Term, tree: false }] if *key == row.key
        ));
    }

    #[test]
    fn ending_a_system_process_asks_first() {
        let mut app = app();
        app.tab = Tab::Details;
        let row = app
            .snapshot
            .processes
            .iter()
            .find(|p| p.kind == Kind::System)
            .unwrap()
            .clone();
        app.details.key = Some(row.key);
        app.sync_details_public();
        let effects = app.dispatch(Action::EndTask);
        assert!(effects.is_empty());
        assert!(matches!(app.dialog, Some(Dialog::Confirm { .. })));
        let effects = app.dispatch(Action::Confirm);
        assert!(matches!(
            effects.as_slice(),
            [Effect::Elevated(ElevatedAction::Signal {
                signal: Signal::Term,
                ..
            })]
        ));
    }

    #[test]
    fn cancel_leaves_no_effect() {
        let mut app = app();
        app.dialog = Some(Dialog::Confirm {
            title: "End task".into(),
            message: "Sure?".into(),
            confirm_label: "End task".into(),
            action: PendingAction::ClearHistory,
        });
        let effects = app.dispatch(Action::Cancel);
        assert!(app.dialog.is_none());
        assert!(effects.is_empty());
    }

    #[test]
    fn fewer_details_toggles() {
        let mut app = app();
        assert!(app.process.more_details);
        app.dispatch(Action::ToggleDetails);
        assert!(!app.process.more_details);
    }

    #[test]
    fn speed_changes_emit_interval_and_persist() {
        let mut app = app();
        let effects = app.dispatch(Action::SpeedSlower);
        assert!(effects.contains(&Effect::PersistSettings));
        assert!(matches!(
            effects.iter().find(|e| matches!(e, Effect::SetInterval(_))),
            Some(Effect::SetInterval(Some(_)))
        ));
        assert_eq!(app.settings.update_speed, "low");
    }

    #[test]
    fn stopping_a_service_asks_first() {
        let mut app = app();
        app.tab = Tab::Services;
        app.sync_services_public();
        let effects = app.dispatch(Action::ServiceStop);
        assert!(effects.is_empty());
        assert!(app.dialog.as_ref().unwrap().title().contains("Stop"));
        let effects = app.dispatch(Action::Confirm);
        assert!(matches!(
            effects.as_slice(),
            [Effect::Elevated(ElevatedAction::Service {
                action: ServiceAction::Stop,
                ..
            })]
        ));
    }

    #[test]
    fn starting_a_user_service_runs_without_a_confirmation() {
        let mut app = app();
        app.tab = Tab::Services;
        app.sync_services_public();
        let index = app
            .service_rows()
            .iter()
            .position(|row| row.scope == crate::model::Scope::User)
            .unwrap();
        app.services.selected = index;
        app.services.key = Some(app.service_rows()[index].name.clone());
        let effects = app.dispatch(Action::ServiceStart);
        assert!(app.dialog.is_none());
        assert!(matches!(
            effects.as_slice(),
            [Effect::Service {
                action: ServiceAction::Start,
                ..
            }]
        ));
    }

    #[test]
    fn starting_a_system_service_offers_elevation() {
        let mut app = app();
        app.tab = Tab::Services;
        app.sync_services_public();
        let index = app
            .service_rows()
            .iter()
            .position(|row| row.scope == crate::model::Scope::System)
            .unwrap();
        app.services.selected = index;
        app.services.key = Some(app.service_rows()[index].name.clone());
        let effects = app.dispatch(Action::ServiceStart);
        assert!(effects.is_empty());
        assert!(matches!(app.dialog, Some(Dialog::Confirm { .. })));
        let effects = app.dispatch(Action::Confirm);
        assert!(matches!(
            effects.as_slice(),
            [Effect::Elevated(ElevatedAction::Service {
                action: ServiceAction::Start,
                ..
            })]
        ));
    }

    #[test]
    fn password_dialog_emits_one_elevated_effect() {
        use crate::platform::ElevatedAction;
        let mut app = app();
        let action = ElevatedAction::Signal {
            pids: vec![42],
            signal: Signal::Term,
        };
        app.dialog = Some(Dialog::Password {
            action: action.clone(),
            input: String::new(),
            error: false,
        });
        app.dispatch(Action::PasswordChar('s'));
        app.dispatch(Action::PasswordChar('e'));
        app.dispatch(Action::PasswordBackspace);
        let effects = app.dispatch(Action::PasswordSubmit);
        match effects.as_slice() {
            [
                Effect::ElevatedWithPassword {
                    action: sent,
                    password,
                },
            ] => {
                assert_eq!(*sent, action);
                assert_eq!(password, "s");
            }
            other => panic!("unexpected effects: {other:?}"),
        }
        assert!(app.dialog.is_none());
    }

    #[test]
    fn empty_password_is_refused() {
        use crate::platform::ElevatedAction;
        let mut app = app();
        app.dialog = Some(Dialog::Password {
            action: ElevatedAction::Signal {
                pids: vec![42],
                signal: Signal::Term,
            },
            input: String::new(),
            error: false,
        });
        let effects = app.dispatch(Action::PasswordSubmit);
        assert!(effects.is_empty());
        assert!(matches!(
            app.dialog,
            Some(Dialog::Password { error: true, .. })
        ));
    }

    #[test]
    fn startup_toggle_emits_an_effect() {
        let mut app = app();
        app.tab = Tab::Startup;
        app.sync_startup_public();
        let enabled = app.selected_startup().unwrap().enabled;
        let effects = app.dispatch(Action::ToggleStartup);
        assert!(matches!(
            effects.as_slice(),
            [Effect::Startup { enabled: new, .. }] if *new != enabled
        ));
    }

    #[test]
    fn sorting_cycles_columns_and_direction() {
        let mut app = app();
        assert_eq!(app.process.sort.key, SortKey::Cpu);
        app.dispatch(Action::SortNext);
        assert_eq!(app.process.sort.key, SortKey::Memory);
        app.dispatch(Action::SortFlip);
        assert_eq!(app.process.sort.dir, crate::sample::group::SortDir::Asc);
    }

    #[test]
    fn menu_navigates_and_activates() {
        let mut app = app();
        app.dispatch(Action::MenuOpen);
        assert!(app.menu.is_some());
        app.dispatch(Action::MenuRight);
        assert_eq!(app.menu.unwrap().menu, 1);
        app.dispatch(Action::MenuDown);
        app.dispatch(Action::MenuActivate);
        assert!(app.menu.is_none());
        assert!(app.dialog.is_none());
    }

    #[test]
    fn run_dialog_splits_without_a_shell() {
        let mut app = app();
        app.dialog = Some(Dialog::Run {
            input: "printf '%s' \"a; rm -rf /\"".to_string(),
        });
        let effects = app.dispatch(Action::RunSubmit);
        match effects.as_slice() {
            [Effect::Run { program, args }] => {
                assert_eq!(program, "printf");
                assert_eq!(args, &vec!["%s".to_string(), "a; rm -rf /".to_string()]);
            }
            other => panic!("unexpected effects: {other:?}"),
        }
    }

    #[test]
    fn empty_run_dialog_does_nothing() {
        let mut app = app();
        app.dialog = Some(Dialog::Run {
            input: "   ".to_string(),
        });
        let effects = app.dispatch(Action::RunSubmit);
        assert!(effects.is_empty());
        assert!(app.dialog.is_none());
    }

    fn a_background_process(app: &mut App) -> ProcRow {
        app.tab = Tab::Details;
        let row = app
            .snapshot
            .processes
            .iter()
            .find(|p| p.kind == Kind::Background)
            .unwrap()
            .clone();
        app.details.key = Some(row.key);
        app.sync_details_public();
        row
    }

    #[test]
    fn priority_dialog_starts_from_the_current_nice() {
        let mut app = app();
        let row = a_background_process(&mut app);
        app.dispatch(Action::SetPriority);
        assert!(matches!(
            &app.dialog,
            Some(Dialog::Priority { input, .. }) if input == &row.priority.nice.to_string()
        ));
    }

    #[test]
    fn priority_steps_and_types_a_value() {
        let mut app = app();
        let row = a_background_process(&mut app);
        app.dispatch(Action::SetPriority);
        app.dispatch(Action::PriorityStep(5));
        assert!(matches!(
            &app.dialog,
            Some(Dialog::Priority { input, .. }) if input == "5"
        ));
        app.dispatch(Action::PriorityChar('7'));
        assert!(matches!(
            &app.dialog,
            Some(Dialog::Priority { input, editing, .. }) if input == "7" && *editing
        ));
        let effects = app.dispatch(Action::Confirm);
        assert!(matches!(
            effects.as_slice(),
            [Effect::Priority { key, nice }] if *key == row.key && *nice == 7
        ));
        assert!(app.dialog.is_none());
    }

    #[test]
    fn priority_clamps_out_of_range_values() {
        let mut app = app();
        a_background_process(&mut app);
        app.dispatch(Action::SetPriority);
        app.dispatch(Action::PriorityChar('4'));
        app.dispatch(Action::PriorityChar('0'));
        let effects = app.dispatch(Action::Confirm);
        assert!(matches!(
            effects.as_slice(),
            [Effect::Priority { nice: 19, .. }]
        ));
        app.dispatch(Action::SetPriority);
        app.dispatch(Action::PriorityChar('-'));
        app.dispatch(Action::PriorityChar('9'));
        let effects = app.dispatch(Action::Confirm);
        assert!(matches!(
            effects.as_slice(),
            [Effect::Priority { nice: -9, .. }]
        ));
    }

    #[test]
    fn priority_refuses_an_empty_field() {
        let mut app = app();
        a_background_process(&mut app);
        app.dispatch(Action::SetPriority);
        app.dispatch(Action::PriorityBackspace);
        let effects = app.dispatch(Action::Confirm);
        assert!(effects.is_empty());
        assert!(matches!(
            app.dialog,
            Some(Dialog::Priority { error: true, .. })
        ));
    }

    #[test]
    fn priority_opens_from_the_processes_tab() {
        let mut app = app();
        app.tab = Tab::Processes;
        let index = app
            .process
            .items
            .iter()
            .position(|item| matches!(item, crate::sample::group::ProcItem::Group { .. }))
            .unwrap();
        app.select_row(index);
        app.dispatch(Action::SetPriority);
        assert!(matches!(app.dialog, Some(Dialog::Priority { .. })));
        assert!(!app.notices.iter().any(|n| n.contains("Details tab")));
    }

    fn owned_background(app: &mut App) -> ProcRow {
        let uid = crate::platform::current_uid();
        let mut snapshot = crate::model::Snapshot::fixture();
        for p in &mut snapshot.processes {
            p.uid = Some(uid);
        }
        app.apply_snapshot(snapshot);
        a_background_process(app)
    }

    #[test]
    fn k_on_a_process_row_opens_the_signal_dialog() {
        let mut app = app();
        app.tab = Tab::Processes;
        for process in &app.snapshot.processes {
            app.process
                .expanded_groups
                .insert((process.kind, process.name.clone()));
        }
        app.process.refresh(&app.snapshot);
        let index = app
            .process
            .items
            .iter()
            .position(|item| matches!(item, crate::sample::group::ProcItem::Child { .. }))
            .unwrap();
        app.select_row(index);
        let effects = app.dispatch(Action::OpenSignal);
        assert!(effects.is_empty());
        assert!(
            matches!(app.dialog, Some(Dialog::Signal { .. })),
            "notices: {:?}",
            app.notices
        );
    }

    #[test]
    fn signal_dialog_sends_term_for_an_owned_process() {
        let mut app = app();
        let row = owned_background(&mut app);
        app.dispatch(Action::OpenSignal);
        assert!(matches!(app.dialog, Some(Dialog::Signal { .. })));
        let effects = app.dispatch(Action::Confirm);
        assert!(matches!(
            effects.as_slice(),
            [Effect::Signal { key, signal: Signal::Term, tree: false }] if *key == row.key
        ));
        assert!(app.dialog.is_none());
    }

    #[test]
    fn signal_dialog_confirms_kill() {
        let mut app = app();
        owned_background(&mut app);
        app.dispatch(Action::OpenSignal);
        app.dispatch(Action::MoveDown);
        let effects = app.dispatch(Action::Confirm);
        assert!(effects.is_empty());
        assert!(matches!(app.dialog, Some(Dialog::Confirm { .. })));
        let effects = app.dispatch(Action::Confirm);
        assert!(matches!(
            effects.as_slice(),
            [Effect::Signal {
                signal: Signal::Kill,
                tree: false,
                ..
            }]
        ));
    }

    #[test]
    fn signal_tree_toggle_reaches_the_effect() {
        let mut app = app();
        owned_background(&mut app);
        app.dispatch(Action::OpenSignal);
        app.dispatch(Action::ToggleSignalTree);
        let effects = app.dispatch(Action::Confirm);
        assert!(matches!(
            effects.as_slice(),
            [Effect::Signal {
                signal: Signal::Term,
                tree: true,
                ..
            }]
        ));
    }

    #[test]
    fn signal_confirms_for_a_system_process() {
        let mut app = app();
        app.tab = Tab::Details;
        let row = app
            .snapshot
            .processes
            .iter()
            .find(|p| p.kind == Kind::System)
            .unwrap()
            .clone();
        app.details.key = Some(row.key);
        app.sync_details_public();
        app.dispatch(Action::OpenSignal);
        let effects = app.dispatch(Action::Confirm);
        assert!(effects.is_empty());
        assert!(matches!(app.dialog, Some(Dialog::Confirm { .. })));
    }

    #[test]
    fn enter_on_a_process_opens_the_info_modal_and_expands_groups() {
        let mut app = app();
        app.tab = Tab::Processes;
        for process in &app.snapshot.processes {
            app.process
                .expanded_groups
                .insert((process.kind, process.name.clone()));
        }
        app.process.refresh(&app.snapshot);
        let child = app
            .process
            .items
            .iter()
            .position(|item| matches!(item, crate::sample::group::ProcItem::Child { .. }))
            .unwrap();
        app.select_row(child);
        let effects = app.dispatch(Action::Expand);
        assert!(effects.is_empty());
        assert!(matches!(app.dialog, Some(Dialog::ProcessInfo { .. })));

        let mut group_app = self::app();
        group_app.tab = Tab::Processes;
        let group = group_app
            .process
            .items
            .iter()
            .position(|item| matches!(item, crate::sample::group::ProcItem::Group { .. }))
            .unwrap();
        group_app.select_row(group);
        group_app.dispatch(Action::Expand);
        assert!(group_app.dialog.is_none(), "groups still expand");
    }

    #[test]
    fn signal_from_a_kind_heading_uses_its_first_process() {
        let mut app = app();
        app.tab = Tab::Processes;
        app.collapse_all();
        assert!(matches!(app.process.selected_key(), Some(ProcSel::Kind(_))));
        app.dispatch(Action::OpenSignal);
        assert!(matches!(app.dialog, Some(Dialog::Signal { .. })));
    }

    #[test]
    fn signal_without_rows_only_notices() {
        let mut app = app();
        app.tab = Tab::Processes;
        app.apply_snapshot(crate::model::Snapshot::default());
        app.process.refresh(&app.snapshot);
        let effects = app.dispatch(Action::OpenSignal);
        assert!(effects.is_empty());
        assert!(app.dialog.is_none());
        assert!(app.notices.last().unwrap().contains("Select a process"));
    }

    #[test]
    fn signal_for_every_descendant_is_one_effect() {
        let processes = crate::model::Snapshot::fixture().processes;
        let found = descendants(&processes, 1);
        assert!(found.iter().any(|key| key.pid == 4242));
    }

    #[test]
    fn performance_rail_moves_with_up_and_down() {
        use crate::app::ResourceChoice;
        let mut app = app();
        app.tab = Tab::Performance;
        assert_eq!(app.performance.resource, ResourceChoice::Cpu);
        app.dispatch(Action::MoveDown);
        assert_eq!(app.performance.resource, ResourceChoice::Memory);
        app.dispatch(Action::MoveUp);
        assert_eq!(app.performance.resource, ResourceChoice::Cpu);
        app.dispatch(Action::MoveUp);
        assert_eq!(app.performance.resource, ResourceChoice::Gpu, "wraps up");
        app.dispatch(Action::MoveDown);
        assert_eq!(app.performance.resource, ResourceChoice::Cpu, "wraps down");
    }

    #[test]
    fn clicking_a_tab_switches_to_it() {
        let mut app = app();
        let effects = app.click(crate::ui::hits::Target::Tab(Tab::Performance));
        assert!(effects.is_empty());
        assert_eq!(app.tab, Tab::Performance);
    }

    #[test]
    fn clicking_a_row_selects_it() {
        let mut app = app();
        app.dispatch(Action::GoTab(5));
        let effects = app.click(crate::ui::hits::Target::Row(2));
        assert!(effects.is_empty());
        assert_eq!(app.details.selected, 2);
        app.dispatch(Action::GoTab(0));
        app.click(crate::ui::hits::Target::Row(1));
        assert_eq!(app.process.selected, 1);
    }

    #[test]
    fn clicking_a_menu_label_opens_it() {
        let mut app = app();
        app.click(crate::ui::hits::Target::Menu(2));
        assert_eq!(app.menu.unwrap().menu, 2);
    }

    #[test]
    fn clicking_a_resource_selects_it() {
        let mut app = app();
        app.click(crate::ui::hits::Target::Resource(
            crate::app::ResourceChoice::Disk,
        ));
        assert_eq!(app.performance.resource, crate::app::ResourceChoice::Disk);
    }

    #[test]
    fn clicking_dialog_buttons_confirms_or_cancels() {
        let mut app = app();
        app.dialog = Some(Dialog::Confirm {
            title: "Delete history".into(),
            message: "Sure?".into(),
            confirm_label: "Delete".into(),
            action: PendingAction::ClearHistory,
        });
        let effects = app.click(crate::ui::hits::Target::DialogConfirm);
        assert_eq!(effects, vec![Effect::ClearHistory]);
        app.dialog = Some(Dialog::Help);
        let effects = app.click(crate::ui::hits::Target::DialogClose);
        assert!(effects.is_empty());
        assert!(app.dialog.is_none());
    }

    #[test]
    fn appearance_dialog_changes_style_colour_and_theme() {
        let mut app = app();
        app.dialog = Some(Dialog::Appearance { selected: 1 });
        let effects = app.dispatch(Action::Confirm);
        assert!(effects.contains(&Effect::PersistSettings));
        assert_eq!(app.chart_style, crate::model::ChartStyle::Line);
        assert_eq!(app.settings.chart_style, "line");
        app.dialog = Some(Dialog::Appearance { selected: 2 });
        app.dispatch(Action::Confirm);
        assert_eq!(app.chart_color, crate::model::ChartColor::Teal);
        assert_eq!(app.settings.chart_color, "teal");
        app.dialog = Some(Dialog::Appearance { selected: 2 });
        app.dispatch(Action::CycleBack);
        assert_eq!(app.chart_color, crate::model::ChartColor::PerResource);
        app.dialog = Some(Dialog::Appearance { selected: 0 });
        app.dispatch(Action::Confirm);
        assert_eq!(app.theme_name, crate::theme::ThemeName::Dark);
    }

    #[test]
    fn f4_hides_the_hints_and_persists() {
        let mut app = app();
        assert!(app.show_hints);
        let effects = app.dispatch(Action::ToggleHints);
        assert!(!app.show_hints);
        assert!(!app.settings.show_hints);
        assert!(effects.contains(&Effect::PersistSettings));
    }

    #[test]
    fn process_menu_lists_the_process_actions() {
        let menu = crate::app::MENUS
            .iter()
            .find(|(label, _)| *label == "Process")
            .unwrap();
        let labels: Vec<&str> = menu.1.iter().map(|(label, _)| *label).collect();
        assert_eq!(
            labels,
            vec![
                "End task",
                "End process tree",
                "Send signal...",
                "Set priority...",
                "Properties"
            ]
        );
    }

    #[test]
    fn help_menu_offers_about() {
        let help = crate::app::MENUS
            .iter()
            .find(|(label, _)| *label == "Help")
            .unwrap();
        assert!(
            help.1
                .iter()
                .any(|(label, item)| *label == "About" && *item == crate::app::MenuItem::About)
        );
    }

    #[test]
    fn options_menu_has_no_theme_entry() {
        let options = crate::app::MENUS
            .iter()
            .find(|(label, _)| *label == "Options")
            .unwrap();
        assert!(
            !options
                .1
                .iter()
                .any(|(label, _)| *label == "Next theme" || label.to_lowercase().contains("theme")),
            "theme is switched from t and Appearance"
        );
        assert!(
            options
                .1
                .iter()
                .all(|(label, _)| label.starts_with("Update speed"))
        );
    }

    #[test]
    fn view_menu_offers_appearance() {
        let view = crate::app::MENUS
            .iter()
            .find(|(label, _)| *label == "View")
            .unwrap();
        assert!(view.1.iter().any(|(label, item)| {
            *label == "Appearance..." && *item == crate::app::MenuItem::Appearance
        }));
    }

    #[test]
    fn appearance_toggles_and_persists_the_gradient() {
        let mut app = app();
        assert!(app.chart_gradient);
        app.dialog = Some(Dialog::Appearance { selected: 3 });
        let effects = app.dispatch(Action::Confirm);
        assert!(!app.chart_gradient);
        assert!(!app.settings.chart_gradient);
        assert!(effects.contains(&Effect::PersistSettings));
        app.dialog = Some(Dialog::Appearance { selected: 3 });
        app.dispatch(Action::Confirm);
        assert!(app.chart_gradient);
    }

    #[test]
    fn appearance_back_cycle_wraps_to_blocks() {
        let mut app = app();
        assert_eq!(app.chart_style, crate::model::ChartStyle::Filled);
        app.dialog = Some(Dialog::Appearance { selected: 1 });
        app.dispatch(Action::CycleBack);
        assert_eq!(app.chart_style, crate::model::ChartStyle::Blocks);
    }

    #[test]
    fn affinity_reports_that_it_is_unavailable() {
        let mut app = app();
        app.dispatch(Action::SetAffinity);
        assert!(
            app.notices
                .last()
                .unwrap()
                .contains("not supported on Linux or macOS")
        );
        assert!(app.dialog.is_none());
    }

    #[test]
    fn app_history_clears_one_entry_and_ends_an_app() {
        let mut app = app();
        app.tab = Tab::AppHistory;
        app.sync_history_public();
        let row = app.history_rows()[0].name.clone();
        app.dispatch(Action::ClearUsageEntry);
        assert!(matches!(app.dialog, Some(Dialog::Confirm { .. })));
        let effects = app.dispatch(Action::Confirm);
        assert!(matches!(
            effects.as_slice(),
            [Effect::ClearUsage(name)] if *name == row
        ));

        app.apply_snapshot(crate::model::Snapshot::fixture());
        app.tab = Tab::AppHistory;
        app.sync_history_public();
        let safari = app
            .history_rows()
            .iter()
            .position(|row| row.name == "Safari")
            .unwrap();
        app.app_history.selected = safari;
        app.app_history.key = Some("Safari".into());
        app.dispatch(Action::EndTask);
        assert!(matches!(app.dialog, Some(Dialog::Confirm { .. })));
        let effects = app.dispatch(Action::Confirm);
        assert!(
            effects
                .iter()
                .any(|effect| matches!(effect, Effect::Signal { .. })),
            "no signals: {effects:?}"
        );
    }

    #[test]
    fn app_history_without_processes_only_notices() {
        let mut snapshot = crate::model::Snapshot::fixture();
        snapshot.app_usage.push(crate::model::AppUsage {
            name: "Ghost".into(),
            ..snapshot.app_usage[0].clone()
        });
        let mut app = app();
        app.apply_snapshot(snapshot);
        app.tab = Tab::AppHistory;
        app.sync_history_public();
        let ghost = app
            .history_rows()
            .iter()
            .position(|row| row.name == "Ghost")
            .unwrap();
        app.app_history.selected = ghost;
        app.app_history.key = Some("Ghost".into());
        app.dispatch(Action::EndTask);
        assert!(app.dialog.is_none());
        assert!(app.notices.last().unwrap().contains("no running processes"));
    }

    #[test]
    fn deleting_history_asks_and_emits_the_effect() {
        let mut app = app();
        app.dispatch(Action::DeleteHistory);
        assert!(matches!(app.dialog, Some(Dialog::Confirm { .. })));
        let effects = app.dispatch(Action::Confirm);
        assert_eq!(effects, vec![Effect::ClearHistory]);
    }

    #[test]
    fn view_menu_can_collapse_everything() {
        let mut app = app();
        let view = crate::app::MENUS
            .iter()
            .position(|(label, _)| *label == "View")
            .unwrap();
        let item = crate::app::MENUS[view]
            .1
            .iter()
            .position(|(label, _)| *label == "Collapse all")
            .unwrap();
        app.menu = Some(crate::app::MenuState { menu: view, item });
        app.dispatch(Action::MenuActivate);
        assert_eq!(app.process.collapsed_kinds.len(), 3);
    }

    #[test]
    fn optional_columns_can_be_hidden_from_the_menu() {
        let mut app = app();
        assert!(app.show_optional());
        app.dispatch(Action::ToggleOptionalColumns);
        assert!(!app.show_optional());
        app.dispatch(Action::ToggleOptionalColumns);
        assert!(app.show_optional());
    }

    #[test]
    fn descendants_walk_the_tree() {
        let processes = crate::model::Snapshot::fixture().processes;
        let found = descendants(&processes, 1);
        assert!(found.iter().any(|key| key.pid == 4242));
    }

    #[test]
    fn command_line_quoting() {
        assert_eq!(
            split_command_line("/bin/echo \"hello world\" plain"),
            vec!["/bin/echo", "hello world", "plain"]
        );
        assert_eq!(split_command_line(""), Vec::<String>::new());
        assert_eq!(split_command_line("a 'b c' d"), vec!["a", "b c", "d"]);
    }

    #[test]
    fn status_bar_speed_steps_through_all_values() {
        let mut app = app();
        app.dispatch(Action::SpeedSlower);
        assert_eq!(app.settings.update_speed, "low");
        app.dispatch(Action::SpeedSlower);
        assert_eq!(app.settings.update_speed, "paused");
        app.dispatch(Action::SpeedSlower);
        assert_eq!(app.settings.update_speed, "high");
    }

    fn type_filter(app: &mut App, text: &str) {
        app.dispatch(Action::FilterStart);
        for ch in text.chars() {
            app.dispatch(Action::FilterChar(ch));
        }
        app.dispatch(Action::FilterDone);
    }

    #[test]
    fn app_history_filters_by_name_without_touching_history() {
        let mut app = app();
        app.tab = Tab::AppHistory;
        let before = app.snapshot.app_usage.len();
        type_filter(&mut app, "SAF");
        assert!(!app.filter_editing, "Enter leaves the field");
        let names: Vec<_> = app.history_rows().iter().map(|r| r.name.clone()).collect();
        assert_eq!(names, vec!["Safari"]);
        type_filter(&mut app, "zzz");
        assert!(app.history_rows().is_empty());
        let screen = crate::ui::test_util::render(&app, 120, 30);
        assert!(screen.contains("No applications match the filter."));
        assert_eq!(app.snapshot.app_usage.len(), before, "history is unchanged");
        app.dispatch(Action::FilterStart);
        app.dispatch(Action::FilterClear);
        assert_eq!(app.history_rows().len(), before);
    }

    #[test]
    fn users_filter_by_name_and_restore_the_selection() {
        let mut snapshot = crate::model::Snapshot::fixture();
        let mut other = snapshot.sessions[0].clone();
        other.user = "alice".into();
        other.uid = 777;
        snapshot.sessions.push(other);
        let mut app = app();
        app.apply_snapshot(snapshot);
        app.tab = Tab::Users;
        app.users.selected = 1;
        app.users.key = Some(777);
        type_filter(&mut app, "MICH");
        let users: Vec<_> = app.user_rows().iter().map(|r| r.user.clone()).collect();
        assert_eq!(users, vec!["michael"]);
        app.dispatch(Action::FilterStart);
        app.dispatch(Action::FilterClear);
        app.users.selected = 1;
        app.users.key = Some(777);
        // A filter that hides every row keeps the selected key.
        type_filter(&mut app, "nobody");
        let screen = crate::ui::test_util::render(&app, 120, 30);
        assert!(screen.contains("No sessions match the filter."));
        app.dispatch(Action::FilterStart);
        app.dispatch(Action::FilterClear);
        assert_eq!(app.user_rows().len(), 2);
        assert_eq!(app.users.key, Some(777), "selection restored");
    }
}
