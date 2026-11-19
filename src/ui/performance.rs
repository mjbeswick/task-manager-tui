//! Performance tab: resource rail, 60 second graphs, and Windows 10 stat boxes.

use crate::app::{App, ResourceChoice};
use crate::theme::Theme;
use crate::ui::chrome::{fill_area, panel, put, put_right};
use crate::ui::format::{DASH, bit_rate, bytes, bytes_opt, duration, mhz, pct, rate, truncate};
use crate::ui::graph::{self, Series};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};

pub fn draw(buf: &mut Buffer, area: Rect, app: &App) {
    let theme = app.theme();
    fill_area(buf, area, theme.surface_style());
    let narrow = area.width < 70;

    let (rail, body) = if narrow {
        let [rail, body] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(area);
        (rail, body)
    } else {
        let [rail, body] =
            Layout::horizontal([Constraint::Length(20), Constraint::Min(0)]).areas(area);
        (rail, body)
    };

    draw_rail(buf, rail, app, narrow);
    draw_panel(buf, body, app);
}

fn draw_rail(buf: &mut Buffer, area: Rect, app: &App, narrow: bool) {
    let theme = app.theme();
    fill_area(buf, area, Style::default().bg(theme.bg));
    let resources = [
        ResourceChoice::Cpu,
        ResourceChoice::Memory,
        ResourceChoice::Disk,
        ResourceChoice::Network,
        ResourceChoice::Gpu,
    ];
    if narrow {
        let mut x = area.x;
        for resource in resources {
            let active = resource == app.performance.resource;
            let text = format!(" {} ", resource.label());
            let style = if active {
                Style::default()
                    .fg(ratatui::style::Color::White)
                    .bg(theme.accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.muted).bg(theme.bg)
            };
            put(buf, x, area.y, &text, style, area);
            x += text.chars().count() as u16 + 1;
        }
        return;
    }

    for (index, resource) in resources.iter().enumerate() {
        let active = *resource == app.performance.resource;
        let y = area.y + index as u16;
        let style = if active {
            Style::default()
                .fg(theme.text)
                .bg(theme.selection)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.text).bg(theme.bg)
        };
        if active {
            fill_area(buf, Rect::new(area.x, y, area.width, 1), style);
            buf[(area.x, y)]
                .set_char('▌')
                .set_style(Style::default().fg(theme.accent).bg(theme.selection));
        }
        put(buf, area.x + 2, y, resource.label(), style, area);
    }
}

fn draw_panel(buf: &mut Buffer, area: Rect, app: &App) {
    let theme = app.theme();
    if area.width < 10 || area.height < 5 {
        return;
    }
    let title = app.performance.resource.label();
    let inner = panel(buf, area, title, theme);
    match app.performance.resource {
        ResourceChoice::Cpu => draw_cpu(buf, inner, app),
        ResourceChoice::Memory => draw_memory(buf, inner, app),
        ResourceChoice::Disk => draw_disk(buf, inner, app),
        ResourceChoice::Network => draw_network(buf, inner, app),
        ResourceChoice::Gpu => draw_gpu(buf, inner, app),
    }
}

fn draw_cpu(buf: &mut Buffer, inner: Rect, app: &App) {
    let theme = app.theme();
    let snapshot = &app.snapshot;
    let show_cores = snapshot.cpu.logical > 1 && inner.height >= 12;
    let (graph_area, boxes_area, cores_area) = if show_cores {
        let cores_height = ((inner.height / 3).max(4)).min(inner.height - 6);
        let [graph_area, boxes_area, cores_area] = Layout::vertical([
            Constraint::Min(4),
            Constraint::Length(stat_height(inner.width)),
            Constraint::Length(cores_height),
        ])
        .areas(inner);
        (graph_area, boxes_area, Some(cores_area))
    } else {
        let [graph_area, boxes_area] =
            Layout::vertical([Constraint::Min(3), Constraint::Length(3)]).areas(inner);
        (graph_area, boxes_area, None)
    };

    let label = pct(app.graphs.cpu.last());
    graph::draw(
        buf,
        graph_area,
        &app.graphs.cpu,
        100.0,
        theme,
        &label,
        "60 seconds",
    );

    let cpu = &snapshot.cpu;
    let boxes = [
        ("Utilization", pct(cpu.total), ""),
        ("Speed", mhz(cpu.speed_mhz), ""),
        ("Processes", snapshot.processes.len().to_string(), ""),
        (
            "Threads",
            snapshot
                .thread_count
                .map(|t| t.to_string())
                .unwrap_or_else(|| DASH.to_string()),
            "",
        ),
        (
            "Handles",
            snapshot
                .handle_count
                .map(|h| h.to_string())
                .unwrap_or_else(|| DASH.to_string()),
            "",
        ),
        ("Up time", duration(snapshot.uptime_secs), ""),
    ];
    draw_stat_row(buf, boxes_area, &boxes, app);

    if let Some(cores_area) = cores_area {
        draw_cores(buf, cores_area, app);
    }
}

