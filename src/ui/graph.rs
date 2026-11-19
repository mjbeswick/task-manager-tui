//! The 60 second area graphs. Half blocks give two levels per cell.

use crate::model::{ChartStyle, Snapshot};
use crate::theme::Theme;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use std::collections::VecDeque;

pub const HISTORY: usize = 60;

#[derive(Debug, Default, Clone)]
pub struct Series {
    pub values: VecDeque<f32>,
}

impl Series {
    pub fn push(&mut self, value: f32) {
        if self.values.len() == HISTORY {
            self.values.pop_front();
        }
        self.values.push_back(value.max(0.0));
    }

    pub fn max(&self) -> f32 {
        self.values.iter().copied().fold(0.0, f32::max)
    }

    pub fn last(&self) -> f32 {
        self.values.back().copied().unwrap_or(0.0)
    }
}

#[derive(Debug, Default, Clone)]
pub struct GraphStore {
    pub cpu: Series,
    pub memory: Series,
    pub disk: Series,
    pub disk_read: Series,
    pub disk_write: Series,
    pub network: Series,
    pub net_rx: Series,
    pub net_tx: Series,
    pub gpu: Series,
    pub cores: Vec<Series>,
}

impl GraphStore {
    pub fn push(&mut self, snapshot: &Snapshot) {
        self.cpu.push(snapshot.cpu.total);
        let memory_pct = if snapshot.memory.total > 0 {
            snapshot.memory.used as f32 / snapshot.memory.total as f32 * 100.0
        } else {
            0.0
        };
        self.memory.push(memory_pct);
        let disk_read: f64 = snapshot.disks.iter().map(|d| d.read_bps).sum();
        let disk_write: f64 = snapshot.disks.iter().map(|d| d.write_bps).sum();
        self.disk_read.push(disk_read as f32);
        self.disk_write.push(disk_write as f32);
        self.disk.push((disk_read + disk_write) as f32);
        let rx: f64 = snapshot.networks.iter().map(|n| n.rx_bps).sum();
        let tx: f64 = snapshot.networks.iter().map(|n| n.tx_bps).sum();
        self.net_rx.push(rx as f32);
        self.net_tx.push(tx as f32);
        self.network.push((rx + tx) as f32);
        let gpu = snapshot.gpus.iter().map(|g| g.usage).fold(0.0, f32::max);
        self.gpu.push(gpu);

        if self.cores.len() != snapshot.cpu.per_core.len() {
            self.cores
                .resize_with(snapshot.cpu.per_core.len(), Series::default);
        }
        for (series, value) in self.cores.iter_mut().zip(&snapshot.cpu.per_core) {
            series.push(*value);
        }
    }
}

/// Blend `a` toward `b`. Non-RGB colours are returned unchanged.
pub fn mix(a: Color, b: Color, toward_b: f32) -> Color {
    match (a, b) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let m = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * toward_b).round() as u8;
            Color::Rgb(m(r1, r2), m(g1, g2), m(b1, b2))
        }
        _ => a,
    }
}

/// Draw the newest samples of `series` as a sparkline across `area`.
/// `max` of 0 autoscales like the main graph, with 10% headroom.
pub fn sparkline(
    buf: &mut Buffer,
    area: Rect,
    series: &Series,
    max: f32,
    color: Color,
    bg: Color,
    theme: &Theme,
) {
    let maximum = if max > 0.0 {
        max
    } else {
        (series.max() * 1.1).max(f32::EPSILON)
    };
    let count = series.values.len();
    for column in 0..(area.width as usize).min(count) {
        let value = series.values[count - 1 - column];
        let x = area.x + area.width - 1 - column as u16;
        fill_column(
            buf,
            Rect::new(x, area.y, 1, area.height),
            value / maximum,
            color,
            bg,
            theme,
        );
    }
}

