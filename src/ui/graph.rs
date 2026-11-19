//! The 60 second area graphs. Half blocks give two levels per cell.

use crate::model::{ChartStyle, Snapshot};
use crate::theme::Theme;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
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
    let count = series.values.len();
    let width = plot.width as usize;
    let height = plot.height as usize;
    let mut previous: Option<usize> = None;

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
                for offset in 0..height {
                    let units = halves.saturating_sub(offset * 2);
                    let row = plot.y + plot.height - 1 - offset as u16;
                    if units >= 2 {
                        buf[(x, row)].set_char(glyphs.block_full).set_style(fill);
                    } else if units == 1 {
                        buf[(x, row)].set_char(glyphs.block_upper).set_style(fill);
                    }
                }
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
        let theme = crate::theme::Theme::from_name(crate::theme::ThemeName::Light, false);
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
}
