//! Actions to effects. The interface never signals a process itself; it returns
//! work for the main loop, which keeps this layer testable.

use crate::app::{App, Dialog, Effect, MENUS, MenuState, PendingAction, Tab};
use crate::input::Action;
use crate::model::{Kind, ProcKey, ProcRow, ProcSel, Signal, Status};
use crate::platform::ServiceAction;
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
                    if service_action == ServiceAction::Start {
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
            | Action::PriorityStep(_) => {}
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
                    *selected = index.min(2);
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
        let Some(row) = self.selected_process() else {
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

    fn submit_signal(&mut self, key: ProcKey, signal: Signal, tree: bool) -> Vec<Effect> {
        let mut effects = Vec::new();
        let row = self.snapshot.process(key);
        let needs_confirm = signal.requires_confirm()
            || row.is_some_and(|row| {
                row.kind == Kind::System || row.uid != Some(crate::platform::current_uid())
            });
        if needs_confirm {
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
            Tab::Details => {
                self.details.panel = !self.details.panel;
            }
            Tab::Users => {
                self.users.panel = !self.users.panel;
            }
            _ => self.show_properties(),
        }
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
        let needs_confirm =
            row.kind == Kind::System || row.uid != Some(crate::platform::current_uid());
        if needs_confirm {
            self.dialog = Some(Dialog::Confirm {
                title: "End task".to_string(),
                message: format!(
                    "End {} (PID {})? This process belongs to {}, so ending it may need administrator rights.",
                    row.name,
                    key.pid,
                    row.user.clone().unwrap_or_else(|| "another user".into())
                ),
                confirm_label: "End task".to_string(),
                action: PendingAction::EndTask(key),
            });
        } else {
            effects.push(Effect::Signal {
                key,
                signal: Signal::Term,
                tree: false,
            });
        }
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
            Action::MoveUp | Action::MoveDown => {
                let delta = if action == Action::MoveUp { -1 } else { 1 };
                match &mut self.dialog {
                    Some(Dialog::Signal { selected, .. }) => {
                        let count = Signal::ALL.len() as isize;
                        *selected = (*selected as isize + delta).rem_euclid(count) as usize;
                    }
                    Some(Dialog::Appearance { selected }) => {
                        *selected = (*selected as isize + delta).rem_euclid(3) as usize;
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
                                + 2)
                                % 3]
                        };
                        self.set_chart_style(style);
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
            MenuItem::ClearHistory => Action::DeleteHistory,
            MenuItem::Help => Action::Help,
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
            [Effect::Signal {
                signal: Signal::Term,
                tree: false,
                ..
            }]
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
            [Effect::Service {
                action: ServiceAction::Stop,
                ..
            }]
        ));
    }

    #[test]
    fn starting_a_service_runs_without_a_confirmation() {
        let mut app = app();
        app.tab = Tab::Services;
        app.sync_services_public();
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
    fn signal_on_a_kind_heading_only_notices() {
        let mut app = app();
        app.tab = Tab::Processes;
        app.collapse_all();
        assert!(matches!(app.process.selected_key(), Some(ProcSel::Kind(_))));
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