/// Paint one filled column, `fraction` of `height` rows tall, bottom-anchored.
/// The top cell resolves to an eighth in the edge colour; cells under it use a
/// lighter fill. ASCII and 16 colour modes round to half cells and skip the mix.
pub fn fill_column(
    buf: &mut Buffer,
    column: Rect,
    fraction: f32,
    edge: Color,
    bg: Color,
    theme: &Theme,
) {
    let (x, top, height) = (column.x, column.y, column.height);
    let glyphs = theme.glyphs();
    let coarse = theme.ascii || theme.coarse;
    let mut total = (fraction.clamp(0.0, 1.0) * height as f32 * 8.0).round() as usize;
    if coarse {
        total = (total + 2) / 4 * 4;
    }
    for offset in 0..height as usize {
        let units = total.saturating_sub(offset * 8).min(8);
        if units == 0 {
            break;
        }
        let is_top = total <= (offset + 1) * 8;
        let rise = if height > 1 {
            offset as f32 / (height - 1) as f32
        } else {
            0.0
        };
        let colour = if is_top || coarse {
            edge
        } else {
            mix(edge, bg, 0.15 + 0.6 * rise)
        };
        let style = ratatui::style::Style::default().fg(colour).bg(bg);
        let row = top + height - 1 - offset as u16;
        buf[(x, row)]
            .set_char(glyphs.eighths[units - 1])
            .set_style(style);
    }
}

/// A per-plot matrix of braille dots: two columns and four rows per cell.
struct Dots {
    width: usize,
    height: usize,
    mask: Vec<u8>,
}

impl Dots {
    fn new(cells_wide: u16, cells_tall: u16) -> Self {
        let width = cells_wide as usize * 2;
        let height = cells_tall as usize * 4;
        Self {
            width,
            height,
            mask: vec![0; width.div_ceil(2) * height.div_ceil(4)],
        }
    }

    fn set(&mut self, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            return;
        }
        let (cx, cy) = (x / 2, y / 4);
        let (dx, dy) = (x % 2, y % 4);
        // Braille dot order: left column 1,2,3,7; right column 4,5,6,8.
        let bit = match (dy, dx) {
            (0, 0) => 0x01,
            (1, 0) => 0x02,
            (2, 0) => 0x04,
            (0, 1) => 0x08,
            (1, 1) => 0x10,
            (2, 1) => 0x20,
            (3, 0) => 0x40,
            (3, 1) => 0x80,
            _ => 0,
        };
        let columns = self.width.div_ceil(2);
        self.mask[cy * columns + cx] |= bit;
    }

    /// Bresenham between two dots, so a line is continuous at dot resolution.
    fn line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        let (mut x0, mut y0) = (x0 as isize, y0 as isize);
        let (x1, y1) = (x1 as isize, y1 as isize);
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;
        loop {
            if x0 >= 0 && y0 >= 0 {
                self.set(x0 as usize, y0 as usize);
            }
            if x0 == x1 && y0 == y1 {
                break;
            }
            let twice = 2 * error;
            if twice >= dy {
                error += dy;
                x0 += sx;
            }
            if twice <= dx {
                error += dx;
                y0 += sy;
            }
        }
    }

    fn flush(&self, buf: &mut Buffer, area: Rect, style: ratatui::style::Style) {
        let columns = self.width.div_ceil(2);
        for cy in 0..self.height.div_ceil(4) {
            for cx in 0..columns {
                let mask = self.mask[cy * columns + cx];
                if mask == 0 {
                    continue;
                }
                if let Some(ch) = char::from_u32(0x2800 + mask as u32) {
                    buf[(area.x + cx as u16, area.y + cy as u16)]
                        .set_char(ch)
                        .set_style(style);
                }
            }
        }
    }
}

/// Draw one graph into `area`. `max` of 0 means autoscale; percentage graphs pass 100.
pub struct GraphSpec<'a> {
    pub max: f32,
    pub value_label: &'a str,
    pub footnote: &'a str,
    pub style: ChartStyle,
    pub color: ratatui::style::Color,
}