fn draw_cores(buf: &mut Buffer, area: Rect, app: &App) {
    let theme = app.theme();
    let count = app.graphs.cores.len();
    if count == 0 || area.height == 0 {
        return;
    }
    let columns = if area.width >= 56 {
        4
    } else if area.width >= 30 {
        2
    } else {
        1
    };
    let rows = count.div_ceil(columns);
    let cell_width = area.width / columns as u16;
    let cell_height = (area.height / rows as u16).max(1);

    for (index, series) in app.graphs.cores.iter().enumerate() {
        let column = index % columns;
        let row = index / columns;
        let x = area.x + column as u16 * cell_width;
        let y = area.y + row as u16 * cell_height;
        if y >= area.y + area.height {
            break;
        }
        let height = cell_height.min(area.y + area.height - y);
        let cell = Rect::new(x, y, cell_width.saturating_sub(1), height);
        let mut graph_cell = cell;
        if cell.height >= 2 {
            let label = format!("CPU {index}  {}", pct(series.last()));
            put(
                buf,
                cell.x,
                cell.y,
                &truncate(&label, cell.width as usize),
                Style::default().fg(theme.muted).bg(theme.surface),
                cell,
            );
            graph_cell = Rect::new(cell.x, cell.y + 1, cell.width, cell.height - 1);
        }
        draw_mini(buf, graph_cell, series, theme);
    }
}

fn draw_mini(buf: &mut Buffer, area: Rect, series: &Series, theme: &Theme) {
    let glyphs = theme.glyphs();
    let style = Style::default().fg(theme.graph_fill).bg(theme.surface);
    let count = series.values.len();
    let width = area.width as usize;
    for column in 0..width.min(count) {
        let value = series.values[count - 1 - column];
        let x = area.x + area.width - 1 - column as u16;
        let halves = ((value / 100.0).clamp(0.0, 1.0) * (area.height * 2) as f32).round() as usize;
        for offset in 0..area.height as usize {
            let units = halves.saturating_sub(offset * 2);
            let y = area.y + area.height - 1 - offset as u16;
            if units >= 2 {
                buf[(x, y)].set_char(glyphs.block_full).set_style(style);
            } else if units == 1 {
                buf[(x, y)].set_char(glyphs.block_upper).set_style(style);
            }
        }
    }
}

fn draw_memory(buf: &mut Buffer, inner: Rect, app: &App) {
    let theme = app.theme();
    let memory = &app.snapshot.memory;
    let [graph_area, boxes_area] = Layout::vertical([
        Constraint::Min(3),
        Constraint::Length(stat_height(inner.width)),
    ])
    .areas(inner);
    let used_pct = if memory.total > 0 {
        memory.used as f32 / memory.total as f32 * 100.0
    } else {
        0.0
    };
    graph::draw(
        buf,
        graph_area,
        &app.graphs.memory,
        100.0,
        theme,
        &pct(used_pct),
        "60 seconds",
    );
    let boxes = [
        ("In use", bytes(memory.used), ""),
        ("Available", bytes(memory.available), ""),
        ("Total", bytes(memory.total), ""),
        ("Cached", bytes_opt(memory.cached), ""),
        ("Compressed", bytes_opt(memory.compressed), ""),
        ("Swap used", bytes(memory.swap_used), ""),
    ];
    draw_stat_row(buf, boxes_area, &boxes, app);
    let note = format!("Committed {} is not reported on this platform.", DASH);
    if inner.height >= 5 {
        put(
            buf,
            inner.x,
            inner.y + inner.height - 1,
            "A dash means the platform does not report that figure.",
            Style::default().fg(theme.muted).bg(theme.surface),
            inner,
        );
    }
    let _ = note;
}

