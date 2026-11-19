//! Services tab: the native manager's units or jobs.

use crate::app::App;
use crate::ui::chrome::{fill_area, put, put_right};
use crate::ui::format::{DASH, pad_left, truncate};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;

pub fn draw(buf: &mut Buffer, area: Rect, app: &App) {
    let theme = app.theme();
    fill_area(buf, area, theme.surface_style());
    if area.height < 3 {
        return;
    }

    let name = area.width.saturating_sub(7 + 30 + 12 + 8).max(12);
    let header = theme.header_style();
    fill_area(
        buf,
        Rect::new(area.x, area.y, area.width, 1),
        Style::default().bg(theme.surface),
    );
    put(
        buf,
        area.x,
        area.y,
        &truncate("Name", name as usize),
        header,
        area,
    );
    put(
        buf,
        area.x + name,
        area.y,
        &pad_left("PID", 5),
        header,
        area,
    );
    put(
        buf,
        area.x + name + 7,
        area.y,
        &truncate("Description", 30),
        header,
        area,
    );
    put(
        buf,
        area.x + name + 39,
        area.y,
        &pad_left("Status", 10),
        header,
        area,
    );
    put(
        buf,
        area.x + name + 51,
        area.y,
        &pad_left("Group", 6),
        header,
        area,
    );

    let rule = Style::default().fg(theme.border).bg(theme.surface);
    let glyphs = theme.glyphs();
    for x in area.x..area.x + area.width {
        buf[(x, area.y + 1)].set_char(glyphs.hline).set_style(rule);
    }

    if let Some(error) = &app.snapshot.services_error {
        put(
            buf,
            area.x,
            area.y + 2,
            &truncate(
                &format!("Services are unavailable: {error}"),
                area.width as usize,
            ),
            theme.danger(),
            area,
        );
        return;
    }

    let rows = app.service_rows();
    if rows.is_empty() {
        let message = if app.services.filter.is_empty() {
            "No services reported by the system's service manager."
        } else {
            "No services match the filter."
        };
        put(buf, area.x, area.y + 2, message, theme.muted_style(), area);
        return;
    }

    let body = Rect::new(
        area.x,
        area.y + 2,
        area.width,
        area.height.saturating_sub(2),
    );
    let selected = app.services.selected;
    let visible = body.height as usize;
    let scroll = if selected < app.services.scroll {
        selected
    } else if selected >= app.services.scroll + visible {
        selected + 1 - visible
    } else {
        app.services.scroll
    };
    let scroll = scroll.min(rows.len().saturating_sub(visible));

    for (offset, row) in rows.iter().skip(scroll).take(visible).enumerate() {
        let y = body.y + offset as u16;
        let is_selected = scroll + offset == selected;
        let base = if is_selected {
            theme.selected_style()
        } else {
            theme.surface_style()
        };
        fill_area(buf, Rect::new(body.x, y, body.width, 1), base);
        put(
            buf,
            body.x,
            y,
            &truncate(&row.name, name as usize),
            base,
            body,
        );
        let pid = row
            .pid
            .map(|p| p.to_string())
            .unwrap_or_else(|| DASH.to_string());
        put_right(buf, body.x + name + 6, y, &pid, base, body);
        let description = row.description.clone().unwrap_or_else(|| DASH.to_string());
        put(
            buf,
            body.x + name + 7,
            y,
            &truncate(&description, 29),
            base,
            body,
        );
        put_right(buf, body.x + name + 49, y, row.state.label(), base, body);
        put_right(buf, body.x + name + 57, y, row.scope.label(), base, body);
    }

    if let Some(row) = rows.get(selected) {
        let hint = format!(
            "[s] start   [S] stop   [R] restart   {}",
            if row.scope == crate::model::Scope::System {
                "system scope needs administrator rights"
            } else {
                "user scope"
            }
        );
        put(
            buf,
            area.x,
            area.y + area.height - 1,
            &truncate(&hint, area.width as usize),
            Style::default().fg(theme.muted).bg(theme.surface),
            area,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::app::App;
    use crate::store::settings::Settings;
    use crate::ui::test_util::render;

    fn app() -> App {
        let (mut app, _) = App::new(Settings::default(), false);
        app.apply_snapshot(crate::model::Snapshot::fixture());
        app.tab = crate::app::Tab::Services;
        app
    }

    #[test]
    fn services_show_state_and_group() {
        let screen = render(&app(), 120, 30);
        assert!(screen.contains("com.apple.metadata.mds"));
        assert!(screen.contains("Running"));
        assert!(screen.contains("homebrew.mxcl.nginx"));
        assert!(screen.contains("Stopped"));
        assert!(screen.contains("system"));
        assert!(screen.contains("user"));
    }

    #[test]
    fn stopped_services_have_no_pid() {
        let screen = render(&app(), 120, 30);
        let line = screen
            .lines()
            .find(|line| line.contains("homebrew.mxcl.nginx"))
            .unwrap();
        assert!(line.contains('-'));
    }

    #[test]
    fn error_state_is_visible() {
        let (mut app, _) = App::new(Settings::default(), false);
        app.apply_snapshot(crate::model::Snapshot {
            services_error: Some("systemctl is not installed".into()),
            ..Default::default()
        });
        app.tab = crate::app::Tab::Services;
        let screen = render(&app, 120, 24);
        assert!(screen.contains("systemctl is not installed"));
    }

    #[test]
    fn empty_state_is_clear() {
        let (mut app, _) = App::new(Settings::default(), false);
        app.tab = crate::app::Tab::Services;
        let screen = render(&app, 120, 24);
        assert!(screen.contains("No services reported"));
    }
}
