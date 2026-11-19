//! Processes tab: the grouped table.

use crate::app::App;
use crate::sample::group::ProcItem;
use crate::ui::chrome::{fill_area, put, put_match, put_right};
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
        network: Some(10),
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
    // Windows 10 puts the system total above each resource label. Short
    // terminals keep the single row header.
    let totals = area.height >= 12;
    let header_rows = if totals { 3 } else { 2 };
    draw_header(buf, area, area.y, &layout, app, totals);
    {
        use crate::sample::group::SortKey as K;
        hits.header(
            area.x,
            area.y,
            header_rows - 1,
            &[
                (Some(layout.name), K::Name),
                (layout.status, K::Status),
                (Some(layout.cpu), K::Cpu),
                (Some(layout.memory), K::Memory),
                (layout.disk, K::Disk),
                (layout.network, K::Network),
                (layout.gpu, K::Gpu),
            ],
        );
    }

    let rows_area = Rect::new(
        area.x,
        area.y + header_rows,
        area.width,
        area.height.saturating_sub(header_rows),
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
}

/// System totals shown above the resource labels, as Windows 10 does.
struct Totals {
    cpu: String,
    memory: String,
    disk: String,
    network: String,
    gpu: String,
}

fn totals(app: &App) -> Totals {
    let s = &app.snapshot;
    let memory = if s.memory.total > 0 {
        format!(
            "{:.0}%",
            s.memory.used as f64 * 100.0 / s.memory.total as f64
        )
    } else {
        DASH.to_string()
    };
    let gpu = if s.gpus.is_empty() {
        DASH.to_string()
    } else {
        format!("{:.0}%", s.gpus.iter().map(|g| g.usage).fold(0.0, f32::max))
    };
    Totals {
        cpu: format!("{:.0}%", s.cpu.total),
        memory,
        // Compact, so a total fits its column: "5.3MB/s", "36.7Mbps".
        disk: rate(disk_total(app)).replace(' ', ""),
        network: crate::ui::format::bit_rate(network_total(app)).replace(' ', ""),
        gpu,
    }
}

fn disk_total(app: &App) -> f64 {
    app.snapshot
        .disks
        .iter()
        .map(|d| d.read_bps + d.write_bps)
        .sum()
}

fn network_total(app: &App) -> f64 {
    app.snapshot
        .networks
        .iter()
        .map(|n| n.rx_bps + n.tx_bps)
        .sum()
}

/// Percent of `total` that `value` makes up, or `None` when there is no total.
fn share(value: f64, total: f64) -> Option<f32> {
    (total > 0.0).then(|| (value * 100.0 / total) as f32)
}

