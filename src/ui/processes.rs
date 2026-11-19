//! Processes tab: the grouped table.

use crate::app::App;
use crate::sample::group::ProcItem;
use crate::ui::chrome::{fill_area, put, put_right};
use crate::ui::format::{DASH, bytes, pad_left, pct, rate, truncate};
use crate::ui::hits::{HitMap, Target};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};

struct Layout {
    name: u16,
    status: Option<u16>,
    cpu: u16,
    memory: u16,
    disk: Option<u16>,
    network: Option<u16>,
    gpu: Option<u16>,
}

fn layout_for(width: u16, more_details: bool, optional: bool) -> Layout {
    if !more_details {
        return Layout {
            name: width.saturating_sub(20).max(8),
            status: None,
            cpu: 9,
            memory: 11,
            disk: None,
            network: None,
            gpu: None,
        };
    }
    let mut layout = Layout {
        name: 0,
        status: Some(14),
        cpu: 7,
        memory: 9,
        disk: Some(11),
        network: Some(9),
        gpu: Some(5),
    };
    if width < 95 || !optional {
        layout.gpu = None;
    }
    if !optional {
        layout.disk = None;
        layout.network = None;
    }
    if width < 85 {
        layout.network = None;
    }
    if width < 75 {
        layout.disk = None;
    }
    if width < 65 {
        layout.status = None;
    }
    let fixed: u16 = layout.status.unwrap_or(0)
        + layout.cpu
        + layout.memory
        + layout.disk.unwrap_or(0)
        + layout.network.unwrap_or(0)
        + layout.gpu.unwrap_or(0);
    layout.name = width.saturating_sub(fixed).max(6);
    layout
}

pub fn draw(buf: &mut Buffer, area: Rect, app: &App, hits: &mut HitMap) {
    let theme = app.theme();
    let surface = theme.surface_style();
    fill_area(buf, area, surface);
    if area.height < 3 {
        return;
    }

    let layout = layout_for(area.width, app.process.more_details, app.show_optional());
    let header_y = area.y;
    draw_header(buf, area, header_y, &layout, app);

    let rows_area = Rect::new(
        area.x,
        area.y + 2,
        area.width,
        area.height.saturating_sub(2),
    );
    let items = &app.process.items;
    if items.is_empty() {
        let message = if app.process.filter.is_empty() {
            "No processes to show."
        } else {
            "No processes match the filter."
        };
        let x = area.x + area.width.saturating_sub(message.len() as u16) / 2;
        put(
            buf,
            x,
            rows_area.y + rows_area.height / 2,
            message,
            theme.muted_style(),
            rows_area,
        );
        return;
    }

    let visible = rows_area.height as usize;
    let selected = app.process.selected;
    let scroll = if selected < app.process.scroll {
        selected
    } else if selected >= app.process.scroll + visible {
        selected + 1 - visible
    } else {
        app.process.scroll
    };
    let scroll = scroll.min(items.len().saturating_sub(visible));

    for (offset, item) in items.iter().skip(scroll).take(visible).enumerate() {
        let y = rows_area.y + offset as u16;
        let is_selected = scroll + offset == selected;
        hits.add(
            Rect::new(area.x, y, area.width, 1),
            Target::Row(scroll + offset),
        );
        draw_item(buf, area, y, item, &layout, app, is_selected);
    }

    if items.len() > visible {
        let marker = format!("{}/{}", selected + 1, items.len());
        put_right(
            buf,
            area.x + area.width - 1,
            rows_area.y + rows_area.height - 1,
            &marker,
            Style::default().fg(theme.muted).bg(theme.surface),
            rows_area,
        );
    }
}

