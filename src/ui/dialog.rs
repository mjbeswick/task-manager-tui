//! Modal panels drawn over the window.

use crate::app::{App, Dialog};
use crate::ui::chrome::{fill_area, panel, put, put_right};
use crate::ui::format::{DASH, bit_rate, bytes, duration, pct, rate, wrap};
use crate::ui::hits::{HitMap, Target};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};

pub(crate) enum HelpRow {
    Section(&'static str),
    Key(&'static str, &'static str),
}

pub(crate) const HELP: &[HelpRow] = &[
    HelpRow::Section("General"),
    HelpRow::Key("q, Ctrl+C", "Quit"),
    HelpRow::Key("F10, Alt+F/O/V/P/H", "Menu bar"),
    HelpRow::Key("Alt+T", "Tab list"),
    HelpRow::Key("F1, ?", "This help"),
    HelpRow::Key("F2", "Appearance: theme, style, colour, gradient"),
    HelpRow::Key("F4", "Show or hide shortcut hints"),
    HelpRow::Key("F5", "Refresh now"),
    HelpRow::Key("t", "Next colour theme"),
    HelpRow::Key("+, -", "Update speed"),
    HelpRow::Section("Tabs and filter"),
    HelpRow::Key("1-7, Tab, Shift+Tab", "Switch tabs"),
    HelpRow::Key("/, Ctrl+F", "Filter the list"),
    HelpRow::Key("Esc", "Clear the filter"),
    HelpRow::Section("Lists and moving"),
    HelpRow::Key("Up/Down, j/k", "Move one row"),
    HelpRow::Key("PgUp/PgDn, Ctrl+u/d", "Move ten rows"),
    HelpRow::Key("g / G, Home / End", "First and last row"),
    HelpRow::Key("Enter", "Process details, or expand a group"),
    HelpRow::Key("i", "Inspect, or process details"),
    HelpRow::Key("Left/Right, h/l", "Collapse, expand, inspect"),
    HelpRow::Key("<, >, r", "Sort column, reverse"),
    HelpRow::Key("x", "Clear the selected app history entry"),
    HelpRow::Section("Processes"),
    HelpRow::Key("Delete, e", "End task"),
    HelpRow::Key("E", "End process tree"),
    HelpRow::Key("K", "Send a signal"),
    HelpRow::Key("p", "Set priority by nice value"),
    HelpRow::Key("a", "Set affinity (unavailable)"),
    HelpRow::Key("f", "Fewer details"),
    HelpRow::Key("Space", "Expand a process group"),
    HelpRow::Section("Services and startup"),
    HelpRow::Key("s, S, R", "Start, stop, restart a service"),
    HelpRow::Key("Space", "Enable or disable a startup entry"),
];

pub fn draw(buf: &mut Buffer, area: Rect, app: &App, dialog: &Dialog, hits: &mut HitMap) {
    let theme = app.theme();

    let max_width = area.width.saturating_sub(4).min(72);
    let (content_width, content_height) = match dialog {
        Dialog::Confirm { message, .. } => {
            let lines = wrap(message, max_width.saturating_sub(6) as usize);
            (max_width, lines.len() as u16 + 5)
        }
        Dialog::Error { message, .. } => {
            let lines = wrap(message, max_width.saturating_sub(6) as usize);
            (max_width, lines.len() as u16 + 5)
        }
        Dialog::Help { .. } => (max_width.min(72), HELP.len() as u16 + 4),
        Dialog::About => (max_width.min(72), 12),
        Dialog::Run { .. } => (max_width.min(52), 9),
        Dialog::Priority { .. } => (max_width.min(52), 6),
        Dialog::Password { .. } => (max_width.min(56), 8),
        Dialog::Signal { .. } => (max_width.min(52), 10),
        Dialog::Appearance { .. } => (max_width.min(46), 10),
        Dialog::Properties { lines, .. } => (
            max_width,
            (lines.len() as u16 + 5).min(area.height.saturating_sub(2)),
        ),
        Dialog::ProcessInfo { key, .. } => {
            let count = app
                .snapshot
                .process(*key)
                .map(|row| process_info_lines(row).len() as u16)
                .unwrap_or(1);
            (
                max_width.min(68),
                (count + 5).min(area.height.saturating_sub(2)),
            )
        }
    };
    let width = content_width.max(24).min(area.width.saturating_sub(2));
    let height = content_height.max(5).min(area.height.saturating_sub(2));
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    let rect = Rect::new(x, y, width, height);
    let inner = panel(buf, rect, dialog.title(), theme);

    match dialog {
        Dialog::Confirm {
            message,
            confirm_label,
            ..
        } => {
            let lines = wrap(message, inner.width.saturating_sub(2) as usize);
            for (index, line) in lines.iter().enumerate() {
                let row = inner.y + 1 + index as u16;
                if row >= inner.y + inner.height {
                    break;
                }
                put(
                    buf,
                    inner.x + 2,
                    row,
                    line,
                    Style::default().fg(theme.text).bg(theme.surface),
                    inner,
                );
            }
            let buttons_y = inner.y + inner.height.saturating_sub(2);
            let buttons = format!("[Enter] {confirm_label}      [Esc] Cancel");
            put(
                buf,
                inner.x + 2,
                buttons_y,
                &buttons,
                Style::default()
                    .fg(theme.accent)
                    .bg(theme.surface)
                    .add_modifier(Modifier::BOLD),
                inner,
            );
            let confirm = format!("[Enter] {confirm_label}");
            let confirm_width = confirm.chars().count() as u16;
            hits.add(
                Rect::new(inner.x + 2, buttons_y, confirm_width, 1),
                Target::DialogConfirm,
            );
            let cancel = "[Esc] Cancel";
            let cancel_x = inner.x + 2 + confirm_width + 6;
            hits.add(
                Rect::new(cancel_x, buttons_y, cancel.chars().count() as u16, 1),
                Target::DialogCancel,
            );
        }
        Dialog::Error { message, .. } => {
            let lines = wrap(message, inner.width.saturating_sub(2) as usize);
            for (index, line) in lines.iter().enumerate() {
                let row = inner.y + 1 + index as u16;
                if row >= inner.y + inner.height {
                    break;
                }
                put(
                    buf,
                    inner.x + 2,
                    row,
                    line,
                    Style::default().fg(theme.text).bg(theme.surface),
                    inner,
                );
            }
            let y = inner.y + inner.height.saturating_sub(2);
            put(
                buf,
                inner.x + 2,
                y,
                "[Esc] Close",
                Style::default().fg(theme.accent).bg(theme.surface),
                inner,
            );
            hits.add(Rect::new(inner.x + 2, y, 11, 1), Target::DialogClose);
        }
        Dialog::About => {
            let lines = about_lines(app);
            for (index, line) in lines.iter().enumerate() {
                let row = inner.y + 1 + index as u16;
                if row >= inner.y + inner.height - 1 {
                    break;
                }
                put(
                    buf,
                    inner.x + 2,
                    row,
                    &truncate_line(line, inner.width.saturating_sub(3) as usize),
                    Style::default().fg(theme.text).bg(theme.surface),
                    inner,
                );
            }
            let y = inner.y + inner.height.saturating_sub(2);
            put(
                buf,
                inner.x + 2,
                y,
                "[Esc] Close",
                Style::default().fg(theme.accent).bg(theme.surface),
                inner,
            );
            hits.add(Rect::new(inner.x + 2, y, 11, 1), Target::DialogClose);
        }
        Dialog::Help { offset } => {
            let [list, footer] =
                Layout::vertical([Constraint::Min(0), Constraint::Length(2)]).areas(inner);
            let visible = list.height as usize;
            let max = HELP.len().saturating_sub(visible);
            let start = offset.get().min(max);
            offset.set(start);
            for (row, index) in (start..HELP.len()).take(visible).enumerate() {
                let y = list.y + row as u16;
                match HELP[index] {
                    HelpRow::Section(title) => put(
                        buf,
                        list.x + 2,
                        y,
                        title,
                        Style::default()
                            .fg(theme.muted)
                            .bg(theme.surface)
                            .add_modifier(Modifier::BOLD),
                        list,
                    ),
                    HelpRow::Key(key, meaning) => {
                        put(
                            buf,
                            list.x + 2,
                            y,
                            &format!("{:<24}", key),
                            Style::default().fg(theme.accent).bg(theme.surface),
                            list,
                        );
                        put(
                            buf,
                            list.x + 26,
                            y,
                            &truncate_line(meaning, list.width.saturating_sub(27) as usize),
                            Style::default().fg(theme.text).bg(theme.surface),
                            list,
                        );
                    }
                }
            }
            let end = (start + visible).min(HELP.len());
            let position = format!("{}-{end} of {}", start + 1, HELP.len());
            put_right(
                buf,
                footer.x + footer.width - 2,
                footer.y,
                &position,
                Style::default().fg(theme.muted).bg(theme.surface),
                footer,
            );
            put(
                buf,
                footer.x + 2,
                footer.y,
                "[Up/Down] Scroll   [PgUp/PgDn] Page   [Esc] Close",
                Style::default().fg(theme.accent).bg(theme.surface),
                footer,
            );
            hits.add(
                Rect::new(footer.x + 2, footer.y, 11, 1),
                Target::DialogClose,
            );
        }
        Dialog::Run { input } => {
            put(
                buf,
                inner.x + 2,
                inner.y + 1,
                "Type a command. Arguments run without a shell.",
                Style::default().fg(theme.muted).bg(theme.surface),
                inner,
            );
            let field = Rect::new(inner.x + 2, inner.y + 3, inner.width.saturating_sub(4), 1);
            fill_area(buf, field, Style::default().fg(theme.text).bg(theme.bg));
            let shown = truncate_line(input, field.width.saturating_sub(1) as usize);
            put(
                buf,
                field.x,
                field.y,
                &shown,
                Style::default().fg(theme.text).bg(theme.bg),
                field,
            );
            let cursor_x = field.x + shown.chars().count() as u16;
            if cursor_x < field.x + field.width {
                buf[(cursor_x, field.y)]
                    .set_char(' ')
                    .set_style(Style::default().bg(theme.accent));
            }
            let y = inner.y + inner.height.saturating_sub(2);
            put(
                buf,
                inner.x + 2,
                y,
                "[Enter] Run      [Esc] Cancel",
                Style::default().fg(theme.accent).bg(theme.surface),
                inner,
            );
            hits.add(Rect::new(inner.x + 2, y, 11, 1), Target::DialogConfirm);
            hits.add(Rect::new(inner.x + 19, y, 12, 1), Target::DialogCancel);
        }
        Dialog::Appearance { selected } => {
            let rows = [
                ("Theme", app.theme_name.label().to_string()),
                ("Chart style", app.chart_style.label().to_string()),
                ("Chart colour", app.chart_color.label().to_string()),
                (
                    "Gradient",
                    if app.chart_gradient { "On" } else { "Off" }.to_string(),
                ),
            ];
            for (index, (label, value)) in rows.iter().enumerate() {
                let row = inner.y + 1 + index as u16;
                let active = index == *selected;
                let style = if active {
                    Style::default().fg(theme.on_accent).bg(theme.accent)
                } else {
                    Style::default().fg(theme.text).bg(theme.surface)
                };
                if active {
                    for x in inner.x + 1..inner.x + inner.width - 1 {
                        buf[(x, row)].set_symbol(" ");
                        buf[(x, row)].set_style(style);
                    }
                }
                put(
                    buf,
                    inner.x + 2,
                    row,
                    &format!("{label:<14}{value}"),
                    style,
                    inner,
                );
                hits.add(
                    Rect::new(inner.x + 1, row, inner.width.saturating_sub(2), 1),
                    Target::AppearanceRow(index),
                );
            }
            let y = inner.y + inner.height.saturating_sub(1);
            put(
                buf,
                inner.x + 2,
                y,
                "[←/→] Change      [Esc] Close",
                Style::default().fg(theme.accent).bg(theme.surface),
                inner,
            );
            hits.add(Rect::new(inner.x + 2, y, 18, 1), Target::DialogConfirm);
            hits.add(Rect::new(inner.x + 22, y, 11, 1), Target::DialogCancel);
        }
        Dialog::Priority {
            name, input, error, ..
        } => {
            let muted = Style::default().fg(theme.muted).bg(theme.surface);
            let text = Style::default().fg(theme.text).bg(theme.surface);
            put(
                buf,
                inner.x + 2,
                inner.y,
                &truncate_line(
                    &format!("Priority for {name}"),
                    inner.width.saturating_sub(3) as usize,
                ),
                muted,
                inner,
            );
            let nice = input.trim().parse::<i32>().ok();
            let label = nice
                .map(|nice| crate::model::Priority { nice }.label())
                .unwrap_or("");
            let field = format!("Nice: [{}]", input);
            put(
                buf,
                inner.x + 2,
                inner.y + 1,
                &truncate_line(&field, inner.width.saturating_sub(3) as usize),
                Style::default()
                    .fg(theme.text)
                    .bg(theme.bg)
                    .add_modifier(Modifier::BOLD),
                inner,
            );
            put(
                buf,
                (inner.x + 4 + field.chars().count() as u16).min(inner.x + inner.width - 1),
                inner.y + 1,
                label,
                text,
                inner,
            );
            let hint = if *error {
                "Enter a value from -20 to 19."
            } else {
                "Range -20 to 19. Arrows 1, PgUp/PgDn 5."
            };
            put(
                buf,
                inner.x + 2,
                inner.y + 2,
                &truncate_line(hint, inner.width.saturating_sub(3) as usize),
                if *error {
                    Style::default().fg(theme.bad).bg(theme.surface)
                } else {
                    muted
                },
                inner,
            );
            let y = inner.y + inner.height.saturating_sub(1);
            put(
                buf,
                inner.x + 2,
                y,
                "[Enter] Set      [Esc] Cancel",
                Style::default().fg(theme.accent).bg(theme.surface),
                inner,
            );
            hits.add(Rect::new(inner.x + 2, y, 11, 1), Target::DialogConfirm);
            hits.add(Rect::new(inner.x + 19, y, 12, 1), Target::DialogCancel);
        }
        Dialog::Signal {
            name,
            pid,
            selected,
            tree,
            ..
        } => {
            put(
                buf,
                inner.x + 2,
                inner.y,
                &truncate_line(
                    &format!("Signal for {name} (PID {pid})"),
                    inner.width.saturating_sub(3) as usize,
                ),
                Style::default().fg(theme.muted).bg(theme.surface),
                inner,
            );
            for (index, signal) in crate::model::Signal::ALL.iter().enumerate() {
                let row = inner.y + 1 + index as u16;
                if row >= inner.y + inner.height - 1 {
                    break;
                }
                let active = index == *selected;
                let style = if active {
                    Style::default().fg(theme.on_accent).bg(theme.accent)
                } else {
                    Style::default().fg(theme.text).bg(theme.surface)
                };
                if active {
                    for x in inner.x + 1..inner.x + inner.width - 1 {
                        buf[(x, row)].set_symbol(" ");
                        buf[(x, row)].set_style(style);
                    }
                }
                put(
                    buf,
                    inner.x + 2,
                    row,
                    &format!(
                        "{:<5}{:<12}{}",
                        signal.name(),
                        signal.label(),
                        signal.number()
                    ),
                    style,
                    inner,
                );
                hits.add(
                    Rect::new(inner.x + 1, row, inner.width.saturating_sub(2), 1),
                    Target::DialogSignal(index),
                );
            }
            let y = inner.y + inner.height.saturating_sub(1);
            let footer = format!(
                "[Enter] Send   [Space] Tree: {}   [Esc] Cancel",
                if *tree { "on" } else { "off" }
            );
            put(
                buf,
                inner.x + 2,
                y,
                &truncate_line(&footer, inner.width.saturating_sub(3) as usize),
                Style::default().fg(theme.accent).bg(theme.surface),
                inner,
            );
            hits.add(Rect::new(inner.x + 2, y, 13, 1), Target::DialogConfirm);
            hits.add(Rect::new(inner.x + 17, y, 18, 1), Target::DialogConfirm);
            hits.add(Rect::new(inner.x + 37, y, 12, 1), Target::DialogCancel);
        }
        Dialog::Password {
            action,
            input,
            error,
        } => {
            let muted = Style::default().fg(theme.muted).bg(theme.surface);
            let text = Style::default().fg(theme.text).bg(theme.surface);
            put(
                buf,
                inner.x + 2,
                inner.y,
                &truncate_line(&action.describe(), inner.width.saturating_sub(3) as usize),
                text,
                inner,
            );
            let masked = "*".repeat(input.chars().count());
            let field = format!("Password: {masked}");
            put(
                buf,
                inner.x + 2,
                inner.y + 1,
                &truncate_line(&field, inner.width.saturating_sub(3) as usize),
                Style::default()
                    .fg(theme.text)
                    .bg(theme.bg)
                    .add_modifier(Modifier::BOLD),
                inner,
            );
            let hint = if *error {
                "Enter the password."
            } else {
                "Used once. Never stored."
            };
            put(
                buf,
                inner.x + 2,
                inner.y + 2,
                &truncate_line(hint, inner.width.saturating_sub(3) as usize),
                if *error {
                    Style::default().fg(theme.bad).bg(theme.surface)
                } else {
                    muted
                },
                inner,
            );
            let y = inner.y + inner.height.saturating_sub(1);
            put(
                buf,
                inner.x + 2,
                y,
                "[Enter] Elevate      [Esc] Cancel",
                Style::default().fg(theme.accent).bg(theme.surface),
                inner,
            );
            hits.add(Rect::new(inner.x + 2, y, 16, 1), Target::DialogConfirm);
            hits.add(Rect::new(inner.x + 24, y, 12, 1), Target::DialogCancel);
        }
        Dialog::ProcessInfo { key, .. } => {
            let lines = match app.snapshot.process(*key) {
                Some(row) => process_info_lines(row),
                None => vec!["This process has exited.".to_string()],
            };
            for (index, line) in lines.iter().enumerate() {
                let row = inner.y + 1 + index as u16;
                if row >= inner.y + inner.height - 1 {
                    break;
                }
                put(
                    buf,
                    inner.x + 2,
                    row,
                    &truncate_line(line, inner.width.saturating_sub(3) as usize),
                    Style::default().fg(theme.text).bg(theme.surface),
                    inner,
                );
            }
            let y = inner.y + inner.height.saturating_sub(2);
            put(
                buf,
                inner.x + 2,
                y,
                "[Esc] Close",
                Style::default().fg(theme.accent).bg(theme.surface),
                inner,
            );
            hits.add(Rect::new(inner.x + 2, y, 11, 1), Target::DialogClose);
        }
        Dialog::Properties { lines, .. } => {
            for (index, line) in lines.iter().enumerate() {
                let row = inner.y + 1 + index as u16;
                if row >= inner.y + inner.height - 1 {
                    break;
                }
                put(
                    buf,
                    inner.x + 2,
                    row,
                    &truncate_line(line, inner.width.saturating_sub(3) as usize),
                    Style::default().fg(theme.text).bg(theme.surface),
                    inner,
                );
            }
            let y = inner.y + inner.height.saturating_sub(2);
            put(
                buf,
                inner.x + 2,
                y,
                "[Esc] Close",
                Style::default().fg(theme.accent).bg(theme.surface),
                inner,
            );
            hits.add(Rect::new(inner.x + 2, y, 11, 1), Target::DialogClose);
        }
    }
}

fn about_lines(app: &App) -> Vec<String> {
    let path = |path: Option<std::path::PathBuf>| {
        path.map(|path| path.display().to_string())
            .unwrap_or_else(|| DASH.to_string())
    };
    vec![
        format!("Task Manager {}", env!("CARGO_PKG_VERSION")),
        format!("Platform: {}", std::env::consts::OS),
        format!("Theme: {}", app.theme_name.label()),
        format!("Colour depth: {}", app.depth.label()),
        format!("Settings: {}", path(crate::store::settings::config_path())),
        format!(
            "History: {}",
            path(crate::store::history::History::data_path())
        ),
    ]
}

fn process_info_lines(row: &crate::model::ProcRow) -> Vec<String> {
    let dash = || DASH.to_string();
    vec![
        format!("Name: {}", row.name),
        format!("PID: {}", row.key.pid),
        format!("Kind: {}", row.kind.label()),
        format!("Status: {}", row.status.label()),
        format!("Owner: {}", row.user.clone().unwrap_or_else(dash)),
        format!("Priority: {}", row.priority.label()),
        format!(
            "Threads: {}",
            row.thread_count()
                .map(|count| count.to_string())
                .unwrap_or_else(dash)
        ),
        format!("Started: {}", duration(row.run_time_secs)),
        format!("CPU: {}", pct(row.cpu)),
        format!("Memory: {}", bytes(row.memory)),
        format!(
            "Disk: {} read / {} write",
            rate(row.disk.read_bps),
            rate(row.disk.write_bps)
        ),
        format!(
            "Network: {}",
            row.network.map(bit_rate).unwrap_or_else(dash)
        ),
        format!("GPU: {}", row.gpu.map(pct).unwrap_or_else(dash)),
        format!(
            "Parent PID: {}",
            row.parent_pid
                .map(|pid| pid.to_string())
                .unwrap_or_else(dash)
        ),
        format!(
            "Command line: {}",
            if row.cmd.is_empty() {
                dash()
            } else {
                row.cmd.clone()
            }
        ),
        format!("Executable: {}", row.exe.clone().unwrap_or_else(dash)),
    ]
}

fn truncate_line(text: &str, width: usize) -> String {
    text.chars().take(width).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::App;
    use crate::store::settings::Settings;
    use crate::ui::test_util::render;

    fn app() -> App {
        let (mut app, _) = App::new(Settings::default(), false);
        app.apply_snapshot(crate::model::Snapshot::fixture());
        app
    }

    #[test]
    fn confirm_dialog_names_the_action() {
        let mut app = app();
        app.dialog = Some(Dialog::Confirm {
            title: "End task".into(),
            message: "End Ghostty (PID 4242)? Unsaved work will be lost.".into(),
            confirm_label: "End task".into(),
            action: crate::app::PendingAction::ClearHistory,
        });
        let screen = render(&app, 100, 30);
        assert!(screen.contains("End task"));
        assert!(screen.contains("Ghostty"));
        assert!(screen.contains("[Enter] End task"));
        assert!(screen.contains("[Esc] Cancel"));
    }

    #[test]
    fn help_lists_the_keys() {
        let mut app = app();
        app.dialog = Some(Dialog::Help {
            offset: std::cell::Cell::new(0),
        });
        let screen = render(&app, 100, 40);
        assert!(screen.contains("Keyboard shortcuts"));
        assert!(screen.contains("End task"));
        assert!(screen.contains("End process tree"));
        assert!(screen.contains("Inspect"));
        assert!(screen.contains("gradient"));
        assert!(screen.contains("F10"));
    }

    #[test]
    fn run_dialog_shows_the_input() {
        let mut app = app();
        app.dialog = Some(Dialog::Run {
            input: "open -a Calculator".into(),
        });
        let screen = render(&app, 100, 30);
        assert!(screen.contains("Run a command"));
        assert!(screen.contains("open -a Calculator"));
        assert!(screen.contains("[Enter] Run"));
    }

    #[test]
    fn process_info_shows_live_metrics_and_exit_state() {
        let mut app = app();
        let row = app
            .snapshot
            .processes
            .iter()
            .find(|row| row.kind == crate::model::Kind::Background)
            .unwrap()
            .clone();
        app.dialog = Some(Dialog::ProcessInfo {
            key: row.key,
            name: row.name.clone(),
        });
        let screen = render(&app, 120, 40);
        assert!(screen.contains("Process details"));
        assert!(screen.contains("Command line:"));
        assert!(screen.contains("CPU:"));
        assert!(screen.contains("[Esc] Close"));

        app.apply_snapshot(crate::model::Snapshot::default());
        let screen = render(&app, 120, 40);
        assert!(screen.contains("This process has exited."));
    }

    #[test]
    fn password_dialog_masks_the_input() {
        let mut app = app();
        app.dialog = Some(Dialog::Password {
            action: crate::platform::ElevatedAction::Signal {
                pids: vec![42],
                signal: crate::model::Signal::Term,
            },
            input: "hunter2".into(),
            error: false,
        });
        let screen = render(&app, 100, 30);
        assert!(screen.contains("Administrator password"));
        assert!(screen.contains("*******"));
        assert!(!screen.contains("hunter2"));
        assert!(screen.contains("Used once. Never stored."));
    }

    #[test]
    fn about_shows_version_paths_and_appearance() {
        let mut app = app();
        app.dialog = Some(Dialog::About);
        let screen = render(&app, 120, 40);
        assert!(screen.contains("About"));
        assert!(screen.contains("Task Manager"));
        assert!(screen.contains("Platform:"));
        assert!(screen.contains("Theme:"));
        assert!(screen.contains("Colour depth:"));
        assert!(screen.contains("Settings:"));
        assert!(screen.contains("History:"));
        assert!(screen.contains("[Esc] Close"));
    }

    #[test]
    fn properties_lists_lines() {
        let mut app = app();
        app.dialog = Some(Dialog::Properties {
            title: "Properties".into(),
            lines: vec!["Name: nginx.service".into(), "Status: Running".into()],
        });
        let screen = render(&app, 100, 30);
        assert!(screen.contains("nginx.service"));
        assert!(screen.contains("Status: Running"));
    }

    #[test]
    fn error_dialog_wraps() {
        let mut app = app();
        app.dialog = Some(Dialog::Error {
            title: "Could not end task".into(),
            message: "Operation not permitted. Ending a process owned by another user needs administrator rights."
                .into(),
        });
        let screen = render(&app, 90, 30);
        assert!(screen.contains("Could not end task"));
        assert!(screen.contains("administrator"));
    }
}