pub fn draw(buf: &mut Buffer, area: Rect, series: &Series, theme: &Theme, spec: GraphSpec<'_>) {
    let GraphSpec {
        max,
        value_label,
        footnote,
        style,
        color,
    } = spec;
    if area.width < 4 || area.height < 2 {
        return;
    }
    let plot = if footnote.is_empty() {
        area
    } else {
        Rect::new(area.x, area.y, area.width, area.height - 1)
    };
    let surface = theme.surface_style();
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            buf[(x, y)].set_symbol(" ");
            buf[(x, y)].set_style(surface);
        }
    }

    // Grid first, so the fill covers it.
    let grid = ratatui::style::Style::default()
        .fg(theme.graph_grid)
        .bg(theme.surface);
    for step in 1..4 {
        let y = plot.y + plot.height * step / 4;
        if y >= plot.y + plot.height {
            continue;
        }
        for x in plot.x..plot.x + plot.width {
            buf[(x, y)].set_char('─').set_style(grid);
        }
    }
    for step in 1..6 {
        let x = plot.x + plot.width * step / 6;
        if x >= plot.x + plot.width {
            continue;
        }
        for y in plot.y..plot.y + plot.height {
            buf[(x, y)].set_char('│').set_style(grid);
        }
    }

    let maximum = if max > 0.0 {
        max
    } else {
        let observed = series.max();
        if observed <= 0.0 { 1.0 } else { observed * 1.1 }
    };

    let glyphs = theme.glyphs();
    let fill = ratatui::style::Style::default().fg(color).bg(theme.surface);
    let column_top = plot.y;
    let count = series.values.len();
    let width = plot.width as usize;
    let height = plot.height as usize;
    let mut previous: Option<usize> = None;
    let mut dots = if matches!(style, ChartStyle::Line | ChartStyle::Dots) && !theme.ascii {
        Some(Dots::new(plot.width, plot.height))
    } else {
        None
    };
    let mut previous_dot: Option<(usize, usize)> = None;

    // The time axis is always the last 60 samples spread over the plot width, so
    // a full history fills the chart on any terminal size.
    for column in 0..width {
        let slot = (column as u64 * HISTORY as u64) / width as u64;
        if slot as usize >= count {
            previous = None;
            continue;
        }
        let value = series.values[count - 1 - slot as usize];
        let x = plot.x + plot.width - 1 - column as u16;
        let halves = ((value / maximum).clamp(0.0, 1.0) * (height * 2) as f32).round() as usize;
        let level = halves.div_ceil(2).clamp(0, height);

        match style {
            ChartStyle::Filled => {
                fill_column(
                    buf,
                    Rect::new(x, column_top, 1, plot.height),
                    value / maximum,
                    color,
                    theme.surface,
                    theme,
                );
            }
            ChartStyle::Line | ChartStyle::Dots if dots.is_some() => {
                let grid = dots.as_mut().unwrap();
                let dx = (width - 1 - column) * 2 + 1;
                let rows = height * 4;
                let dy = rows
                    - 1
                    - ((value / maximum).clamp(0.0, 1.0) * (rows - 1) as f32).round() as usize;
                if style == ChartStyle::Line
                    && let Some((px, py)) = previous_dot
                {
                    grid.line(px, py, dx, dy);
                } else {
                    grid.set(dx, dy);
                }
                previous_dot = Some((dx, dy));
            }
            ChartStyle::Line | ChartStyle::Dots => {
                let marker_row = plot.y + plot.height - level.clamp(1, height) as u16;
                buf[(x, marker_row)].set_char(glyphs.bullet).set_style(fill);
                if style == ChartStyle::Line
                    && let Some(previous) = previous
                {
                    let (low, high) = if previous < level {
                        (previous, level)
                    } else {
                        (level, previous)
                    };
                    for step in low + 1..high {
                        let row = plot.y + plot.height - step.clamp(1, height) as u16;
                        buf[(x, row)].set_char(glyphs.vline).set_style(fill);
                    }
                }
            }
        }
        previous = Some(level);
    }
    if let Some(dots) = &dots {
        dots.flush(buf, plot, fill);
    }

    let label_style = ratatui::style::Style::default()
        .fg(theme.muted)
        .bg(theme.surface);
    let value_style = ratatui::style::Style::default()
        .fg(theme.text)
        .bg(theme.surface);
    if !value_label.is_empty() {
        let width = value_label.chars().count() as u16;
        let x = (plot.x + plot.width).saturating_sub(width + 1);
        buf.set_stringn(x, plot.y, value_label, plot.width as usize, value_style);
    }
    if !footnote.is_empty() {
        let y = area.y + area.height - 1;
        buf.set_stringn(area.x, y, footnote, area.width as usize, label_style);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn series_keeps_sixty_samples() {
        let mut series = Series::default();
        for value in 0..100 {
            series.push(value as f32);
        }
        assert_eq!(series.values.len(), HISTORY);
        assert_eq!(series.last(), 99.0);
        assert_eq!(series.values.front(), Some(&40.0));
    }

    #[test]
    fn graph_store_tracks_cores() {
        let mut snapshot = Snapshot::fixture();
        let mut store = GraphStore::default();
        store.push(&snapshot);
        assert_eq!(store.cores.len(), 8);
        assert_eq!(store.cpu.last(), 27.0);
        snapshot.cpu.per_core.truncate(4);
        store.push(&snapshot);
        assert_eq!(store.cores.len(), 4);
    }

    #[test]
    fn full_value_fills_the_height() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let area = Rect::new(0, 0, 10, 4);
        let mut buffer = Buffer::empty(area);
        let mut series = Series::default();
        series.push(100.0);
        draw(
            &mut buffer,
            area,
            &series,
            &theme,
            GraphSpec {
                max: 100.0,
                value_label: "",
                footnote: "",
                style: ChartStyle::Filled,
                color: theme.graph_fill,
            },
        );
        let bottom = buffer[(9, 3)].symbol().to_string();
        let top = buffer[(9, 0)].symbol().to_string();
        assert_eq!(bottom, "█");
        assert_eq!(top, "█");
    }

    #[test]
    fn empty_history_leaves_the_left_side_blank() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let area = Rect::new(0, 0, 20, 6);
        let mut buffer = Buffer::empty(area);
        let mut series = Series::default();
        series.push(50.0);
        draw(
            &mut buffer,
            area,
            &series,
            &theme,
            GraphSpec {
                max: 100.0,
                value_label: "",
                footnote: "",
                style: ChartStyle::Filled,
                color: theme.graph_fill,
            },
        );
        // The newest sample sits at the right edge, the left is untouched grid.
        assert_eq!(buffer[(19, 5)].symbol().to_string(), "█");
        assert_ne!(buffer[(0, 5)].symbol().to_string(), "█");
    }

    #[test]
    fn a_full_history_fills_the_whole_width() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let area = Rect::new(0, 0, 100, 10);
        let mut buffer = Buffer::empty(area);
        let mut series = Series::default();
        for _ in 0..HISTORY {
            series.push(50.0);
        }
        draw(
            &mut buffer,
            area,
            &series,
            &theme,
            GraphSpec {
                max: 100.0,
                value_label: "",
                footnote: "",
                style: ChartStyle::Filled,
                color: theme.graph_fill,
            },
        );
        assert_eq!(buffer[(0, 9)].symbol(), "█", "left edge is drawn");
        assert_eq!(buffer[(99, 9)].symbol(), "█", "right edge is drawn");
    }

    #[test]
    fn a_short_history_only_fills_the_right_side() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let area = Rect::new(0, 0, 100, 10);
        let mut buffer = Buffer::empty(area);
        let mut series = Series::default();
        for _ in 0..(HISTORY / 2) {
            series.push(50.0);
        }
        draw(
            &mut buffer,
            area,
            &series,
            &theme,
            GraphSpec {
                max: 100.0,
                value_label: "",
                footnote: "",
                style: ChartStyle::Filled,
                color: theme.graph_fill,
            },
        );
        assert_eq!(buffer[(99, 9)].symbol(), "█");
        assert_ne!(buffer[(0, 9)].symbol(), "█");
    }

    #[test]
    fn line_style_marks_the_point_without_filling_below() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, true);
        let area = Rect::new(0, 0, 60, 10);
        let mut buffer = Buffer::empty(area);
        let mut series = Series::default();
        series.push(100.0);
        draw(
            &mut buffer,
            area,
            &series,
            &theme,
            GraphSpec {
                max: 100.0,
                value_label: "",
                footnote: "",
                style: ChartStyle::Line,
                color: theme.graph_fill,
            },
        );
        assert_eq!(buffer[(59, 0)].symbol(), theme.glyphs().bullet.to_string());
        assert_ne!(buffer[(59, 9)].symbol(), "█");
    }

    #[test]
    fn zero_value_draws_nothing() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let area = Rect::new(0, 0, 10, 4);
        let mut buffer = Buffer::empty(area);
        let mut series = Series::default();
        series.push(0.0);
        draw(
            &mut buffer,
            area,
            &series,
            &theme,
            GraphSpec {
                max: 100.0,
                value_label: "",
                footnote: "",
                style: ChartStyle::Filled,
                color: theme.graph_fill,
            },
        );
        assert_ne!(buffer[(9, 3)].symbol().to_string(), "█");
    }

    #[test]
    fn braille_dots_land_in_the_right_cells() {
        let mut dots = Dots::new(2, 1);
        dots.set(0, 0);
        dots.set(1, 0);
        dots.set(3, 3);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 1));
        dots.flush(
            &mut buffer,
            Rect::new(0, 0, 2, 1),
            ratatui::style::Style::default(),
        );
        assert_eq!(buffer[(0, 0)].symbol(), "⠉");
        assert_eq!(buffer[(1, 0)].symbol(), "⢀");
    }

    #[test]
    fn braille_line_is_continuous() {
        let mut dots = Dots::new(2, 1);
        dots.line(0, 3, 3, 0);
        let columns = dots.width.div_ceil(2);
        assert!(dots.mask[0] & 0x40 != 0, "start dot is set");
        assert!(dots.mask[columns - 1] & 0x08 != 0, "end dot is set");
        assert_eq!(dots.mask.iter().filter(|m| **m != 0).count(), 2);
    }

    fn one_column(theme: &crate::theme::Theme, fraction: f32) -> Buffer {
        let area = Rect::new(0, 0, 1, 2);
        let mut buffer = Buffer::empty(area);
        fill_column(
            &mut buffer,
            Rect::new(0, 0, 1, 2),
            fraction,
            theme.graph_fill,
            theme.surface,
            theme,
        );
        buffer
    }

    #[test]
    fn partial_cells_resolve_to_eighths_with_an_edge() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        // 11/16 of two rows: one full cell, then three eighths on top.
        let buffer = one_column(&theme, 11.0 / 16.0);
        assert_eq!(buffer[(0, 1)].symbol(), "█");
        assert_eq!(buffer[(0, 0)].symbol(), "▃");
        assert_eq!(buffer[(0, 0)].fg, theme.graph_fill, "top cell is the edge");
        assert_ne!(
            buffer[(0, 1)].fg,
            theme.graph_fill,
            "cells below are the fill"
        );
    }

    #[test]
    fn line_style_draws_braille_not_bullets() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let area = Rect::new(0, 0, 30, 8);
        let mut buffer = Buffer::empty(area);
        let mut series = Series::default();
        series.push(30.0);
        series.push(80.0);
        draw(
            &mut buffer,
            area,
            &series,
            &theme,
            GraphSpec {
                max: 100.0,
                value_label: "",
                footnote: "",
                style: ChartStyle::Line,
                color: theme.graph_fill,
            },
        );
        let symbols: Vec<char> = buffer
            .content
            .iter()
            .filter_map(|cell| cell.symbol().chars().next())
            .collect();
        assert!(
            symbols
                .iter()
                .any(|ch| ('\u{2800}'..='\u{28ff}').contains(ch)),
            "no braille cell was drawn"
        );
        assert!(!symbols.contains(&theme.glyphs().bullet));
    }

    #[test]
    fn sparkline_fades_with_the_same_gradient() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let area = Rect::new(0, 0, 2, 6);
        let mut buffer = Buffer::empty(area);
        let mut series = Series::default();
        series.push(100.0);
        series.push(100.0);
        sparkline(
            &mut buffer,
            area,
            &series,
            100.0,
            theme.graph_fill,
            theme.surface,
            &theme,
        );
        let colours: std::collections::HashSet<_> = buffer
            .content
            .iter()
            .filter(|cell| cell.symbol() != " ")
            .map(|cell| cell.fg)
            .collect();
        assert!(colours.len() > 1, "the sparkline is one flat colour");
    }

    #[test]
    fn filled_columns_fade_toward_the_top() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let buffer = one_column(&theme, 1.0);
        assert_ne!(buffer[(0, 0)].fg, buffer[(0, 1)].fg, "a gradient ran");
        assert_ne!(buffer[(0, 1)].fg, theme.surface, "the base stays coloured");
    }

    #[test]
    fn coarse_and_ascii_modes_keep_half_steps() {
        let mut theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Dark, false);
        theme.coarse = true;
        let buffer = one_column(&theme, 11.0 / 16.0);
        assert_eq!(buffer[(0, 0)].symbol(), "▄");
        assert_eq!(
            buffer[(0, 1)].fg,
            theme.graph_fill,
            "no mixed fill at 16 colours"
        );
        let ascii = crate::theme::Theme::from_name(crate::theme::ThemeName::Dark, true);
        let buffer = one_column(&ascii, 11.0 / 16.0);
        assert!(buffer.content.iter().all(|c| c.symbol().is_ascii()));
        assert_eq!(buffer[(0, 0)].symbol(), "_");
    }
}