fn draw_header(buf: &mut Buffer, area: Rect, y: u16, layout: &Layout, app: &App) {
    let theme = app.theme();
    let style = theme.header_style();
    fill_area(
        buf,
        Rect::new(area.x, y, area.width, 1),
        Style::default().bg(theme.surface),
    );
    let sort = app.process.sort;
    let arrow = if sort.dir == crate::sample::group::SortDir::Asc {
        theme.glyphs().sort_asc
    } else {
        theme.glyphs().sort_desc
    };
    let marker = |key: crate::sample::group::SortKey| {
        if key == sort.key {
            format!(" {arrow}")
        } else {
            String::new()
        }
    };

    let mut x = area.x;
    let name_title = format!("Name{}", marker(crate::sample::group::SortKey::Name));
    put(
        buf,
        x,
        y,
        &truncate(&name_title, layout.name as usize),
        style,
        area,
    );
    x += layout.name;
    if let Some(width) = layout.status {
        let title = format!(
            "{}{}",
            pad_left("Status", width as usize - 2),
            marker(crate::sample::group::SortKey::Status)
        );
        put(buf, x, y, &title, style, area);
        x += width;
    }
    let cpu = format!(
        "{}{}",
        pad_left("CPU", layout.cpu as usize - 2),
        marker(crate::sample::group::SortKey::Cpu)
    );
    put(buf, x, y, &cpu, style, area);
    x += layout.cpu;
    let memory = format!(
        "{}{}",
        pad_left("Memory", layout.memory as usize - 2),
        marker(crate::sample::group::SortKey::Memory)
    );
    put(buf, x, y, &memory, style, area);
    x += layout.memory;
    if let Some(width) = layout.disk {
        let text = format!(
            "{}{}",
            pad_left("Disk", width as usize - 2),
            marker(crate::sample::group::SortKey::Disk)
        );
        put(buf, x, y, &text, style, area);
        x += width;
    }
    if let Some(width) = layout.network {
        let text = format!(
            "{}{}",
            pad_left("Network", width as usize - 2),
            marker(crate::sample::group::SortKey::Network)
        );
        put(buf, x, y, &text, style, area);
        x += width;
    }
    if let Some(width) = layout.gpu {
        let text = format!(
            "{}{}",
            pad_left("GPU", width as usize - 2),
            marker(crate::sample::group::SortKey::Gpu)
        );
        put(buf, x, y, &text, style, area);
    }

    // The Windows 10 rule under the header.
    let rule = Style::default().fg(theme.border).bg(theme.surface);
    let glyphs = theme.glyphs();
    for x in area.x..area.x + area.width {
        buf[(x, y + 1)].set_char(glyphs.hline).set_style(rule);
    }
}

