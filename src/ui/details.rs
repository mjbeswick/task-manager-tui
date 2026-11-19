//! Details tab: every process, one row each, plus the properties panel.

use crate::app::App;
use crate::model::ProcRow;
use crate::sample::group::{SortDir, SortKey};
use crate::ui::chrome::{fill_area, panel, put, put_right};
use crate::ui::format::{DASH, bytes, pad_left, pct, rate, truncate};
use crate::ui::hits::{HitMap, Target};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};

struct Layout2 {
    name: u16,
    pid: u16,
    status: Option<u16>,
    user: Option<u16>,
    cpu: u16,
    memory: u16,
    disk: Option<u16>,
    network: Option<u16>,
    gpu: Option<u16>,
}

fn layout_for(width: u16, optional: bool) -> Layout2 {
    let mut fixed = Layout2 {
        name: 0,
        pid: 7,
        status: Some(13),
        user: Some(12),
        cpu: 7,
        memory: 9,
        disk: Some(11),
        network: Some(9),
        gpu: Some(5),
    };
    if width < 110 || !optional {
        fixed.gpu = None;
    }
    if !optional {
        fixed.disk = None;
        fixed.network = None;
    }
    if width < 100 {
        fixed.network = None;
    }
    if width < 90 {
        fixed.disk = None;
    }
    if width < 75 {
        fixed.user = None;
    }
    if width < 65 {
        fixed.status = None;
    }
    let used: u16 = fixed.pid
        + fixed.status.unwrap_or(0)
        + fixed.user.unwrap_or(0)
        + fixed.cpu
        + fixed.memory
        + fixed.disk.unwrap_or(0)
        + fixed.network.unwrap_or(0)
        + fixed.gpu.unwrap_or(0);
    fixed.name = width.saturating_sub(used).max(6);
    fixed
}

pub fn draw(buf: &mut Buffer, area: Rect, app: &App, hits: &mut HitMap) {
    let theme = app.theme();
    fill_area(buf, area, theme.surface_style());
    if area.height < 3 {
        return;
    }

    let (table_area, panel_area) = if app.details.panel && area.height >= 18 {
        let [table, panel] =
            Layout::vertical([Constraint::Min(6), Constraint::Length(12)]).areas(area);
        (table, Some(panel))
    } else {
        (area, None)
    };

    let layout = layout_for(table_area.width, app.show_optional());
    draw_header(buf, table_area, &layout, app);
    hits.header(
        table_area.x,
        table_area.y,
        1,
        &[
            (Some(layout.name), SortKey::Name),
            (Some(layout.pid), SortKey::Pid),
            (layout.status, SortKey::Status),
            (layout.user, SortKey::User),
            (Some(layout.cpu), SortKey::Cpu),
            (Some(layout.memory), SortKey::Memory),
            (layout.disk, SortKey::Disk),
            (layout.network, SortKey::Network),
            (layout.gpu, SortKey::Gpu),
        ],
    );

    let rows = app.details_rows();
    if rows.is_empty() {
        let message = if app.details.filter.is_empty() {
            "No processes to show."
        } else {
            "No processes match the filter."
        };
        let y = table_area.y + 2 + table_area.height.saturating_sub(2) / 2;
        put(
            buf,
            table_area.x,
            y,
            message,
            theme.muted_style(),
            table_area,
        );
        return;
    }

    let body = Rect::new(
        table_area.x,
        table_area.y + 2,
        table_area.width,
        table_area.height.saturating_sub(2),
    );
    let visible = body.height as usize;
    let selected = app.details.selected;
    let scroll = if selected < app.details.scroll {
        selected
    } else if selected >= app.details.scroll + visible {
        selected + 1 - visible
    } else {
        app.details.scroll
    };
    let scroll = scroll.min(rows.len().saturating_sub(visible));

    for (offset, row) in rows.iter().skip(scroll).take(visible).enumerate() {
        let y = body.y + offset as u16;
        let is_selected = scroll + offset == selected;
        hits.add(
            Rect::new(body.x, y, body.width, 1),
            Target::Row(scroll + offset),
        );
        draw_row(buf, body, y, row, &layout, app, is_selected);
    }

    if let Some(panel_area) = panel_area
        && let Some(row) = rows.get(selected)
    {
        {
            let lines = crate::action::process_lines(row);
            let inner = panel(buf, panel_area, "Details", theme);
            for (index, line) in lines.iter().enumerate() {
                let y = inner.y + index as u16;
                if y >= inner.y + inner.height {
                    break;
                }
                put(
                    buf,
                    inner.x + 1,
                    y,
                    &truncate(line, inner.width.saturating_sub(2) as usize),
                    Style::default().fg(theme.text).bg(theme.surface),
                    inner,
                );
            }
        }
    }
}

