//! Users tab: interactive sessions.

use crate::app::App;
use crate::ui::chrome::{fill_area, panel, put, put_match, put_right};
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
    let content = area;
    let hint = crate::ui::chrome::hint_area(area);
    let area = crate::ui::chrome::without_hint(area, hint);
    if let Some(hint) = hint {
        crate::ui::chrome::draw_hint(buf, content, app, hint);
    }
    let rows = app.user_rows();
    let (table, panel_area) = if app.users.panel && area.height >= 16 {
        let [table, panel] =
            Layout::vertical([Constraint::Min(5), Constraint::Length(9)]).areas(area);
        (table, Some(panel))
    } else {
        (area, None)
    };
    let body_height = table.height.saturating_sub(2);
    let bar = crate::ui::chrome::scrollbar_rect(table, table.y + 2, body_height, rows.len());
    let table = if bar.is_some() {
        Rect::new(
            table.x,
            table.y,
            table.width.saturating_sub(1),
            table.height,
        )
    } else {
        table
    };

    let session = 10;
    let status = 14;
    let cpu = 8;
    let memory = 10;
    let disk = 12;
    let name = table
        .width
        .saturating_sub(session + status + cpu + memory + disk)
        .max(8);
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
        &truncate("Session", session as usize),
        header,
        table,
    );
    put(
        buf,
        table.x + name + session,
        table.y,
        &pad_left("Status", (status - 2) as usize),
        header,
        table,
    );
    put(
        buf,
        table.x + name + session + status,
        table.y,
        &pad_left("CPU", (cpu - 2) as usize),
        header,
        table,
    );
    put(
        buf,
        table.x + name + session + status + cpu,
        table.y,
        &pad_left("Memory", (memory - 2) as usize),
        header,
        table,
    );
    put(
        buf,
        table.x + name + session + status + cpu + memory,
        table.y,
        &pad_left("Disk", (disk - 2) as usize),
        header,
        table,
    );

    let rule = Style::default().fg(theme.border).bg(theme.surface);
    let glyphs = theme.glyphs();
    for x in table.x..table.x + table.width {
        buf[(x, table.y + 1)].set_char(glyphs.hline).set_style(rule);
    }

    if rows.is_empty() {
        put(
            buf,
            table.x,
            table.y + 2,
            if app.users.filter.trim().is_empty() {
                "No interactive sessions. Only this application's own session is running."
            } else {
                "No sessions match the filter."
            },
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
    app.users.visible.set(body.height as usize);
    hits.add(body, Target::ScrollZone(crate::app::Tab::Users));
    let scroll = app
        .users
        .scroll
        .min(rows.len().saturating_sub(body.height as usize));
    for (index, row) in rows
        .iter()
        .skip(scroll)
        .take(body.height as usize)
        .enumerate()
    {
        let y = body.y + index as u16;
        let index = scroll + index;
        let selected = index == app.users.selected;
        hits.add(Rect::new(body.x, y, body.width, 1), Target::Row(index));
        let base = theme.row_style(selected, index);
        fill_area(buf, Rect::new(body.x, y, body.width, 1), base);
        let mut user = row.user.clone();
        if row.current {
            user.push_str("  (current)");
        }
        put_match(
            buf,
            body.x,
            y,
            &truncate(&user, name as usize),
            base,
            body,
            app,
        );
        put(
            buf,
            body.x + name,
            y,
            &truncate(
                &if row.others > 0 {
                    format!("{} +{}", row.tty, row.others)
                } else {
                    row.tty.clone()
                },
                session as usize,
            ),
            base,
            body,
        );
        let status_x = body.x + name + session;
        put_right(
            buf,
            status_x + status - 2,
            y,
            row.status.label(),
            base,
            body,
        );
        let cpu_x = status_x + status;
        put_right(buf, cpu_x + cpu - 2, y, &pct(row.cpu), base, body);
        let memory_x = cpu_x + cpu;
        put_right(
            buf,
            memory_x + memory - 2,
            y,
            &bytes(row.memory),
            base,
            body,
        );
        let disk_x = memory_x + memory;
        put_right(
            buf,
            disk_x + disk - 2,
            y,
            &rate(row.disk.io_bps()),
            base,
            body,
        );
    }
    if let Some(bar) = bar {
        crate::ui::chrome::draw_scrollbar(
            buf,
            theme,
            hits,
            crate::ui::chrome::ScrollbarSpec {
                rect: bar,
                len: rows.len(),
                visible: body.height as usize,
                viewport: scroll,
                tab: crate::app::Tab::Users,
            },
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

    #[test]
    fn merged_sessions_show_a_count() {
        let (mut app, _) = App::new(crate::store::settings::Settings::default(), false);
        let mut snapshot = crate::model::Snapshot::fixture();
        snapshot.sessions[0].tty = "pts/0".into();
        snapshot.sessions[0].others = 3;
        app.apply_snapshot(snapshot);
        app.tab = crate::app::Tab::Users;
        let screen = crate::ui::test_util::render(&app, 120, 30);
        assert!(screen.contains("pts/0 +3"), "{screen}");
        assert_eq!(screen.matches("michael").count(), 1, "one row for the user");
    }
}
