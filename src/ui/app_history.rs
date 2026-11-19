//! App history tab: accumulated usage per application.

use crate::app::App;
use crate::ui::chrome::{fill_area, put, put_match, put_right};
use crate::ui::format::{DASH, bytes, cpu_time, pad_left, truncate};
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

    let rows = app.history_rows();
    let body_height = area.height.saturating_sub(2);
    let bar = crate::ui::chrome::scrollbar_rect(area, area.y + 2, body_height, rows.len());
    let area = if bar.is_some() {
        Rect::new(area.x, area.y, area.width.saturating_sub(1), area.height)
    } else {
        area
    };
    // Title, width, right-aligned.
    let columns: [(&str, u16, bool); 5] = [
        ("CPU time", 11, true),
        ("Peak memory", 14, true),
        ("Disk", 21, true),
        ("Last seen", 10, true),
        ("Network", 9, true),
    ];
    let used: u16 = columns.iter().map(|(_, width, _)| *width).sum();
    let name = area.width.saturating_sub(used).max(10);

    let header = theme.header_style();
    fill_area(
        buf,
        Rect::new(area.x, area.y, area.width, 1),
        Style::default().bg(theme.surface),
    );
    use crate::sample::group::{SortDir, SortKey};
    let sort = app.app_history.sort;
    let arrow = if sort.dir == SortDir::Asc {
        theme.glyphs().sort_asc
    } else {
        theme.glyphs().sort_desc
    };
    let keys = [
        Some(SortKey::Cpu),
        Some(SortKey::Memory),
        Some(SortKey::Disk),
        None,
        None,
    ];
    let marked = |title: &str, key: Option<SortKey>| match key {
        Some(key) if key == sort.key => format!("{arrow} {title}"),
        _ => title.to_string(),
    };
    let mut x = area.x;
    put(
        buf,
        x,
        area.y,
        &truncate(&marked("Name", Some(SortKey::Name)), name as usize),
        header,
        area,
    );
    hits.header(
        area.x,
        area.y,
        1,
        &[
            (Some(name), SortKey::Name),
            (Some(columns[0].1), SortKey::Cpu),
            (Some(columns[1].1), SortKey::Memory),
            (Some(columns[2].1), SortKey::Disk),
        ],
    );
    x += name;
    for ((title, width, right), key) in columns.into_iter().zip(keys) {
        let title = marked(title, key);
        let title = title.as_str();
        if right {
            put(
                buf,
                x,
                area.y,
                &pad_left(title, width as usize - 1),
                header,
                area,
            );
        } else {
            put(
                buf,
                x,
                area.y,
                &truncate(title, width as usize),
                header,
                area,
            );
        }
        x += width;
    }

    let rule = Style::default().fg(theme.border).bg(theme.surface);
    let glyphs = theme.glyphs();
    for x in area.x..area.x + area.width {
        buf[(x, area.y + 1)].set_char(glyphs.hline).set_style(rule);
    }

    if rows.is_empty() {
        put(
            buf,
            area.x,
            area.y + 2,
            if app.app_history.filter.trim().is_empty() {
                "No usage recorded yet. Totals appear as applications run."
            } else {
                "No applications match the filter."
            },
            theme.muted_style(),
            area,
        );
        return;
    }

    let now = crate::store::history::now_epoch();
    let body = Rect::new(
        area.x,
        area.y + 2,
        area.width,
        area.height.saturating_sub(2),
    );
    app.app_history.visible.set(body.height as usize);
    hits.add(body, Target::ScrollZone(crate::app::Tab::AppHistory));
    let scroll = app
        .app_history
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
        let selected = index == app.app_history.selected;
        hits.add(Rect::new(body.x, y, body.width, 1), Target::Row(index));
        let base = theme.row_style(selected, index);
        fill_area(buf, Rect::new(body.x, y, body.width, 1), base);
        let mut x = body.x;
        put_match(
            buf,
            x,
            y,
            &truncate(&row.name, name as usize),
            base,
            body,
            app,
        );
        x += name;
        // Figures end where their right-aligned titles end: one cell short of
        // the column edge.
        put_right(buf, x + 10, y, &cpu_time(row.cpu_ms), base, body);
        x += 11;
        put_right(buf, x + 13, y, &bytes(row.peak_memory), base, body);
        x += 14;
        let disk = format!("{} / {}", bytes(row.disk_read), bytes(row.disk_written));
        put_right(buf, x + 20, y, &disk, base, body);
        x += 21;
        let last_seen = if row.last_seen == 0 {
            DASH.to_string()
        } else {
            ago(now.saturating_sub(row.last_seen))
        };
        put_right(buf, x + 9, y, &last_seen, base, body);
        x += 10;
        let network = row.network.map(bytes).unwrap_or_else(|| DASH.to_string());
        put_right(buf, x + 8, y, &network, base, body);
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
                tab: crate::app::Tab::AppHistory,
            },
        );
    }
}

pub fn ago(seconds: u64) -> String {
    if seconds < 60 {
        "just now".to_string()
    } else if seconds < 3_600 {
        format!("{}m ago", seconds / 60)
    } else if seconds < 86_400 {
        format!("{}h ago", seconds / 3_600)
    } else {
        format!("{}d ago", seconds / 86_400)
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
        let mut snapshot = crate::model::Snapshot::fixture();
        let now = crate::store::history::now_epoch();
        for row in &mut snapshot.app_usage {
            row.last_seen = now - 3_600;
        }
        app.apply_snapshot(snapshot);
        app.tab = crate::app::Tab::AppHistory;
        app
    }

    #[test]
    fn columns_and_values() {
        let screen = render(&app(), 130, 30);
        for header in [
            "Name",
            "CPU time",
            "Peak memory",
            "Disk",
            "Last seen",
            "Network",
        ] {
            assert!(screen.contains(header), "missing {header}");
        }
        assert!(screen.contains("Safari"));
        assert!(screen.contains("1:00:00"));
        assert!(screen.contains("1h ago"));
    }

    #[test]
    fn network_is_a_dash_here_too() {
        let screen = render(&app(), 130, 30);
        let safari_line = screen.lines().find(|line| line.contains("Safari")).unwrap();
        assert!(safari_line.contains('-'));
    }

    #[test]
    fn empty_history_explains_itself() {
        let (mut app, _) = App::new(Settings::default(), false);
        app.tab = crate::app::Tab::AppHistory;
        let screen = render(&app, 120, 24);
        assert!(screen.contains("No usage recorded yet"));
    }

    #[test]
    fn relative_times() {
        assert_eq!(ago(10), "just now");
        assert_eq!(ago(120), "2m ago");
        assert_eq!(ago(7_200), "2h ago");
        assert_eq!(ago(200_000), "2d ago");
    }
}
