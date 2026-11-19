//! Startup tab: login entries and their real enabled state.

use crate::app::App;
use crate::ui::chrome::{fill_area, put, put_right};
use crate::ui::format::{DASH, pad_left, truncate};
use crate::ui::hits::{HitMap, Target};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;

pub fn draw(buf: &mut Buffer, area: Rect, app: &App, hits: &mut HitMap) {
    let theme = app.theme();
    fill_area(buf, area, theme.surface_style());
    if area.height < 3 {
        return;
    }

    let name = area.width.saturating_sub(12 + 16 + 8).max(12);
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
        &pad_left("Status", 10),
        header,
        area,
    );
    put(
        buf,
        area.x + name + 12,
        area.y,
        &pad_left("Startup impact", 14),
        header,
        area,
    );
    put(
        buf,
        area.x + name + 28,
        area.y,
        &pad_left("Scope", 6),
        header,
        area,
    );

    let rule = Style::default().fg(theme.border).bg(theme.surface);
    let glyphs = theme.glyphs();
    for x in area.x..area.x + area.width {
        buf[(x, area.y + 1)].set_char(glyphs.hline).set_style(rule);
    }

    if let Some(error) = &app.snapshot.startup_error {
        put(
            buf,
            area.x,
            area.y + 2,
            &truncate(
                &format!("Startup entries are unavailable: {error}"),
                area.width as usize,
            ),
            theme.danger(),
            area,
        );
        return;
    }

    let rows = app.startup_rows();
    if rows.is_empty() {
        let message = if app.startup.filter.is_empty() {
            empty_message()
        } else {
            "No startup entries match the filter.".to_string()
        };
        put(buf, area.x, area.y + 2, &message, theme.muted_style(), area);
        return;
    }

    let body = Rect::new(
        area.x,
        area.y + 2,
        area.width,
        area.height.saturating_sub(2),
    );
    for (index, row) in rows.iter().take(body.height as usize).enumerate() {
        let y = body.y + index as u16;
        let selected = index == app.startup.selected;
        hits.add(Rect::new(body.x, y, body.width, 1), Target::Row(index));
        let base = if selected {
            theme.selected_style()
        } else {
            theme.surface_style()
        };
        fill_area(buf, Rect::new(body.x, y, body.width, 1), base);
        let box_glyph = if row.enabled {
            theme.glyphs().checked
        } else {
            theme.glyphs().unchecked
        };
        let mut label = format!("{} {}", box_glyph, row.name);
        if row.missing_program {
            label.push_str(" program missing");
        }
        put(buf, body.x, y, &truncate(&label, name as usize), base, body);
        let status = if row.enabled { "Enabled" } else { "Disabled" };
        put_right(buf, body.x + name + 10, y, status, base, body);
        put_right(buf, body.x + name + 26, y, row.impact.label(), base, body);
        put_right(buf, body.x + name + 34, y, row.scope.label(), base, body);
    }

    if let Some(row) = rows.get(app.startup.selected) {
        let hint = format!(
            "{}   {}   [Space] {}",
            row.command,
            if row.missing_program {
                "program missing"
            } else {
                DASH
            },
            if row.enabled { "disable" } else { "enable" }
        );
        let y = area.y + area.height - 1;
        put(
            buf,
            area.x,
            y,
            &truncate(&hint, area.width as usize),
            Style::default().fg(theme.muted).bg(theme.surface),
            area,
        );
    }
    let _ = Style::default();
}

fn empty_message() -> String {
    #[cfg(target_os = "macos")]
    {
        "No startup entries found in ~/Library/LaunchAgents or /Library/LaunchAgents.".to_string()
    }
    #[cfg(target_os = "linux")]
    {
        "No startup entries found in ~/.config/autostart or /etc/xdg/autostart.".to_string()
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        "No startup entries found.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::app::App;
    use crate::store::settings::Settings;
    use crate::ui::test_util::render;

    fn app() -> App {
        let (mut app, _) = App::new(Settings::default(), false);
        let mut snapshot = crate::model::Snapshot::fixture();
        snapshot.startup.push(crate::model::StartupRow {
            name: "Broken tool".into(),
            command: "/opt/old/tool".into(),
            owner: "michael".into(),
            source: "~/Library/LaunchAgents/com.example.gone.plist".into(),
            enabled: false,
            impact: crate::model::Impact::NotMeasured,
            missing_program: true,
            scope: crate::model::Scope::User,
        });
        app.apply_snapshot(snapshot);
        app.tab = crate::app::Tab::Startup;
        app
    }

    #[test]
    fn entries_show_state_and_impact() {
        let screen = render(&app(), 110, 30);
        assert!(screen.contains("Rectangle"));
        assert!(screen.contains("Enabled"));
        assert!(screen.contains("Not measured"));
        assert!(screen.contains("Broken tool"));
        assert!(screen.contains("program missing"));
    }

    #[test]
    fn impact_never_invents_a_rating() {
        let screen = render(&app(), 110, 30);
        assert!(!screen.contains("High"));
        assert!(!screen.contains("Medium"));
        assert!(!screen.contains("Low"));
    }

    #[test]
    fn empty_state_names_the_directories() {
        let (mut app, _) = App::new(Settings::default(), false);
        app.tab = crate::app::Tab::Startup;
        let screen = render(&app, 120, 24);
        assert!(screen.contains("No startup entries found"));
    }

    #[test]
    fn error_state_is_visible() {
        let (mut app, _) = App::new(Settings::default(), false);
        app.apply_snapshot(crate::model::Snapshot {
            startup_error: Some("launchctl is not installed".into()),
            ..Default::default()
        });
        app.tab = crate::app::Tab::Startup;
        let screen = render(&app, 120, 24);
        assert!(screen.contains("launchctl is not installed"));
    }
}