fn draw_disk(buf: &mut Buffer, inner: Rect, app: &App) {
    let theme = app.theme();
    let snapshot = &app.snapshot;
    let busiest = snapshot
        .disks
        .iter()
        .max_by(|a, b| a.io_bps().total_cmp(&b.io_bps()));
    let [graph_area, boxes_area, list_area] = Layout::vertical([
        Constraint::Min(3),
        Constraint::Length(stat_height(inner.width)),
        Constraint::Length(4),
    ])
    .areas(inner);

    let label = busiest
        .map(|d| rate(d.io_bps()))
        .unwrap_or_else(|| DASH.into());
    graph::draw(
        buf,
        graph_area,
        &app.graphs.disk,
        0.0,
        theme,
        &label,
        "60 seconds",
    );

    let boxes = [
        ("Active time", DASH.to_string(), ""),
        (
            "Read speed",
            rate(snapshot.disks.iter().map(|d| d.read_bps).sum()),
            "",
        ),
        (
            "Write speed",
            rate(snapshot.disks.iter().map(|d| d.write_bps).sum()),
            "",
        ),
    ];
    draw_stat_row(buf, boxes_area, &boxes, app);

    if snapshot.disks.is_empty() {
        put(
            buf,
            list_area.x,
            list_area.y,
            "No physical disks reported.",
            theme.muted_style(),
            list_area,
        );
        return;
    }
    for (index, disk) in snapshot.disks.iter().enumerate() {
        let y = list_area.y + index as u16;
        if y >= list_area.y + list_area.height {
            break;
        }
        let text = format!(
            "{}   {}   {} free of {}",
            disk.name,
            rate(disk.io_bps()),
            bytes(disk.available),
            bytes(disk.total)
        );
        put(
            buf,
            list_area.x + 1,
            y,
            &truncate(&text, list_area.width.saturating_sub(2) as usize),
            Style::default().fg(theme.text).bg(theme.surface),
            list_area,
        );
    }
}

fn draw_network(buf: &mut Buffer, inner: Rect, app: &App) {
    let theme = app.theme();
    let snapshot = &app.snapshot;
    let adapter = snapshot
        .networks
        .iter()
        .max_by(|a, b| (a.rx_bps + a.tx_bps).total_cmp(&(b.rx_bps + b.tx_bps)));
    let [graph_area, boxes_area] = Layout::vertical([
        Constraint::Min(3),
        Constraint::Length(stat_height(inner.width)),
    ])
    .areas(inner);

    let label = adapter
        .map(|a| bit_rate(a.rx_bps + a.tx_bps))
        .unwrap_or_else(|| DASH.into());
    graph::draw(
        buf,
        graph_area,
        &app.graphs.network,
        0.0,
        theme,
        &label,
        "60 seconds",
    );

    let boxes = [
        (
            "Receive",
            adapter
                .map(|a| bit_rate(a.rx_bps))
                .unwrap_or_else(|| DASH.into()),
            "",
        ),
        (
            "Send",
            adapter
                .map(|a| bit_rate(a.tx_bps))
                .unwrap_or_else(|| DASH.into()),
            "",
        ),
        (
            "Adapter",
            adapter
                .map(|a| a.name.clone())
                .unwrap_or_else(|| DASH.into()),
            "",
        ),
        (
            "Link speed",
            adapter
                .and_then(|a| a.link_speed)
                .map(|bits| format!("{:.1} Gbps", bits as f64 / 1e9))
                .unwrap_or_else(|| DASH.into()),
            "",
        ),
    ];
    draw_stat_row(buf, boxes_area, &boxes, app);

    if adapter.is_none() {
        put(
            buf,
            inner.x,
            inner.y + inner.height.saturating_sub(1),
            "No active network adapter.",
            theme.muted_style(),
            inner,
        );
    }
}

fn draw_gpu(buf: &mut Buffer, inner: Rect, app: &App) {
    let theme = app.theme();
    if app.snapshot.gpus.is_empty() {
        let message = "Not available on this system";
        let y = inner.y + inner.height / 2;
        let x = inner.x + inner.width.saturating_sub(message.len() as u16) / 2;
        put(buf, x, y, message, theme.muted_style(), inner);
        return;
    }
    let [graph_area, boxes_area] = Layout::vertical([
        Constraint::Min(3),
        Constraint::Length(stat_height(inner.width)),
    ])
    .areas(inner);
    let gpu = app
        .snapshot
        .gpus
        .iter()
        .max_by(|a, b| a.usage.total_cmp(&b.usage));
    let label = gpu.map(|g| pct(g.usage)).unwrap_or_else(|| DASH.into());
    graph::draw(
        buf,
        graph_area,
        &app.graphs.gpu,
        100.0,
        theme,
        &label,
        "60 seconds",
    );
    let boxes = [
        ("Utilization", label, ""),
        (
            "Adapter",
            gpu.map(|g| g.name.clone()).unwrap_or_else(|| DASH.into()),
            "",
        ),
        (
            "Memory",
            gpu.and_then(|g| g.used_memory)
                .map(bytes)
                .unwrap_or_else(|| DASH.into()),
            "",
        ),
    ];
    draw_stat_row(buf, boxes_area, &boxes, app);
}