fn draw_header(buf: &mut Buffer, area: Rect, layout: &Layout2, app: &App) {
    let theme = app.theme();
    let style = theme.header_style();
    let sort = app.details.sort;
    let arrow = if sort.dir == SortDir::Asc {
        theme.glyphs().sort_asc
    } else {
        theme.glyphs().sort_desc
    };
    let marker = |key: SortKey| {
        if key == sort.key {
            format!(" {arrow}")
        } else {
            String::new()
        }
    };

    let mut x = area.x;
    put(
        buf,
        x,
        area.y,
        &truncate(
            &format!("Name{}", marker(SortKey::Name)),
            layout.name as usize,
        ),
        style,
        area,
    );
    x += layout.name;
    put(
        buf,
        x,
        area.y,
        &format!(
            "{}{}",
            pad_left("PID", layout.pid as usize - 2),
            marker(SortKey::Pid)
        ),
        style,
        area,
    );
    x += layout.pid;
    if let Some(width) = layout.status {
        put(
            buf,
            x,
            area.y,
            &format!(
                "{}{}",
                pad_left("Status", width as usize - 2),
                marker(SortKey::Status)
            ),
            style,
            area,
        );
        x += width;
    }
    if let Some(width) = layout.user {
        put(
            buf,
            x,
            area.y,
            &format!(
                "{}{}",
                pad_left("User name", width as usize - 2),
                marker(SortKey::User)
            ),
            style,
            area,
        );
        x += width;
    }
    put(
        buf,
        x,
        area.y,
        &format!(
            "{}{}",
            pad_left("CPU", layout.cpu as usize - 2),
            marker(SortKey::Cpu)
        ),
        style,
        area,
    );
    x += layout.cpu;
    put(
        buf,
        x,
        area.y,
        &format!(
            "{}{}",
            pad_left("Memory", layout.memory as usize - 2),
            marker(SortKey::Memory)
        ),
        style,
        area,
    );
    x += layout.memory;
    if let Some(width) = layout.disk {
        put(
            buf,
            x,
            area.y,
            &format!(
                "{}{}",
                pad_left("Disk", width as usize - 2),
                marker(SortKey::Disk)
            ),
            style,
            area,
        );
        x += width;
    }
    if let Some(width) = layout.network {
        put(
            buf,
            x,
            area.y,
            &format!(
                "{}{}",
                pad_left("Network", width as usize - 2),
                marker(SortKey::Network)
            ),
            style,
            area,
        );
        x += width;
    }
    if let Some(width) = layout.gpu {
        put(
            buf,
            x,
            area.y,
            &format!(
                "{}{}",
                pad_left("GPU", width as usize - 2),
                marker(SortKey::Gpu)
            ),
            style,
            area,
        );
    }

    let rule = Style::default().fg(theme.border).bg(theme.surface);
    let glyphs = theme.glyphs();
    for x in area.x..area.x + area.width {
        buf[(x, area.y + 1)].set_char(glyphs.hline).set_style(rule);
    }
}

fn draw_row(
    buf: &mut Buffer,
    area: Rect,
    y: u16,
    row: &ProcRow,
    layout: &Layout2,
    app: &App,
    selected: bool,
) {
    let theme = app.theme();
    let base = if selected {
        theme.selected_style()
    } else {
        theme.surface_style()
    };
    fill_area(buf, Rect::new(area.x, y, area.width, 1), base);

    let mut x = area.x;
    put(
        buf,
        x,
        y,
        &truncate(&row.name, layout.name as usize),
        base,
        area,
    );
    x += layout.name;
    put_right(
        buf,
        x + layout.pid - 2,
        y,
        &row.key.pid.to_string(),
        base,
        area,
    );
    x += layout.pid;
    if let Some(width) = layout.status {
        put_right(buf, x + width - 2, y, row.status.label(), base, area);
        x += width;
    }
    if let Some(width) = layout.user {
        let user = row.user.clone().unwrap_or_else(|| DASH.into());
        put_right(buf, x + width - 2, y, &user, base, area);
        x += width;
    }
    put_right(buf, x + layout.cpu - 2, y, &pct(row.cpu), base, area);
    x += layout.cpu;
    put_right(
        buf,
        x + layout.memory - 2,
        y,
        &bytes(row.memory),
        base,
        area,
    );
    x += layout.memory;
    if let Some(width) = layout.disk {
        put_right(buf, x + width - 2, y, &rate(row.disk.io_bps()), base, area);
        x += width;
    }
    if let Some(width) = layout.network {
        let text = row
            .network
            .map(crate::ui::format::bit_rate)
            .unwrap_or_else(|| DASH.into());
        put_right(buf, x + width - 2, y, &text, base, area);
        x += width;
    }
    if let Some(width) = layout.gpu {
        let text = row.gpu.map(pct).unwrap_or_else(|| DASH.into());
        put_right(buf, x + width - 2, y, &text, base, area);
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
        app.apply_snapshot(crate::model::Snapshot::fixture());
        app.tab = crate::app::Tab::Details;
        app
    }

    #[test]
    fn every_column_header_is_present_when_wide() {
        let screen = render(&app(), 140, 40);
        for header in [
            "Name",
            "PID",
            "Status",
            "User name",
            "CPU",
            "Memory",
            "Disk",
            "Network",
            "GPU",
        ] {
            assert!(screen.contains(header), "missing {header}");
        }
    }

    #[test]
    fn network_is_a_dash() {
        let screen = render(&app(), 140, 40);
        assert!(screen.contains("Ghostty"));
        assert!(screen.contains("WindowServer"));
    }

    #[test]
    fn narrow_hides_optional_columns() {
        let screen = render(&app(), 70, 30);
        let header = screen
            .lines()
            .find(|line| line.trim_start().starts_with("Name"))
            .expect("header row");
        assert!(header.contains("PID"));
        assert!(!header.contains("Network"), "{header}");
        assert!(!header.contains("GPU"), "{header}");
    }

    #[test]
    fn panel_shows_process_details() {
        let mut app = app();
        app.details.panel = true;
        let screen = render(&app, 120, 40);
        assert!(screen.contains("Details"));
        assert!(screen.contains("Command line"));
        assert!(screen.contains("Priority"));
    }
}
