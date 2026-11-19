//! Users tab: interactive sessions.

use crate::app::App;
use crate::ui::chrome::{fill_area, panel, put, put_right};
use crate::ui::format::{DASH, bytes, pad_left, pct, rate, truncate};
use crate::ui::hits::{HitMap, Target};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};

pub fn draw(buf: &mut Buffer, area: Rect, app: &App, hits: &mut HitMap) {
    let theme = app.theme();
    fill_area(buf, area, theme.surface_style());
    if area.height < 3 {
        return;
    }
    let (table, panel_area) = if app.users.panel && area.height >= 16 {
        let [table, panel] =
            Layout::vertical([Constraint::Min(5), Constraint::Length(9)]).areas(area);
        (table, Some(panel))
    } else {
        (area, None)
    };

    let name = table.width.saturating_sub(14 + 8 + 10 + 12).max(10);
    let header = theme.header_style();
    fill_area(
        buf,
        Rect::new(table.x, table.y, table.width, 1),
        Style::default().bg(theme.surface),
    );
    put(
        buf,
        table.x,
        table.y,
        &truncate("User", name as usize),
        header,
        table,
    );
    put(
        buf,
        table.x + name,
        table.y,
        &pad_left("Status", 12),
        header,
        table,
    );
    put(
        buf,
        table.x + name + 14,
        table.y,
        &pad_left("CPU", 6),
        header,
        table,
    );
    put(
        buf,
        table.x + name + 22,
        table.y,
        &pad_left("Memory", 8),
        header,
        table,
    );
    put(
        buf,
        table.x + name + 32,
        table.y,
        &pad_left("Disk", 10),
        header,
        table,
    );

    let rule = Style::default().fg(theme.border).bg(theme.surface);
    let glyphs = theme.glyphs();
    for x in table.x..table.x + table.width {
        buf[(x, table.y + 1)].set_char(glyphs.hline).set_style(rule);
    }

    let rows = app.user_rows();
    if rows.is_empty() {
        put(
            buf,
            table.x,
            table.y + 2,
            "No interactive sessions. Only this application's own session is running.",
            theme.muted_style(),
            table,
        );
        return;
    }

    let body = Rect::new(
        table.x,
        table.y + 2,
        table.width,
        table.height.saturating_sub(2),
    );
    for (index, row) in rows.iter().take(body.height as usize).enumerate() {
        let y = body.y + index as u16;
        let selected = index == app.users.selected;
        hits.add(Rect::new(body.x, y, body.width, 1), Target::Row(index));
        let base = if selected {
            theme.selected_style()
        } else {
            theme.surface_style()
        };
        fill_area(buf, Rect::new(body.x, y, body.width, 1), base);
        let mut label = format!("{}  {}", row.user, row.tty);
        if row.current {
            label.push_str("  (current)");
        }
        if !row.host.is_empty() {
            label.push_str(&format!("  from {}", row.host));
        }
        put(buf, body.x, y, &truncate(&label, name as usize), base, body);
        put_right(buf, body.x + name + 12, y, row.status.label(), base, body);
        put_right(buf, body.x + name + 20, y, &pct(row.cpu), base, body);
        put_right(buf, body.x + name + 30, y, &bytes(row.memory), base, body);
        put_right(
            buf,
            body.x + name + 42,
            y,
            &rate(row.disk.io_bps()),
            base,
            body,
        );
    }

    if let Some(panel_area) = panel_area
        && let Some(session) = rows.get(app.users.selected)
    {
        {
            let inner = panel(
                buf,
                panel_area,
                &format!("Processes of {}", session.user),
                theme,
            );
            let mut list: Vec<_> = app
                .snapshot
                .processes
                .iter()
                .filter(|p| p.uid == Some(session.uid))
                .collect();
            list.sort_by(|a, b| b.cpu.total_cmp(&a.cpu));
            for (index, process) in list.iter().take(inner.height as usize).enumerate() {
                let y = inner.y + index as u16;
                let line = format!(
                    "{}  {}  {}  {}",
                    truncate(&process.name, 24),
                    process.key.pid,
                    pct(process.cpu),
                    bytes(process.memory)
                );
                put(
                    buf,
                    inner.x + 1,
                    y,
                    &truncate(&line, inner.width.saturating_sub(2) as usize),
                    Style::default().fg(theme.text).bg(theme.surface),
                    inner,
                );
            }
            if list.is_empty() {
                put(
                    buf,
                    inner.x + 1,
                    inner.y,
                    &format!("No readable processes for this session ({})", DASH),
                    Style::default().fg(theme.muted).bg(theme.surface),
                    inner,
                );
            }
        }
    }
    let _ = Modifier::BOLD;
}

#[cfg(test)]
mod tests {
    use crate::app::App;
    use crate::store::settings::Settings;
    use crate::ui::test_util::render;

    fn app() -> App {
        let (mut app, _) = App::new(Settings::default(), false);
        let mut snapshot = crate::model::Snapshot::fixture();
        let mut second = snapshot.sessions[0].clone();
        second.user = "guest".into();
        second.tty = "ttys003".into();
        second.uid = 502;
        second.status = crate::model::SessionStatus::Disconnected;
        second.current = false;
        second.host = "192.168.1.4".into();
        snapshot.sessions.push(second);
        let mut root = snapshot.sessions[0].clone();
        root.user = "root".into();
        root.uid = 0;
        root.current = false;
        snapshot.sessions.push(root);
        app.apply_snapshot(snapshot);
        app.tab = crate::app::Tab::Users;
        app
    }

    #[test]
    fn sessions_are_listed_with_status() {
        let screen = render(&app(), 100, 30);
        assert!(screen.contains("michael"));
        assert!(screen.contains("(current)"));
        assert!(screen.contains("guest"));
        assert!(screen.contains("Disconnected"));
        assert!(screen.contains("root"));
    }

    #[test]
    fn panel_lists_the_users_processes() {
        let mut app = app();
        app.users.panel = true;
        let screen = render(&app, 110, 40);
        assert!(screen.contains("Processes of michael"));
        assert!(screen.contains("Ghostty"));
    }

    #[test]
    fn empty_state_names_the_situation() {
        let (mut app, _) = App::new(Settings::default(), false);
        app.tab = crate::app::Tab::Users;
        let screen = render(&app, 100, 24);
        assert!(screen.contains("No interactive sessions"));
    }
}