fn draw_header(buf: &mut Buffer, area: Rect, top: u16, layout: &Layout, app: &App, totals: bool) {
    let theme = app.theme();
    let style = theme.header_style();
    let y = if totals { top + 1 } else { top };
    fill_area(
        buf,
        Rect::new(area.x, top, area.width, y - top + 1),
        Style::default().bg(theme.surface),
    );
    if totals {
        let figures = self::totals(app);
        let total_style = Style::default()
            .fg(theme.text)
            .bg(theme.surface)
            .add_modifier(Modifier::BOLD);
        let mut x = area.x + layout.name + layout.status.unwrap_or(0);
        let mut cell = |x: &mut u16, width: Option<u16>, text: &str| {
            if let Some(width) = width {
                put_right(buf, *x + width - 2, top, text, total_style, area);
                *x += width;
            }
        };
        cell(&mut x, Some(layout.cpu), &figures.cpu);
        cell(&mut x, Some(layout.memory), &figures.memory);
        cell(&mut x, layout.disk, &figures.disk);
        cell(&mut x, layout.network, &figures.network);
        cell(&mut x, layout.gpu, &figures.gpu);
    }
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
    put_match(
        buf,
        x,
        y,
        &truncate(&name_text, name_width as usize),
        style,
        area,
        app,
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

    // Shade a resource cell by its share of the system total. Selected rows
    // and dash cells keep the plain row background.
    let shaded = |buf: &mut Buffer, x: u16, width: u16, share: Option<f32>| match share
        .filter(|_| !selected)
        .and_then(|s| theme.heat_for(s))
    {
        Some(heat) => {
            let style = Style::default().fg(theme.text).bg(heat);
            fill_area(buf, Rect::new(x, y, width, 1), style);
            style
        }
        None => base,
    };

    let cpu_text = cpu.map(pct).unwrap_or_default();
    let cpu_style = shaded(buf, x, layout.cpu, cpu);
    put_right(buf, x + layout.cpu - 2, y, &cpu_text, cpu_style, area);
    x += layout.cpu;

    let memory_text = memory.map(bytes).unwrap_or_default();
    let memory_share = memory.and_then(|m| share(m as f64, snapshot.memory.total as f64));
    let memory_style = shaded(buf, x, layout.memory, memory_share);
    put_right(
        buf,
        x + layout.memory - 2,
        y,
        &memory_text,
        memory_style,
        area,
    );
    x += layout.memory;

    if let Some(width) = layout.disk {
        let text = disk.map(|d| rate(d.io_bps())).unwrap_or_default();
        let disk_share = disk.and_then(|d| share(d.io_bps(), disk_total(app)));
        let style = shaded(buf, x, width, disk_share);
        put_right(buf, x + width - 2, y, &text, style, area);
        x += width;
    }
    if let Some(width) = layout.network {
        let text = match network {
            Some(value) => crate::ui::format::bit_rate(value),
            None if is_child => DASH.to_string(),
            None => String::new(),
        };
        let style = shaded(
            buf,
            x,
            width,
            network.and_then(|n| share(n, network_total(app))),
        );
        put_right(buf, x + width - 2, y, &text, style, area);
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

    fn frame(app: &App, width: u16, height: u16) -> (String, Buffer) {
        let mut hits = crate::ui::hits::HitMap::default();
        let mut terminal =
            ratatui::Terminal::new(ratatui::backend::TestBackend::new(width, height)).unwrap();
        terminal
            .draw(|f| crate::ui::draw(f, app, &mut hits))
            .unwrap();
        let buffer = terminal.backend().buffer().clone();
        (crate::ui::test_util::render(app, width, height), buffer)
    }

    /// Background of the cell under the last character of `needle` on its row.
    fn bg_at(screen: &str, buffer: &Buffer, row_text: &str, needle: &str) -> ratatui::style::Color {
        let (y, line) = screen
            .lines()
            .enumerate()
            .find(|(_, l)| l.contains(row_text))
            .unwrap_or_else(|| panic!("no row {row_text}"));
        let x =
            line[..line.find(needle).expect(needle)].chars().count() + needle.chars().count() - 1;
        buffer[(x as u16, y as u16)].bg
    }

    #[test]
    fn resource_cells_are_shaded_by_share() {
        let mut app = app();
        app.process.selected = 0;
        let theme = *app.theme();
        let (screen, buffer) = frame(&app, 120, 36);
        // node uses 12.5% of the CPU: the 5 to 15 percent step.
        assert_eq!(bg_at(&screen, &buffer, "node", "12.5%"), theme.heat[1]);
        // launchd's 0.2% is below the first step.
        assert_eq!(bg_at(&screen, &buffer, "launchd", "0.2%"), theme.surface);
        // 1.2 GB of 16 GB memory is about 7.5%: the second step.
        assert_eq!(bg_at(&screen, &buffer, "Safari", "1.2 GB"), theme.heat[1]);
        // Kind headings are never shaded.
        assert_eq!(bg_at(&screen, &buffer, "Apps (", "Apps"), theme.selection);
        assert_eq!(
            bg_at(&screen, &buffer, "Background", "Background"),
            theme.surface
        );
        // No process reports network, so no network cell is shaded.
        let network_x = screen.lines().nth(3).unwrap().find("Network").unwrap() as u16 + 6;
        for y in 5..20 {
            assert!(!theme.heat.contains(&buffer[(network_x, y)].bg), "row {y}");
        }
    }

    #[test]
    fn the_selected_row_is_not_shaded() {
        let mut app = app();
        let index = app
            .process
            .items
            .iter()
            .position(|i| matches!(i, ProcItem::Group { name, .. } if name == "node"))
            .unwrap();
        app.process.selected = index;
        let theme = *app.theme();
        let (screen, buffer) = frame(&app, 120, 36);
        assert_eq!(bg_at(&screen, &buffer, "node", "12.5%"), theme.selection);
    }

    #[test]
    fn header_shows_totals_when_tall() {
        let app = app();
        let (screen, _) = frame(&app, 120, 36);
        let lines: Vec<&str> = screen.lines().collect();
        assert!(lines[3].starts_with("Name"), "{:?}", lines[3]);
        let cpu = lines[3].find("CPU").unwrap();
        assert!(
            lines[2][..cpu + 3].trim_end().ends_with("27%"),
            "{:?}",
            lines[2]
        );
        assert!(lines[2].contains("62%"));
        let (short, _) = frame(&app, 80, 14);
        assert!(
            short.lines().nth(2).unwrap().starts_with("Name"),
            "one row header"
        );
    }

    #[test]
    fn paging_keeps_the_selection_visible_at_both_header_heights() {
        for height in [14, 36] {
            let mut snapshot = crate::model::Snapshot::fixture();
            let template = snapshot.processes[0].clone();
            for i in 0..80u32 {
                let mut row = template.clone();
                row.key = crate::model::ProcKey::new(10_000 + i, 1);
                row.name = format!("proc{i:02}");
                row.kind = crate::model::Kind::Background;
                snapshot.processes.push(row);
            }
            let mut app = app();
            app.apply_snapshot(snapshot);
            for _ in 0..4 {
                app.dispatch(crate::input::Action::PageDown);
            }
            let theme = *app.theme();
            let (_, buffer) = frame(&app, 120, height);
            let visible = (0..height).any(|y| buffer[(10, y)].bg == theme.selection);
            assert!(visible, "selection off screen at height {height}");
        }
    }
}
