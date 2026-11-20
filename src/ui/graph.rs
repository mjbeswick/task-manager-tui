//! The 60 second area graphs. Half blocks give two levels per cell.

use crate::model::{ChartStyle, Snapshot};
use crate::theme::{Glyphs, Theme};
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

/// A three-stop ramp: chart colour at the base, a dim tint in the middle,
/// and the surface at the top. `rise` is 0 at the base and 1 at the top.
pub fn ramp(edge: Color, bg: Color, rise: f32) -> Color {
    let mid = crate::theme::mix_linear(edge, bg, 0.22);
    let top = crate::theme::mix_linear(edge, bg, 0.50);
    let rise = rise.clamp(0.0, 1.0);
    if rise < 0.5 {
        crate::theme::mix_linear(edge, mid, rise * 2.0)
    } else {
        crate::theme::mix_linear(mid, top, (rise - 0.5) * 2.0)
    }
}

#[derive(Clone, Copy)]
pub struct FillSpec<'a> {
    pub edge: Color,
    pub bg: Color,
    pub theme: &'a Theme,
    pub gradient: bool,
}

/// One column's value on the 60 sample time axis. Wider plots interpolate
/// between samples; narrower plots keep the peak of the samples they cover.
pub fn sample_column(values: &[f32], column: usize, width: usize) -> Option<f32> {
    if values.is_empty() || width == 0 || column >= width {
        return None;
    }
    let len = values.len();
    let value_at = |slot: usize| values[len - 1 - slot];
    let start = column as f64 * HISTORY as f64 / width as f64;
    let end = (column as f64 + 1.0) * HISTORY as f64 / width as f64;
    if start >= len as f64 {
        return None;
    }
    let span = end - start;
    if span < 1.0 {
        let centre = (start + end) / 2.0;
        let index = centre.floor() as usize;
        if index >= len {
            return None;
        }
        let fraction = (centre - index as f64) as f32;
        let a = value_at(index);
        let b = if index + 1 < len {
            value_at(index + 1)
        } else {
            a
        };
        Some(a + (b - a) * fraction)
    } else {
        let first = start.floor() as usize;
        let last = (end.ceil() as usize).min(len);
        let mut peak = f32::MIN;
        for slot in first..last.max(first + 1) {
            if slot < len {
                peak = peak.max(value_at(slot));
            }
        }
        (peak != f32::MIN).then_some(peak)
    }
}

/// Draw the newest samples of `series` as a sparkline across `area`.
/// `max` of 0 autoscales like the main graph, with 10% headroom.
pub struct PlotSpec<'a> {
    pub max: f32,
    pub style: ChartStyle,
    pub color: Color,
    pub bg: Color,
    pub theme: &'a Theme,
    pub gradient: bool,
}

pub fn sparkline(buf: &mut Buffer, area: Rect, series: &Series, spec: PlotSpec<'_>) {
    let maximum = if spec.max > 0.0 {
        spec.max
    } else {
        (series.max() * 1.1).max(f32::EPSILON)
    };
    plot_series(buf, area, series, maximum, spec);
}

