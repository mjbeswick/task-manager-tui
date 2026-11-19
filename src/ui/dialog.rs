//! Modal panels drawn over the window.

use crate::app::{App, Dialog};
use crate::ui::chrome::{fill_area, panel, put};
use crate::ui::format::wrap;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};

const HELP: &[(&str, &str)] = &[
    ("q, Ctrl+C", "Quit"),
    ("F10", "Menu bar"),
    ("F1", "This help"),
    ("F5", "Refresh now"),
    ("1-7, Tab", "Switch tabs"),
    ("Up/Down, PgUp/PgDn", "Move and scroll"),
    ("Enter, Left, Right", "Expand, collapse, inspect"),
    ("<, >, r", "Sort column, reverse"),
    ("/", "Filter"),
    ("Delete, e, E", "End task, end tree"),
    ("f", "Fewer details"),
    ("t", "Light or dark theme"),
    ("+, -", "Update speed"),
    ("p", "Set process priority"),
    ("a", "Set affinity (unavailable)"),
    ("Space", "Toggle a startup entry"),
    ("s, S, R", "Start, stop, restart a service"),
];

pub fn draw(buf: &mut Buffer, area: Rect, app: &App, dialog: &Dialog) {
    let theme = app.theme();
    dim(buf, area, theme);

    let max_width = area.width.saturating_sub(4).min(64);
    let (content_width, content_height) = match dialog {
        Dialog::Confirm { message, .. } => {
            let lines = wrap(message, max_width.saturating_sub(6) as usize);
            (max_width, lines.len() as u16 + 5)
        }
        Dialog::Error { message, .. } => {
            let lines = wrap(message, max_width.saturating_sub(6) as usize);
            (max_width, lines.len() as u16 + 5)
        }
        Dialog::Help => (max_width.min(56), HELP.len() as u16 + 4),
        Dialog::Run { .. } => (max_width.min(52), 9),
        Dialog::Priority { .. } => (max_width.min(40), 12),
        Dialog::Properties { lines, .. } => (
            max_width,
            (lines.len() as u16 + 5).min(area.height.saturating_sub(2)),
        ),
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
            put(
                buf,
                inner.x + 2,
                inner.y + inner.height.saturating_sub(2),
                "[Esc] Close",
                Style::default().fg(theme.accent).bg(theme.surface),
                inner,
            );
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
                    &format!("{:<20}", key),
                    Style::default().fg(theme.accent).bg(theme.surface),
                    list,
                );
                put(
                    buf,
                    list.x + 22,
                    row,
                    &truncate_line(meaning, list.width.saturating_sub(23) as usize),
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
            put(
                buf,
                inner.x + 2,
                inner.y + inner.height.saturating_sub(2),
                "[Enter] Run      [Esc] Cancel",
                Style::default().fg(theme.accent).bg(theme.surface),
                inner,
            );
        }
        Dialog::Priority { name, selected, .. } => {
            put(
                buf,
                inner.x + 2,
                inner.y,
                &truncate_line(
                    &format!("Priority for {name}"),
                    inner.width.saturating_sub(3) as usize,
                ),
                Style::default().fg(theme.muted).bg(theme.surface),
                inner,
            );
            for (index, (label, nice)) in crate::model::Priority::LEVELS.iter().enumerate() {
                let row = inner.y + 1 + index as u16;
                if row >= inner.y + inner.height - 2 {
                    break;
                }
                let active = index == *selected;
                let style = if active {
                    Style::default()
                        .fg(ratatui::style::Color::White)
                        .bg(theme.accent)
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
                    &format!("{label:<14} nice {nice}"),
                    style,
                    inner,
                );
            }
            put(
                buf,
                inner.x + 2,
                inner.y + inner.height.saturating_sub(1),
                "[Enter] Set      [Esc] Cancel",
                Style::default().fg(theme.accent).bg(theme.surface),
                inner,
            );
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
            put(
                buf,
                inner.x + 2,
                inner.y + inner.height.saturating_sub(2),
                "[Esc] Close",
                Style::default().fg(theme.accent).bg(theme.surface),
                inner,
            );
        }
    }
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