/// Height a stat box row needs: one row of boxes on wide panels, two on narrow.
fn stat_height(width: u16) -> u16 {
    if width >= 80 { 3 } else { 6 }
}

/// A row of Windows 10 stat boxes, wrapping to two rows when narrow.
fn draw_stat_row(buf: &mut Buffer, area: Rect, boxes: &[(&str, String, &str)], app: &App) {
    if boxes.is_empty() || area.height == 0 {
        return;
    }
    let per_row = if area.width >= 80 {
        boxes.len()
    } else if area.width >= 40 {
        boxes.len().div_ceil(2)
    } else {
        2
    };
    let rows = boxes.len().div_ceil(per_row) as u16;
    let row_height = (area.height / rows.max(1)).max(1);
    let width = area.width / per_row as u16;

    for (index, (label, value, _unit)) in boxes.iter().enumerate() {
        let column = index % per_row;
        let row = index / per_row;
        let x = area.x + column as u16 * width;
        let y = area.y + row as u16 * row_height;
        draw_stat_box(
            buf,
            Rect::new(x, y, width.saturating_sub(1), row_height),
            label,
            value,
            app,
        );
    }
}

fn draw_stat_box(buf: &mut Buffer, area: Rect, label: &str, value: &str, app: &App) {
    let theme = app.theme();
    if area.height == 0 || area.width < 4 {
        return;
    }
    let surface = Style::default().bg(theme.surface);
    fill_area(buf, area, surface);
    put(
        buf,
        area.x,
        area.y,
        &truncate(label, area.width as usize),
        Style::default().fg(theme.muted).bg(theme.surface),
        area,
    );
    if area.height > 1 {
        put_right(
            buf,
            area.x + area.width,
            area.y + 1,
            &truncate(value, area.width as usize),
            Style::default()
                .fg(theme.text)
                .bg(theme.surface)
                .add_modifier(Modifier::BOLD),
            area,
        );
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
        for _ in 0..3 {
            app.apply_snapshot(crate::model::Snapshot::fixture());
        }
        app.tab = crate::app::Tab::Performance;
        app
    }

    #[test]
    fn cpu_panel_shows_boxes_and_core_graphs() {
        let screen = render(&app(), 120, 40);
        assert!(screen.contains("Utilization"));
        assert!(screen.contains("Speed"));
        assert!(screen.contains("Processes"));
        assert!(screen.contains("Threads"));
        assert!(screen.contains("Handles"));
        assert!(screen.contains("Up time"));
        assert!(screen.contains("CPU 0"));
        assert!(screen.contains("60 seconds"));
    }

    #[test]
    fn single_core_hides_the_core_grid() {
        let mut app = app();
        let mut snapshot = crate::model::Snapshot::fixture();
        snapshot.cpu.logical = 1;
        snapshot.cpu.per_core = vec![42.0];
        app.apply_snapshot(snapshot);
        let screen = render(&app, 120, 40);
        assert!(!screen.contains("CPU 0"));
        assert!(screen.contains("Utilization"));
    }

    #[test]
    fn memory_panel_dashes_unknown_figures() {
        let mut app = app();
        app.performance.resource = ResourceChoice::Memory;
        let screen = render(&app, 120, 40);
        assert!(screen.contains("In use"));
        assert!(screen.contains("Available"));
        assert!(screen.contains("Compressed"));
        assert!(screen.contains("Swap used"));
    }

    #[test]
    fn disk_panel_names_the_busiest_disk() {
        let mut app = app();
        app.performance.resource = ResourceChoice::Disk;
        let screen = render(&app, 120, 40);
        assert!(screen.contains("disk0"));
        assert!(screen.contains("Read speed"));
        assert!(screen.contains("Active time"));
    }

    #[test]
    fn network_panel_shows_the_adapter() {
        let mut app = app();
        app.performance.resource = ResourceChoice::Network;
        let screen = render(&app, 120, 40);
        assert!(screen.contains("en0"));
        assert!(screen.contains("Receive"));
        assert!(screen.contains("Link speed"));
    }

    #[test]
    fn gpu_panel_says_unavailable() {
        let mut app = app();
        app.performance.resource = ResourceChoice::Gpu;
        let screen = render(&app, 120, 40);
        assert!(screen.contains("Not available on this system"));
    }

    #[test]
    fn narrow_layout_puts_the_rail_on_top() {
        let screen = render(&app(), 60, 24);
        assert!(screen.contains("Memory"));
        assert!(screen.contains("Utilization"));
    }
}