/// Plot one column per sample, newest on the right, without grid or labels.
fn plot_series(buf: &mut Buffer, area: Rect, series: &Series, maximum: f32, spec: PlotSpec<'_>) {
    let PlotSpec {
        style,
        color,
        bg,
        theme,
        gradient,
        ..
    } = spec;
    let width = area.width as usize;
    let height = area.height as usize;
    if width == 0 || height == 0 {
        return;
    }
    let values: Vec<f32> = series.values.iter().copied().collect();
    let count = values.len();
    let fractions: Vec<Option<f32>> = (0..width)
        .map(|column| {
            (column < count).then(|| (values[count - 1 - column] / maximum).clamp(0.0, 1.0))
        })
        .collect();
    let totals: Vec<usize> = fractions
        .iter()
        .map(|fraction| {
            fraction
                .map(|fraction| (fraction * (height * 8) as f32).round() as usize)
                .unwrap_or(0)
        })
        .collect();
    let glyphs = theme.glyphs();
    let mut dots = if matches!(style, ChartStyle::Line | ChartStyle::Dots) && !theme.ascii {
        Some(Dots::new(area.width, area.height))
    } else {
        None
    };
    let mut previous_dot: Option<(usize, usize)> = None;
    for column in 0..width {
        let Some(fraction) = fractions[column] else {
            continue;
        };
        let x = area.x + area.width - 1 - column as u16;
        match style {
            ChartStyle::Filled => fill_column(
                buf,
                Rect::new(x, area.y, 1, area.height),
                fraction,
                FillSpec {
                    edge: color,
                    bg,
                    theme,
                    gradient,
                },
            ),
            ChartStyle::Blocks => {
                let older = if column + 1 < width && fractions[column + 1].is_some() {
                    Some(totals[column + 1])
                } else {
                    None
                };
                blocks_column(
                    buf,
                    Rect::new(x, area.y, 1, area.height),
                    totals[column],
                    older,
                    FillSpec {
                        edge: color,
                        bg,
                        theme,
                        gradient,
                    },
                );
            }
            ChartStyle::Line | ChartStyle::Dots if dots.is_some() => {
                let grid = dots.as_mut().unwrap();
                let dx = (width - 1 - column) * 2 + 1;
                let rows = height * 4;
                let dy = rows - 1 - (fraction * (rows - 1) as f32).round() as usize;
                if style == ChartStyle::Line
                    && let Some((px, py)) = previous_dot
                {
                    grid.line(px, py, dx, dy, fraction);
                } else {
                    grid.set(dx, dy, fraction);
                }
                previous_dot = Some((dx, dy));
            }
            ChartStyle::Line | ChartStyle::Dots => {
                let level = (fraction * height as f32).ceil().clamp(1.0, height as f32) as u16;
                let row = area.y + area.height - level;
                buf[(x, row)]
                    .set_char(glyphs.bullet)
                    .set_style(ratatui::style::Style::default().fg(color).bg(bg));
            }
        }
    }
    if let Some(dots) = &dots {
        dots.flush(
            buf,
            area,
            FillSpec {
                edge: color,
                bg,
                theme,
                gradient,
            },
        );
    }
}

/// Paint one filled column, `fraction` of `height` rows tall, bottom-anchored.
/// The top cell resolves to an eighth in the edge colour; cells under it use a
/// lighter fill. ASCII and 16 colour modes round to half cells and skip the mix.
pub fn fill_column(buf: &mut Buffer, column: Rect, fraction: f32, spec: FillSpec<'_>) {
    let FillSpec {
        edge,
        bg,
        theme,
        gradient,
    } = spec;
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
        let rise = if height > 1 {
            offset as f32 / (height - 1) as f32
        } else {
            0.0
        };
        let colour = if coarse || !gradient {
            edge
        } else {
            ramp(edge, bg, 0.05 + rise * 0.95)
        };
        let style = ratatui::style::Style::default().fg(colour).bg(bg);
        let row = top + height - 1 - offset as u16;
        buf[(x, row)]
            .set_char(glyphs.eighths[units - 1])
            .set_style(style);
    }
}

/// Pick the top-cell glyph for a Blocks column. `own` and `other` are eighths
/// of fill in this row, newest and older. The fill is bottom-anchored, so a
/// slope flips left to right, never top to bottom.
fn slope_glyph(own: u8, other: u8, glyphs: &Glyphs) -> char {
    let eighth = |units: u8| glyphs.eighths[units.clamp(1, 8) as usize - 1];
    if own >= 8 {
        return glyphs.block_full;
    }
    let diff = own as i16 - other as i16;
    if other == 0 || diff.abs() <= 2 {
        return eighth(own);
    }
    if diff >= 6
        && let Some(ch) = glyphs.slope_rise
    {
        return ch;
    }
    if diff <= -6
        && let Some(ch) = glyphs.slope_fall
    {
        return ch;
    }
    if diff > 0
        && own >= 5
        && let Some(ch) = glyphs.block_rise
    {
        return ch;
    }
    if diff < 0
        && other >= 5
        && own <= 5
        && let Some(ch) = glyphs.block_fall
    {
        return ch;
    }
    eighth(own)
}