fn draw_item(
    buf: &mut Buffer,
    area: Rect,
    y: u16,
    item: &ProcItem,
    layout: &Layout,
    app: &App,
    selected: bool,
) {
    let theme = app.theme();
    let snapshot = &app.snapshot;
    let base = if selected {
        theme.selected_style()
    } else {
        theme.surface_style()
    };
    fill_area(buf, Rect::new(area.x, y, area.width, 1), base);

    let mut x = area.x;
    let (indent, marker, label, count, cpu, memory, disk, network, gpu) = match item {
        ProcItem::Header {
            kind,
            count,
            expanded,
        } => (
            0,
            if *expanded {
                theme.glyphs().expanded
            } else {
                theme.glyphs().collapsed
            },
            kind.label().to_string(),
            Some(*count),
            None,
            None,
            None,
            None,
            None,
        ),
        ProcItem::Group {
            kind: _,
            name,
            cpu,
            memory,
            disk,
            count,
            expanded,
        } => (
            2,
            if *expanded {
                theme.glyphs().expanded
            } else {
                theme.glyphs().collapsed
            },
            name.clone(),
            Some(*count),
            Some(*cpu),
            Some(*memory),
            Some(*disk),
            None,
            None,
        ),
        ProcItem::Child { key, name, depth } => {
            let row = snapshot.process(*key);
            (
                (2 + *depth * 2) as u16,
                ' ',
                name.clone(),
                None,
                row.map(|r| r.cpu),
                row.map(|r| r.memory),
                row.map(|r| r.disk),
                row.and_then(|r| r.network),
                row.and_then(|r| r.gpu),
            )
        }
    };

    let is_child = matches!(item, ProcItem::Child { .. });
    let name_width = layout.name;
    let mut name_text = String::new();
    name_text.push_str(&" ".repeat(indent as usize));
    if marker != ' ' {
        name_text.push(marker);
        name_text.push(' ');
    }
    name_text.push_str(&label);
    if let Some(count) = count {
        name_text.push_str(&format!(" ({count})"));
    }
    let style = if matches!(item, ProcItem::Header { .. }) {
        base.add_modifier(Modifier::BOLD)
    } else {
        base
    };
    put(
        buf,
        x,
        y,
        &truncate(&name_text, name_width as usize),
        style,
        area,
    );
    x += name_width;

    if let Some(width) = layout.status {
        let status = match item {
            ProcItem::Child { key, .. } => snapshot
                .process(*key)
                .map(|r| r.status.label().to_string())
                .unwrap_or_else(|| DASH.to_string()),
            _ => String::new(),
        };
        put_right(buf, x + width - 2, y, &status, base, area);
        x += width;
    }

    let cpu_text = cpu.map(pct).unwrap_or_default();
    let cpu_style = if cpu.unwrap_or(0.0) >= 50.0 {
        Style::default().fg(theme.warn).bg(if selected {
            theme.selection
        } else {
            theme.surface
        })
    } else {
        base
    };
    put_right(buf, x + layout.cpu - 2, y, &cpu_text, cpu_style, area);
    x += layout.cpu;

    let memory_text = memory.map(bytes).unwrap_or_default();
    put_right(buf, x + layout.memory - 2, y, &memory_text, base, area);
    x += layout.memory;

    if let Some(width) = layout.disk {
        let text = disk.map(|d| rate(d.io_bps())).unwrap_or_default();
        put_right(buf, x + width - 2, y, &text, base, area);
        x += width;
    }
    if let Some(width) = layout.network {
        let text = match network {
            Some(value) => crate::ui::format::bit_rate(value),
            None if is_child => DASH.to_string(),
            None => String::new(),
        };
        put_right(buf, x + width - 2, y, &text, base, area);
        x += width;
    }
    if let Some(width) = layout.gpu {
        let text = match gpu {
            Some(value) => pct(value),
            None if is_child => DASH.to_string(),
            None => String::new(),
        };
        put_right(buf, x + width - 2, y, &text, base, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::App;
    use crate::model::ProcSel;
    use crate::store::settings::Settings;

    fn app() -> App {
        let (mut app, _) = App::new(Settings::default(), false);
        app.apply_snapshot(crate::model::Snapshot::fixture());
        app
    }

    #[test]
    fn layout_drops_optional_columns_when_narrow() {
        let wide = layout_for(120, true, true);
        assert!(wide.gpu.is_some() && wide.network.is_some() && wide.disk.is_some());
        let medium = layout_for(80, true, true);
        assert!(medium.gpu.is_none() && medium.network.is_none());
        assert!(medium.disk.is_some());
        let narrow = layout_for(50, true, true);
        assert!(narrow.disk.is_none() && narrow.status.is_none());
        let fewer = layout_for(120, false, true);
        assert!(fewer.status.is_none() && fewer.disk.is_none());
        assert_eq!(fewer.name + fewer.cpu + fewer.memory, 120);
        let hidden = layout_for(120, true, false);
        assert!(hidden.gpu.is_none() && hidden.disk.is_none() && hidden.network.is_none());
    }

    #[test]
    fn group_rows_show_counts_and_aggregates() {
        let app = app();
        assert!(app.process.items.iter().any(|item| matches!(
            item,
            ProcItem::Header { kind, count, .. } if *kind == crate::model::Kind::App && *count == 2
        )));
        assert!(app.process.items.iter().any(|item| matches!(
            item,
            ProcItem::Group { name, count, .. } if name == "Ghostty" && *count == 1
        )));
    }

    #[test]
    fn selection_is_by_key_not_index() {
        let mut app = app();
        app.process.selected = 1;
        app.process.key = Some(app.process.items[1].sel());
        let key = app.process.key.clone().unwrap();
        app.process.items.reverse();
        app.process.key = Some(key.clone());
        assert_eq!(app.process.selected_key(), Some(key));
    }

    #[test]
    fn collapsing_a_kind_hides_its_groups() {
        let mut app = app();
        app.collapse_all();
        assert!(
            app.process
                .items
                .iter()
                .all(|item| matches!(item, ProcItem::Header { .. }))
        );
        let selected = app.process.items[0].sel();
        assert!(matches!(selected, ProcSel::Kind(_)));
    }
}
