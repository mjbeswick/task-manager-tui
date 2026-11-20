//! Modal panels drawn over the window.

use crate::app::{App, Dialog};
use crate::ui::chrome::{fill_area, panel, put};
use crate::ui::format::{DASH, bit_rate, bytes, duration, pct, rate, wrap};
use crate::ui::hits::{HitMap, Target};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};

const HELP: &[(&str, &str)] = &[
    ("q, Ctrl+C", "Quit"),
    ("F10, Alt+F/O/V/H", "Menu bar"),
    ("F1", "This help"),
    ("F2", "Appearance: theme, style, colour"),
    ("F5", "Refresh now"),
    ("1-7, Tab", "Switch tabs"),
    ("Up/Down, j/k, PgUp/PgDn", "Move and scroll"),
    ("g / G, Ctrl+d / Ctrl+u", "Top, bottom, half page"),
    ("Enter", "Process details, or expand a group"),
    ("Left/Right, h/l", "Collapse, expand, inspect"),
    ("Mouse", "Click away closes a menu; wheel scrolls"),
    ("<, >, r", "Sort column, reverse"),
    ("/", "Filter"),
    ("Delete, e, E", "End task, end tree"),
    ("K", "Send a signal"),
    ("f", "Fewer details"),
    ("t", "Next colour theme"),
    ("+, -", "Update speed"),
    ("p", "Set priority by nice value"),
    ("a", "Set affinity (unavailable)"),
    ("Space", "Toggle a startup entry"),
    ("s, S, R", "Start, stop, restart a service"),
];

pub fn draw(buf: &mut Buffer, area: Rect, app: &App, dialog: &Dialog, hits: &mut HitMap) {
    let theme = app.theme();
    dim(buf, area, theme);

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
        Dialog::Help => (max_width.min(72), HELP.len() as u16 + 4),
        Dialog::Run { .. } => (max_width.min(52), 9),
        Dialog::Priority { .. } => (max_width.min(52), 6),
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
        Dialog::Help => {
            let [list, footer] =
                Layout::vertical([Constraint::Min(0), Constraint::Length(2)]).areas(inner);
            for (index, (key, meaning)) in HELP.iter().enumerate() {
                let row = list.y + index as u16;
                if row >= list.y + list.height {
                    break;
                }
                put(
                    buf,
                    list.x + 2,
                    row,
                    &format!("{:<24}", key),
                    Style::default().fg(theme.accent).bg(theme.surface),
                    list,
                );
                put(
                    buf,
                    list.x + 26,
                    row,
                    &truncate_line(meaning, list.width.saturating_sub(27) as usize),
                    Style::default().fg(theme.text).bg(theme.surface),
                    list,
                );
            }
            put(
                buf,
                footer.x + 2,
                footer.y,
                "[Esc] Close",
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
                "Range -20 to 19. Up/Down 1, PgUp/PgDn 5."
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

fn process_info_lines(row: &crate::model::ProcRow) -> Vec<String> {
    let dash = || DASH.to_string();
    vec![
        format!("Name: {}", row.name),
        format!("PID: {}", row.key.pid),
        format!("Kind: {}", row.kind.label()),
        format!("Status: {}", row.status.label()),
        format!("Owner: {}", row.user.clone().unwrap_or_else(|| dash())),
        format!("Priority: {}", row.priority.label()),
        format!(
            "Threads: {}",
            row.thread_count()
                .map(|count| count.to_string())
                .unwrap_or_else(|| dash())
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
            row.network.map(bit_rate).unwrap_or_else(|| dash())
        ),
        format!("GPU: {}", row.gpu.map(pct).unwrap_or_else(|| dash())),
        format!(
            "Parent PID: {}",
            row.parent_pid
                .map(|pid| pid.to_string())
                .unwrap_or_else(|| dash())
        ),
        format!(
            "Command line: {}",
            if row.cmd.is_empty() {
                dash()
            } else {
                row.cmd.clone()
            }
        ),
        format!("Executable: {}", row.exe.clone().unwrap_or_else(|| dash())),
    ]
}

fn truncate_line(text: &str, width: usize) -> String {
    text.chars().take(width).collect()
}

/// Darken the window behind a modal so focus is obvious.
fn dim(buf: &mut Buffer, area: Rect, theme: &crate::theme::Theme) {
    let style = Style::default().fg(theme.muted).bg(theme.bg);
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            let fg = buf[(x, y)].fg;
            let _ = style;
            buf[(x, y)].set_style(Style::default().fg(fg).bg(theme.bg));
        }
    }
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
        app.dialog = Some(Dialog::Help);
        let screen = render(&app, 100, 40);
        assert!(screen.contains("Keyboard shortcuts"));
        assert!(screen.contains("End task, end tree"));
        assert!(screen.contains("F10"));
    }

    #[test]
    fn run_dialog_shows_the_input() {
        let mut app = app();
        app.dialog = Some(Dialog::Run {
            input: "open -a Calculator".into(),
        });
        let screen = render(&app, 100, 30);
        assert!(screen.contains("Run a new task"));
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