/// A Blocks column: eighths for the tip, triangles for a one-step slope, and
/// quadrants for a steeper join with the older column to the left.
fn blocks_column(
    buf: &mut Buffer,
    column: Rect,
    total: usize,
    older: Option<usize>,
    spec: FillSpec<'_>,
) {
    let FillSpec {
        edge,
        bg,
        theme,
        gradient,
    } = spec;
    let glyphs = theme.glyphs();
    let (x, top, height) = (column.x, column.y, column.height);
    let coarse = theme.ascii || theme.coarse;
    for offset in 0..height as usize {
        let units = total.saturating_sub(offset * 8).min(8);
        if units == 0 {
            break;
        }
        let rise = if height > 1 {
            offset as f32 / (height - 1) as f32
        } else {
            0.0
        };
        let colour = if coarse || !gradient {
            edge
        } else {
            ramp(edge, bg, 0.05 + rise * 0.95)
        };
        let is_top = total <= (offset + 1) * 8;
        let glyph = if units == 8 {
            glyphs.block_full
        } else if is_top {
            let own = units as u8;
            let other = older
                .map(|total| total.saturating_sub(offset * 8).min(8) as u8)
                .unwrap_or(own);
            slope_glyph(own, other, glyphs)
        } else {
            glyphs.eighths[units - 1]
        };
        let row = top + height - 1 - offset as u16;
        buf[(x, row)]
            .set_char(glyph)
            .set_style(ratatui::style::Style::default().fg(colour).bg(bg));
    }
}

/// A per-plot matrix of braille dots: two columns and four rows per cell.
struct Dots {
    width: usize,
    height: usize,
    mask: Vec<u8>,
    peaks: Vec<f32>,
}

impl Dots {
    fn new(cells_wide: u16, cells_tall: u16) -> Self {
        let width = cells_wide as usize * 2;
        let height = cells_tall as usize * 4;
        Self {
            width,
            height,
            mask: vec![0; width.div_ceil(2) * height.div_ceil(4)],
            peaks: vec![0.0; width.div_ceil(2) * height.div_ceil(4)],
        }
    }

    fn set(&mut self, x: usize, y: usize, peak: f32) {
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
        let cell = cy * columns + cx;
        self.mask[cell] |= bit;
        self.peaks[cell] = self.peaks[cell].max(peak);
    }

    /// Bresenham between two dots, so a line is continuous at dot resolution.
    fn line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, peak: f32) {
        let (mut x0, mut y0) = (x0 as isize, y0 as isize);
        let (x1, y1) = (x1 as isize, y1 as isize);
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;
        loop {
            if x0 >= 0 && y0 >= 0 {
                self.set(x0 as usize, y0 as usize, peak);
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

    fn flush(&self, buf: &mut Buffer, area: Rect, spec: FillSpec<'_>) {
        let FillSpec {
            edge,
            bg,
            theme,
            gradient,
        } = spec;
        let columns = self.width.div_ceil(2);
        for cy in 0..self.height.div_ceil(4) {
            for cx in 0..columns {
                let cell = cy * columns + cx;
                let mask = self.mask[cell];
                if mask == 0 {
                    continue;
                }
                let colour = if theme.coarse || !gradient {
                    edge
                } else {
                    ramp(edge, bg, (1.0 - self.peaks[cell]) * 0.85)
                };
                let style = ratatui::style::Style::default().fg(colour).bg(bg);
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
    pub gradient: bool,
}

pub fn draw(buf: &mut Buffer, area: Rect, series: &Series, theme: &Theme, spec: GraphSpec<'_>) {
    let GraphSpec {
        max,
        value_label,
        footnote,
        style,
        color,
        gradient,
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
    let values: Vec<f32> = series.values.iter().copied().collect();
    let width = plot.width as usize;
    let height = plot.height as usize;
    let mut previous: Option<usize> = None;
    let mut dots = if matches!(style, ChartStyle::Line | ChartStyle::Dots) && !theme.ascii {
        Some(Dots::new(plot.width, plot.height))
    } else {
        None
    };
    let mut previous_dot: Option<(usize, usize)> = None;
    let fractions: Vec<Option<f32>> = (0..width)
        .map(|column| {
            sample_column(&values, column, width).map(|value| (value / maximum).clamp(0.0, 1.0))
        })
        .collect();
    let totals: Vec<usize> = fractions
        .iter()
        .map(|fraction| {
            fraction
                .map(|fraction| (fraction * (height * 8) as f32).round() as usize)
                .unwrap_or(0)
        })
        .collect();

    // The time axis is always the last 60 samples spread over the plot width, so
    // a full history fills the chart on any terminal size.
    for column in 0..width {
        let Some(fraction) = fractions[column] else {
            previous = None;
            continue;
        };
        let value = fraction * maximum;
        let x = plot.x + plot.width - 1 - column as u16;
        let halves = ((value / maximum).clamp(0.0, 1.0) * (height * 2) as f32).round() as usize;
        let level = halves.div_ceil(2).clamp(0, height);

        match style {
            ChartStyle::Filled => {
                fill_column(
                    buf,
                    Rect::new(x, column_top, 1, plot.height),
                    fraction,
                    FillSpec {
                        edge: color,
                        bg: theme.surface,
                        theme,
                        gradient,
                    },
                );
            }
            ChartStyle::Blocks => {
                let older = if column + 1 < width && fractions[column + 1].is_some() {
                    Some(totals[column + 1])
                } else {
                    None
                };
                blocks_column(
                    buf,
                    Rect::new(x, column_top, 1, plot.height),
                    totals[column],
                    older,
                    FillSpec {
                        edge: color,
                        bg: theme.surface,
                        theme,
                        gradient,
                    },
                );
            }
            ChartStyle::Line | ChartStyle::Dots if dots.is_some() => {
                let grid = dots.as_mut().unwrap();
                let dx = (width - 1 - column) * 2 + 1;
                let rows = height * 4;
                let dy = rows - 1 - (fraction * (rows - 1) as f32).round() as usize;
                if style == ChartStyle::Line
                    && let Some((px, py)) = previous_dot
                {
                    grid.line(px, py, dx, dy, fraction);
                } else {
                    grid.set(dx, dy, fraction);
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
        dots.flush(
            buf,
            plot,
            FillSpec {
                edge: color,
                bg: theme.surface,
                theme,
                gradient,
            },
        );
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
                gradient: true,
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
                gradient: true,
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
                gradient: true,
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
                gradient: true,
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
                gradient: true,
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
                gradient: true,
            },
        );
        assert_ne!(buffer[(9, 3)].symbol().to_string(), "█");
    }

    #[test]
    fn braille_dots_land_in_the_right_cells() {
        let mut dots = Dots::new(2, 1);
        dots.set(0, 0, 0.5);
        dots.set(1, 0, 0.5);
        dots.set(3, 3, 0.5);
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 1));
        dots.flush(
            &mut buffer,
            Rect::new(0, 0, 2, 1),
            FillSpec {
                edge: theme.graph_fill,
                bg: theme.surface,
                theme: &theme,
                gradient: true,
            },
        );
        assert_eq!(buffer[(0, 0)].symbol(), "⠉");
        assert_eq!(buffer[(1, 0)].symbol(), "⢀");
    }

    #[test]
    fn braille_cells_take_the_colour_of_their_peak() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let mut dots = Dots::new(2, 1);
        dots.set(0, 3, 0.0);
        dots.set(2, 0, 1.0);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 1));
        dots.flush(
            &mut buffer,
            Rect::new(0, 0, 2, 1),
            FillSpec {
                edge: theme.graph_fill,
                bg: theme.surface,
                theme: &theme,
                gradient: true,
            },
        );
        assert_eq!(
            buffer[(1, 0)].fg,
            theme.graph_fill,
            "peak cell is brightest"
        );
        assert_ne!(buffer[(0, 0)].fg, buffer[(1, 0)].fg);
    }

    #[test]
    fn coarse_mode_uses_one_braille_colour() {
        let mut theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        theme.coarse = true;
        let mut dots = Dots::new(2, 1);
        dots.set(0, 3, 0.1);
        dots.set(3, 0, 0.9);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 1));
        dots.flush(
            &mut buffer,
            Rect::new(0, 0, 2, 1),
            FillSpec {
                edge: theme.graph_fill,
                bg: theme.surface,
                theme: &theme,
                gradient: true,
            },
        );
        assert_eq!(buffer[(0, 0)].fg, theme.graph_fill);
        assert_eq!(buffer[(1, 0)].fg, theme.graph_fill);
    }

    #[test]
    fn braille_line_is_continuous() {
        let mut dots = Dots::new(2, 1);
        dots.line(0, 3, 3, 0, 0.5);
        let columns = dots.width.div_ceil(2);
        assert!(dots.mask[0] & 0x40 != 0, "start dot is set");
        assert!(dots.mask[columns - 1] & 0x08 != 0, "end dot is set");
        assert_eq!(dots.mask.iter().filter(|m| **m != 0).count(), 2);
    }

    #[test]
    fn ramp_runs_from_edge_to_surface_through_a_dim_midpoint() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let edge = theme.graph_fill;
        let bg = theme.surface;
        assert_eq!(ramp(edge, bg, 0.0), edge);
        let top = ramp(edge, bg, 1.0);
        assert_ne!(top, bg, "the top keeps some hue");
        let middle = ramp(edge, bg, 0.5);
        let Color::Rgb(r, g, b) = middle else {
            panic!("expected rgb");
        };
        assert!(
            g > r && g > b,
            "the midpoint keeps the teal hue: {r},{g},{b}"
        );
        assert_ne!(middle, edge);
    }

    #[test]
    fn an_eight_row_fill_uses_eight_colours() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let area = Rect::new(0, 0, 1, 8);
        let mut buffer = Buffer::empty(area);
        fill_column(
            &mut buffer,
            area,
            1.0,
            FillSpec {
                edge: theme.graph_fill,
                bg: theme.surface,
                theme: &theme,
                gradient: true,
            },
        );
        let colours: std::collections::HashSet<_> = buffer
            .content
            .iter()
            .filter(|cell| cell.symbol() != " ")
            .map(|cell| cell.fg)
            .collect();
        assert!(colours.len() >= 8, "only {} colours", colours.len());
    }

    #[test]
    fn braille_cells_are_coloured_by_their_peak() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let mut dots = Dots::new(2, 1);
        dots.set(0, 3, 0.1);
        dots.set(3, 0, 0.9);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 1));
        dots.flush(
            &mut buffer,
            Rect::new(0, 0, 2, 1),
            FillSpec {
                edge: theme.graph_fill,
                bg: theme.surface,
                theme: &theme,
                gradient: true,
            },
        );
        assert_ne!(buffer[(0, 0)].fg, buffer[(1, 0)].fg);
    }

    fn sixty() -> Vec<f32> {
        (0..HISTORY).map(|i| i as f32).collect()
    }

    #[test]
    fn downscaling_keeps_the_peak_in_a_narrow_plot() {
        let mut values = sixty();
        values[HISTORY - 1 - 33] = 100.0;
        // Ten columns cover six slots each; the spike sits in column 5.
        let peak = sample_column(&values, 5, 10).unwrap();
        assert_eq!(peak, 100.0);
        let ordinary = sample_column(&values, 0, 10).unwrap();
        assert!(ordinary < 100.0);
    }

    #[test]
    fn upscaling_interpolates_between_samples() {
        let values = sixty();
        let a = sample_column(&values, 100, 120).unwrap();
        let b = sample_column(&values, 101, 120).unwrap();
        assert!(a > b, "older columns hold smaller samples: {a} vs {b}");
        assert!(a > 8.0 && a < 10.0, "interpolated between 9 and 8: {a}");
    }

    #[test]
    fn columns_older_than_the_history_are_empty() {
        assert_eq!(sample_column(&[1.0], 0, 60), Some(1.0));
        assert_eq!(sample_column(&[1.0], 1, 60), None);
        assert_eq!(sample_column(&[], 0, 60), None);
    }

    fn one_column(theme: &crate::theme::Theme, fraction: f32) -> Buffer {
        let area = Rect::new(0, 0, 1, 2);
        let mut buffer = Buffer::empty(area);
        fill_column(
            &mut buffer,
            Rect::new(0, 0, 1, 2),
            fraction,
            FillSpec {
                edge: theme.graph_fill,
                bg: theme.surface,
                theme,
                gradient: true,
            },
        );
        buffer
    }

    #[test]
    fn partial_cells_resolve_to_eighths() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        // 11/16 of two rows: one full cell, then three eighths on top.
        let buffer = one_column(&theme, 11.0 / 16.0);
        assert_eq!(buffer[(0, 1)].symbol(), "█");
        assert_eq!(buffer[(0, 0)].symbol(), "▃");
        assert_ne!(
            buffer[(0, 0)].fg,
            theme.graph_fill,
            "the tip follows the ramp"
        );
        assert_ne!(buffer[(0, 1)].fg, buffer[(0, 0)].fg, "rows differ");
    }

    #[test]
    fn the_gradient_has_no_bright_rim() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let area = Rect::new(0, 0, 1, 8);
        let mut buffer = Buffer::empty(area);
        fill_column(
            &mut buffer,
            area,
            1.0,
            FillSpec {
                edge: theme.graph_fill,
                bg: theme.surface,
                theme: &theme,
                gradient: true,
            },
        );
        let distance = |a: ratatui::style::Color, b: ratatui::style::Color| {
            let (a, b) = (crate::theme::indexed_rgb(a), crate::theme::indexed_rgb(b));
            (a.0 as i32 - b.0 as i32).abs()
                + (a.1 as i32 - b.1 as i32).abs()
                + (a.2 as i32 - b.2 as i32).abs()
        };
        let tip = buffer[(0, 0)].fg;
        let below = buffer[(0, 1)].fg;
        assert_ne!(tip, theme.graph_fill, "the tip is blended");
        assert!(
            distance(tip, theme.graph_fill) >= distance(below, theme.graph_fill),
            "the tip is no brighter than the row below"
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
                gradient: true,
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

    fn blocks_cell(total: usize, older: usize) -> Buffer {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let area = Rect::new(0, 0, 1, 8);
        let mut buffer = Buffer::empty(area);
        blocks_column(
            &mut buffer,
            area,
            total,
            Some(older),
            FillSpec {
                edge: theme.graph_fill,
                bg: theme.surface,
                theme: &theme,
                gradient: true,
            },
        );
        buffer
    }

    fn symbols(buffer: &Buffer) -> Vec<char> {
        buffer
            .content
            .iter()
            .filter_map(|cell| cell.symbol().chars().next())
            .collect()
    }

    #[test]
    fn blocks_slope_glyph_matrix() {
        let g = crate::theme::UNICODE_GLYPHS;
        let rows = [
            "▁▁▁▁▁▙▙◣◣",
            "▂▂▂▂▂▙▙▙◣",
            "▃▃▃▃▃▃▙▙▙",
            "▄▄▄▄▄▄▄▙▙",
            "▅▟▟▅▅▅▅▅▙",
            "▆▟▟▟▆▆▆▆▆",
            "▇◢▟▟▟▇▇▇▇",
            "█████████",
        ];
        for (index, row) in rows.iter().enumerate() {
            let own = index as u8 + 1;
            for (other, expected) in row.chars().enumerate() {
                assert_eq!(
                    slope_glyph(own, other as u8, &g),
                    expected,
                    "own {own} other {other}"
                );
            }
        }
    }

    #[test]
    fn blocks_glyphs_stay_bottom_anchored_and_mirror() {
        let g = crate::theme::UNICODE_GLYPHS;
        let forbidden = ['▛', '▜', '▘', '▝', '▚', '▞', '◤', '◥', '▖', '▗'];
        let mirror = |ch: char| match ch {
            '▟' => Some('▙'),
            '▙' => Some('▟'),
            '◢' => Some('◣'),
            '◣' => Some('◢'),
            _ => None,
        };
        // The ceiling arm deliberately wins over symmetry, so skip 8 here.
        for own in 1..=7u8 {
            for other in 1..=7u8 {
                let glyph = slope_glyph(own, other, &g);
                assert!(!forbidden.contains(&glyph), "{own}/{other} -> {glyph}");
                if let Some(mirrored) = mirror(glyph) {
                    assert_eq!(
                        slope_glyph(other, own, &g),
                        mirrored,
                        "mirror for {own}/{other}"
                    );
                }
            }
        }
    }

    #[test]
    fn blocks_falling_quadrant_keeps_the_floor() {
        // Own 4/8 at the tip against a full older column.
        let chars = symbols(&blocks_cell(20, 40));
        assert!(chars.contains(&'▙'), "falling quadrant: {chars:?}");
    }

    #[test]
    fn blocks_rising_quadrant_points_up_right() {
        // Own 6/8 against 1/8.
        let chars = symbols(&blocks_cell(14, 9));
        assert!(chars.contains(&'▟'), "rising quadrant: {chars:?}");
    }

    #[test]
    fn blocks_triangles_are_only_for_steep_steps() {
        let fall = symbols(&blocks_cell(34, 40));
        assert!(fall.contains(&'◣'), "steep fall: {fall:?}");
        let rise = symbols(&blocks_cell(39, 33));
        assert!(rise.contains(&'◢'), "steep rise: {rise:?}");
        let small = symbols(&blocks_cell(34, 35));
        assert!(
            small.contains(&'▂'),
            "small step stays an eighth: {small:?}"
        );
        assert!(!small.iter().any(|ch| "◢◣".contains(*ch)));
    }

    #[test]
    fn blocks_do_not_flare_above_an_empty_neighbour() {
        // Own tip 5/8; the older column is three whole cells.
        let chars = symbols(&blocks_cell(37, 24));
        assert!(chars.contains(&'▅'), "eighth block: {chars:?}");
        assert!(!chars.iter().any(|ch| "▙▛▜▟◢◣◤◥".contains(*ch)));
    }

    #[test]
    fn blocks_ceiling_stays_full() {
        let chars = symbols(&blocks_cell(64, 22));
        assert!(chars.contains(&'█'), "full ceiling: {chars:?}");
    }

    #[test]
    fn native_width_keeps_a_spike_exact() {
        let mut values = vec![10.0; HISTORY];
        values[HISTORY - 1] = 100.0;
        assert_eq!(sample_column(&values, 0, HISTORY), Some(100.0));
        assert_eq!(sample_column(&values, 1, HISTORY), Some(10.0));
    }

    #[test]
    fn blocks_draw_joins_the_newest_column() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let area = Rect::new(0, 0, HISTORY as u16, 8);
        let mut buffer = Buffer::empty(area);
        let mut series = Series::default();
        for _ in 0..30 {
            series.push(100.0);
        }
        for _ in 0..30 {
            series.push(20.0);
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
                style: ChartStyle::Blocks,
                color: theme.graph_fill,
                gradient: true,
            },
        );
        assert!(symbols(&buffer).contains(&'▙'), "falling join at the right");
    }

    fn sparkline_symbols(style: ChartStyle) -> Vec<char> {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let area = Rect::new(0, 0, 20, 1);
        let mut buffer = Buffer::empty(area);
        let mut series = Series::default();
        for value in [20.0, 90.0, 30.0, 70.0, 10.0, 100.0] {
            series.push(value);
        }
        sparkline(
            &mut buffer,
            area,
            &series,
            PlotSpec {
                max: 100.0,
                style,
                color: theme.graph_fill,
                bg: theme.surface,
                theme: &theme,
                gradient: true,
            },
        );
        symbols(&buffer)
    }

    #[test]
    fn sparkline_uses_the_selected_style() {
        let filled = sparkline_symbols(ChartStyle::Filled);
        assert!(filled.iter().any(|ch| "▁▂▃▄▅▆▇█".contains(*ch)));
        assert!(
            !filled
                .iter()
                .any(|ch| ('\u{2800}'..='\u{28ff}').contains(ch))
        );

        let blocks = sparkline_symbols(ChartStyle::Blocks);
        assert!(
            blocks.iter().any(|ch| "▙▛▜▟◢◣◤◥".contains(*ch)),
            "blocks slope: {blocks:?}"
        );

        for style in [ChartStyle::Line, ChartStyle::Dots] {
            let chars = sparkline_symbols(style);
            assert!(
                chars
                    .iter()
                    .any(|ch| ('\u{2800}'..='\u{28ff}').contains(ch)),
                "{style:?} braille: {chars:?}"
            );
        }
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
            PlotSpec {
                max: 100.0,
                style: ChartStyle::Filled,
                color: theme.graph_fill,
                bg: theme.surface,
                theme: &theme,
                gradient: true,
            },
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
    fn a_flat_fill_uses_one_colour() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let area = Rect::new(0, 0, 1, 8);
        let mut buffer = Buffer::empty(area);
        fill_column(
            &mut buffer,
            area,
            1.0,
            FillSpec {
                edge: theme.graph_fill,
                bg: theme.surface,
                theme: &theme,
                gradient: false,
            },
        );
        let colours: std::collections::HashSet<_> = buffer
            .content
            .iter()
            .filter(|cell| cell.symbol() != " ")
            .map(|cell| cell.fg)
            .collect();
        assert_eq!(colours.len(), 1, "flat fill");
        assert!(colours.contains(&theme.graph_fill));
    }

    #[test]
    fn filled_columns_fade_toward_the_top() {
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
        let area = Rect::new(0, 0, 1, 8);
        let mut buffer = Buffer::empty(area);
        fill_column(
            &mut buffer,
            area,
            1.0,
            FillSpec {
                edge: theme.graph_fill,
                bg: theme.surface,
                theme: &theme,
                gradient: true,
            },
        );
        let below_tip = buffer[(0, 1)].fg;
        assert_ne!(below_tip, theme.graph_fill, "a gradient ran");
        assert_ne!(below_tip, theme.surface, "the top is not washed out");
        let distance = |a: ratatui::style::Color, b: ratatui::style::Color| {
            let (a, b) = (crate::theme::indexed_rgb(a), crate::theme::indexed_rgb(b));
            (a.0 as i32 - b.0 as i32).abs()
                + (a.1 as i32 - b.1 as i32).abs()
                + (a.2 as i32 - b.2 as i32).abs()
        };
        assert!(
            distance(below_tip, theme.graph_fill) >= 25,
            "the top is visibly lighter than the base"
        );
        assert!(
            distance(below_tip, theme.surface) >= 10,
            "the top is not the surface colour"
        );
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
