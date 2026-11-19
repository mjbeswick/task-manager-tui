//! Window chrome: menu bar, tab strip, status bar, and the frame that owns them.

use crate::app::{App, MENUS, Tab};
use crate::theme::Theme;
use crate::ui::format::{DASH, bit_rate, pct, truncate};
use crate::ui::hits::{HitMap, Target};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};

pub const MIN_WIDTH: u16 = 40;
pub const MIN_HEIGHT: u16 = 12;

pub fn draw(frame: &mut Frame, app: &App, hits: &mut HitMap) {
    hits.clear();
    let area = frame.area();
    let buf = frame.buffer_mut();
    let theme = app.theme();

    fill_area(buf, area, Style::default().bg(theme.bg));
    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
        let message = format!(
            "taskmgr needs at least {MIN_WIDTH} columns and {MIN_HEIGHT} rows (now {}x{})",
            area.width, area.height
        );
        let y = area.y + area.height / 2;
        let x = area.x + area.width.saturating_sub(message.chars().count() as u16) / 2;
        put(buf, x, y, &message, theme.base(), area);
        app.depth.apply(buf, theme);
        return;
    }

    let [menu_row, tab_row, content, status_row] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(area);

    draw_menu_bar(buf, menu_row, app, hits);
    draw_tabs(buf, tab_row, app, hits);

    match app.tab {
        Tab::Processes => crate::ui::processes::draw(buf, content, app, hits),
        Tab::Performance => crate::ui::performance::draw(buf, content, app, hits),
        Tab::AppHistory => crate::ui::app_history::draw(buf, content, app, hits),
        Tab::Startup => crate::ui::startup::draw(buf, content, app, hits),
        Tab::Users => crate::ui::users::draw(buf, content, app, hits),
        Tab::Details => crate::ui::details::draw(buf, content, app, hits),
        Tab::Services => crate::ui::services::draw(buf, content, app, hits),
    }

    draw_status_bar(buf, status_row, app, hits);

    if let Some(menu) = app.menu {
        draw_menu_dropdown(buf, area, app, menu, hits);
    }
    if let Some(dialog) = &app.dialog {
        crate::ui::dialog::draw(buf, area, app, dialog, hits);
    }
    app.depth.apply(buf, theme);
}

pub fn fill_area(buf: &mut Buffer, area: Rect, style: Style) {
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            buf[(x, y)].set_symbol(" ");
            buf[(x, y)].set_style(style);
        }
    }
}

/// Draw a single-line Windows 10 panel and return its inner area.
pub fn panel(buf: &mut Buffer, area: Rect, title: &str, theme: &Theme) -> Rect {
    if area.width < 2 || area.height < 2 {
        return area;
    }
    let glyphs = theme.glyphs();
    let style = Style::default().fg(theme.border).bg(theme.surface);
    let inner_style = Style::default().fg(theme.text).bg(theme.surface);
    fill_area(buf, area, inner_style);
    let right = area.x + area.width - 1;
    let bottom = area.y + area.height - 1;
    for x in area.x + 1..right {
        buf[(x, area.y)].set_char(glyphs.hline).set_style(style);
        buf[(x, bottom)].set_char(glyphs.hline).set_style(style);
    }
    for y in area.y + 1..bottom {
        buf[(area.x, y)].set_char(glyphs.vline).set_style(style);
        buf[(right, y)].set_char(glyphs.vline).set_style(style);
    }
    buf[(area.x, area.y)].set_char(glyphs.tl).set_style(style);
    buf[(right, area.y)].set_char(glyphs.tr).set_style(style);
    buf[(area.x, bottom)].set_char(glyphs.bl).set_style(style);
    buf[(right, bottom)].set_char(glyphs.br).set_style(style);

    if !title.is_empty() && area.width > 4 {
        let text = format!(" {title} ");
        buf.set_stringn(
            area.x + 2,
            area.y,
            text,
            area.width.saturating_sub(4) as usize,
            Style::default()
                .fg(theme.muted)
                .bg(theme.surface)
                .add_modifier(Modifier::BOLD),
        );
    }
    Rect::new(
        area.x + 1,
        area.y + 1,
        area.width.saturating_sub(2),
        area.height.saturating_sub(2),
    )
}

pub fn put(buf: &mut Buffer, x: u16, y: u16, text: &str, style: Style, clip: Rect) {
    if y < clip.y || y >= clip.y + clip.height || x >= clip.x + clip.width {
        return;
    }
    let max = (clip.x + clip.width).saturating_sub(x) as usize;
    buf.set_stringn(x, y, text, max, style);
}

/// Put text, then draw its first match of the active filter in the accent.
/// ponytail: one cell per char, so wide CJK names misplace the highlight.
pub fn put_match(
    buf: &mut Buffer,
    x: u16,
    y: u16,
    text: &str,
    style: Style,
    clip: Rect,
    app: &App,
) {
    put(buf, x, y, text, style, clip);
    let needle = app.filter().trim();
    if needle.is_empty() {
        return;
    }
    if let Some((start, end)) = crate::ui::format::find_match(text, needle) {
        let accent = style.fg(app.theme().accent).add_modifier(Modifier::BOLD);
        for i in start..end {
            let cx = x + i as u16;
            if cx < clip.x + clip.width && y >= clip.y && y < clip.y + clip.height {
                buf[(cx, y)].set_style(accent);
            }
        }
    }
}

/// Put right-aligned text inside the clip.
pub fn put_right(buf: &mut Buffer, right: u16, y: u16, text: &str, style: Style, clip: Rect) {
    let width = text.chars().count() as u16;
    let x = right.saturating_sub(width).max(clip.x);
    put(buf, x, y, text, style, clip);
}

fn draw_menu_bar(buf: &mut Buffer, area: Rect, app: &App, hits: &mut HitMap) {
    let theme = app.theme();
    fill_area(buf, area, Style::default().bg(theme.bg));
    let mut x = area.x;
    for (index, (label, _)) in MENUS.iter().enumerate() {
        let active = app.menu.is_some_and(|menu| menu.menu == index);
        let style = if active {
            Style::default()
                .fg(ratatui::style::Color::White)
                .bg(theme.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(theme.text)
                .bg(theme.bg)
                .add_modifier(Modifier::BOLD)
        };
        let text = format!(" {label} ");
        let width = text.chars().count() as u16;
        put(buf, x, area.y, &text, style, area);
        hits.add(Rect::new(x, area.y, width, 1), Target::Menu(index));
        x += width;
    }
    if app.tab == Tab::Performance {
        let title = "Task Manager";
        put_right(
            buf,
            area.x + area.width - 1,
            area.y,
            title,
            Style::default()
                .fg(theme.muted)
                .bg(theme.bg)
                .add_modifier(Modifier::BOLD),
            area,
        );
        return;
    }
    let free = (area.x + area.width).saturating_sub(x + 2);
    draw_filter_field(buf, area, free, app, hits);
}

/// The filter field at the right of the menu bar. It shrinks to its key hint
/// when the menus leave too little room, and menus are never cut short.
fn draw_filter_field(buf: &mut Buffer, bar: Rect, free: u16, app: &App, hits: &mut HitMap) {
    let theme = app.theme();
    let glyphs = theme.glyphs();
    let text = app.filter();
    let editing = app.filter_editing;
    let active = editing || !text.is_empty();
    let field = Style::default().fg(theme.text).bg(theme.surface);
    let key = Style::default()
        .fg(if active { theme.accent } else { theme.muted })
        .bg(theme.surface)
        .add_modifier(Modifier::BOLD);
    let right = bar.x + bar.width;
    if free < 3 {
        return;
    }
    if free < 16 {
        let rect = Rect::new(right - 3, bar.y, 3, 1);
        fill_area(buf, rect, field);
        put(buf, rect.x + 1, bar.y, "/", key, rect);
        hits.add(rect, Target::FilterField);
        return;
    }
    let width = free.min(30);
    let rect = Rect::new(right - width, bar.y, width, 1);
    fill_area(buf, rect, field);
    put(buf, rect.x + 1, bar.y, "/", key, rect);
    hits.add(rect, Target::FilterField);
    let start = rect.x + 3;
    // Room for the text, a cursor or clear mark, and a trailing space.
    let room = (width - 6) as usize;
    if text.is_empty() && !editing {
        put(
            buf,
            start,
            bar.y,
            "Filter",
            Style::default().fg(theme.muted).bg(theme.surface),
            rect,
        );
        return;
    }
    let count = text.chars().count();
    let shown: String = text.chars().skip(count.saturating_sub(room)).collect();
    put(buf, start, bar.y, &shown, field, rect);
    let end = start + shown.chars().count() as u16;
    if editing {
        let cursor = Style::default().fg(theme.accent).bg(theme.surface);
        // Linux console fonts lack the thin bar, so 16 colour mode uses `|`.
        let glyph = if theme.coarse { '|' } else { glyphs.cursor };
        put(buf, end, bar.y, &glyph.to_string(), cursor, rect);
    }
    if !text.is_empty() {
        let x = right - 2;
        put(buf, x, bar.y, &glyphs.clear.to_string(), key, rect);
        hits.add(Rect::new(x - 1, bar.y, 3, 1), Target::FilterClear);
    }
}

fn draw_tabs(buf: &mut Buffer, area: Rect, app: &App, hits: &mut HitMap) {
    let theme = app.theme();
    fill_area(buf, area, Style::default().bg(theme.bg));
    let glyphs = theme.glyphs();
    let compact = area.width < 90;
    let mut x = area.x;
    for tab in Tab::ALL {
        let active = tab == app.tab;
        let style = if active {
            Style::default()
                .fg(theme.accent)
                .bg(theme.bg)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(theme.muted).bg(theme.bg)
        };
        let text = if compact {
            format!("{} ", tab.short_label())
        } else {
            format!("{} ", tab.label())
        };
        if x + text.chars().count() as u16 > area.x + area.width {
            break;
        }
        let width = text.chars().count() as u16;
        put(buf, x, area.y, &text, style, area);
        hits.add(Rect::new(x, area.y, width, 1), Target::Tab(tab));
        x += width;
        if tab != Tab::Services {
            let separator = if compact {
                " ".to_string()
            } else {
                format!("{} ", glyphs.vline)
            };
            put(
                buf,
                x,
                area.y,
                &separator,
                Style::default().fg(theme.border).bg(theme.bg),
                area,
            );
            x += separator.chars().count() as u16;
        }
    }
    let underline = area.y + area.height;
    let _ = underline;
}

fn draw_status_bar(buf: &mut Buffer, area: Rect, app: &App, hits: &mut HitMap) {
    let theme = app.theme();
    let bar = Style::default().bg(theme.surface).fg(theme.muted);
    fill_area(buf, area, bar);

    let action = if app.tab == Tab::Processes {
        if app.process.more_details {
            "Fewer details"
        } else {
            "More details"
        }
    } else {
        ""
    };
    let position = status_position(app);

    let snapshot = &app.snapshot;
    let memory = if snapshot.memory.total > 0 {
        pct(snapshot.memory.used as f32 / snapshot.memory.total as f32 * 100.0)
    } else {
        DASH.to_string()
    };
    let disk: f64 = snapshot.disks.iter().map(|d| d.io_bps()).sum();
    let network: f64 = snapshot.networks.iter().map(|n| n.rx_bps + n.tx_bps).sum();
    let speed = match app.settings.update_speed.as_str() {
        "high" => "High",
        "low" => "Low",
        "paused" => "Paused",
        _ => "Normal",
    };

    // Metrics are dropped from the least important end until the bar fits.
    let mut parts = vec![
        format!("CPU {}", pct(snapshot.cpu.total)),
        format!("Memory {memory}"),
        format!("Disk {}", crate::ui::format::rate(disk)),
        format!("Network {}", bit_rate(network)),
        speed.to_string(),
    ];
    if !snapshot.gpus.is_empty() {
        let gpu = snapshot.gpus.iter().map(|g| g.usage).fold(0.0, f32::max);
        parts.push(format!("GPU {}", pct(gpu)));
    }

    let mut show_action = !action.is_empty();
    let mut show_position = position.is_some();
    let summary = loop {
        let summary = parts.join("   ");
        let left = left_width(action, show_action, &position, show_position);
        if left + summary.chars().count() as u16 + 2 <= area.width {
            break summary;
        }
        if parts.len() > 2 {
            parts.pop();
            continue;
        }
        if show_position {
            show_position = false;
            continue;
        }
        if show_action {
            show_action = false;
            continue;
        }
        break summary;
    };

    let mut x = area.x + 1;
    if show_action {
        let style = Style::default()
            .fg(theme.accent)
            .bg(theme.surface)
            .add_modifier(Modifier::UNDERLINED);
        let width = action.chars().count() as u16;
        put(buf, x, area.y, action, style, area);
        hits.add(Rect::new(x, area.y, width, 1), Target::FewerDetails);
        x += width + 2;
    }
    if show_position && let Some(position) = &position {
        let width = position.chars().count() as u16;
        put(buf, x, area.y, position, bar, area);
        x += width + 2;
    }

    // A notice takes the middle when there is room for both without clipping.
    if let Some(notice) = app.notices.last() {
        let room = area
            .width
            .saturating_sub((x - area.x) + summary.chars().count() as u16 + 3);
        if room > 20 {
            let text = truncate(notice, room.min(60) as usize);
            put(
                buf,
                x,
                area.y,
                text.trim_end(),
                Style::default().fg(theme.text).bg(theme.surface),
                area,
            );
        }
    }

    put_right(buf, area.x + area.width - 2, area.y, &summary, bar, area);
}

fn left_width(
    action: &str,
    show_action: bool,
    position: &Option<String>,
    show_position: bool,
) -> u16 {
    let mut width = 0;
    if show_action {
        width += action.chars().count() as u16 + 2;
    }
    if show_position && let Some(position) = position {
        width += position.chars().count() as u16 + 2;
    }
    width
}

/// Where the selection sits inside the active list, when it has rows.
fn status_position(app: &App) -> Option<String> {
    let (selected, len) = match app.tab {
        Tab::Processes => (app.process.selected, app.process.items.len()),
        Tab::Details => (app.details.selected, app.details_rows().len()),
        Tab::Services => (app.services.selected, app.service_rows().len()),
        Tab::Startup => (app.startup.selected, app.startup_rows().len()),
        Tab::Users => (app.users.selected, app.user_rows().len()),
        Tab::AppHistory => (app.app_history.selected, app.history_rows().len()),
        Tab::Performance => return None,
    };
    (len > 1).then(|| format!("{}/{}", selected + 1, len))
}

fn draw_menu_dropdown(
    buf: &mut Buffer,
    area: Rect,
    app: &App,
    menu: crate::app::MenuState,
    hits: &mut HitMap,
) {
    let theme = app.theme();
    let items = MENUS[menu.menu].1;
    let width = items
        .iter()
        .map(|(label, _)| label.chars().count())
        .max()
        .unwrap_or(10) as u16
        + 4;
    let height = items.len() as u16 + 2;

    let mut x = area.x;
    for (label, _) in MENUS.iter().take(menu.menu) {
        x += label.chars().count() as u16 + 2;
    }
    x = x.min(area.x + area.width.saturating_sub(width));
    let y = area.y + 1;
    let rect = Rect::new(x, y, width, height.min(area.height.saturating_sub(y)));
    let inner = panel(buf, rect, "", theme);

    for (index, (label, _)) in items.iter().enumerate() {
        let row = inner.y + index as u16;
        if row >= inner.y + inner.height {
            break;
        }
        let selected = index == menu.item;
        let style = if selected {
            Style::default()
                .fg(ratatui::style::Color::White)
                .bg(theme.accent)
        } else {
            Style::default().fg(theme.text).bg(theme.surface)
        };
        if selected {
            for x in inner.x..inner.x + inner.width {
                buf[(x, row)].set_symbol(" ");
                buf[(x, row)].set_style(style);
            }
        }
        put(buf, inner.x + 1, row, label, style, inner);
        hits.add(
            Rect::new(inner.x, row, inner.width, 1),
            Target::MenuItem(menu.menu, index),
        );
    }
}

/// Split a width into a flexible name column and right-aligned fixed columns.
pub fn columns(width: u16, fixed: &[(&'static str, u16)]) -> (u16, Vec<(&'static str, u16)>) {
    let used: u16 = fixed.iter().map(|(_, w)| *w).sum();
    let name = width.saturating_sub(used);
    (name.max(6), fixed.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::App;
    use crate::store::settings::Settings;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    fn screen_for(tab: crate::app::Tab, width: u16, filter: &str, editing: bool) -> String {
        let (mut app, _) = App::new(Settings::default(), false);
        app.apply_snapshot(crate::model::Snapshot::fixture());
        app.tab = tab;
        if let Some(f) = app.filter_mut() {
            f.push_str(filter);
        }
        app.filter_editing = editing;
        crate::ui::test_util::render(&app, width, 30)
    }

    #[test]
    fn filter_field_sits_in_the_menu_bar() {
        use crate::app::Tab;
        let bar = |s: String| s.lines().next().unwrap().to_string();
        let wide = bar(screen_for(Tab::Processes, 120, "", false));
        assert!(wide.trim_end().ends_with("/ Filter"), "{wide:?}");
        assert!(!wide.contains("Task Manager"));
        let perf = bar(screen_for(Tab::Performance, 120, "", false));
        assert!(perf.contains("Task Manager") && !perf.contains("Filter"));
        let set = bar(screen_for(Tab::Services, 120, "nginx", false));
        assert!(set.contains("/ nginx") && set.contains('×'), "{set:?}");
        let editing = bar(screen_for(Tab::Startup, 120, "rect", true));
        assert!(editing.contains("rect▏"), "{editing:?}");
        // At 40 columns the menus stay whole and the field shrinks to its key.
        let narrow = bar(screen_for(Tab::Processes, 40, "", false));
        assert!(
            narrow.starts_with(" File  Options  View  Help "),
            "{narrow:?}"
        );
        assert!(narrow.trim_end().ends_with('/'), "{narrow:?}");
        assert!(!narrow.contains("Filter"));
    }

    #[test]
    fn filter_matches_are_drawn_in_the_accent() {
        let (mut app, _) = App::new(Settings::default(), false);
        app.apply_snapshot(crate::model::Snapshot::fixture());
        app.tab = crate::app::Tab::Details;
        app.details.filter = "DOW".into();
        let mut hits = crate::ui::hits::HitMap::default();
        let mut terminal = Terminal::new(TestBackend::new(120, 30)).unwrap();
        terminal.draw(|frame| draw(frame, &app, &mut hits)).unwrap();
        let buffer = terminal.backend().buffer().clone();
        let screen = crate::ui::test_util::render(&app, 120, 30);
        let y = screen
            .lines()
            .position(|l| l.starts_with("WindowServer"))
            .unwrap() as u16;
        let accent = app.theme().accent;
        let colours: Vec<_> = (0..12).map(|x| buffer[(x, y)].fg == accent).collect();
        // "Win[dow]Server": only characters 3 to 5 are highlighted.
        let expected: Vec<bool> = (0..12).map(|x| (3..6).contains(&x)).collect();
        assert_eq!(colours, expected);
    }

    #[test]
    fn reduced_depth_frames_hold_no_rgb() {
        use crate::app::Tab;
        use crate::theme::{ColorDepth, ThemeName};
        for depth in [ColorDepth::Ansi256, ColorDepth::Ansi16] {
            for theme in [ThemeName::Light, ThemeName::Dark] {
                for tab in [Tab::Processes, Tab::Performance] {
                    let (mut app, _) = App::new(Settings::default(), false);
                    app.apply_snapshot(crate::model::Snapshot::fixture());
                    app.set_theme(theme);
                    app.set_depth(depth);
                    app.tab = tab;
                    let mut hits = crate::ui::hits::HitMap::default();
                    let mut terminal = Terminal::new(TestBackend::new(100, 30)).unwrap();
                    terminal.draw(|frame| draw(frame, &app, &mut hits)).unwrap();
                    let rgb = terminal.backend().buffer().content.iter().any(|c| {
                        matches!(c.fg, ratatui::style::Color::Rgb(..))
                            || matches!(c.bg, ratatui::style::Color::Rgb(..))
                    });
                    assert!(!rgb, "{depth:?} {theme:?} {tab:?} left an RGB colour");
                    if depth == ColorDepth::Ansi16 {
                        let indexed = terminal.backend().buffer().content.iter().any(|c| {
                            matches!(c.fg, ratatui::style::Color::Indexed(_))
                                || matches!(c.bg, ratatui::style::Color::Indexed(_))
                        });
                        assert!(!indexed, "{theme:?} {tab:?} left the 16 colour palette");
                    }
                }
            }
        }
    }

    fn render(width: u16, height: u16) -> String {
        let (mut app, _) = App::new(Settings::default(), false);
        app.apply_snapshot(crate::model::Snapshot::fixture());
        let mut hits = crate::ui::hits::HitMap::default();
        let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
        terminal.draw(|frame| draw(frame, &app, &mut hits)).unwrap();
        let buffer = terminal.backend().buffer().clone();
        let mut out = String::new();
        for y in 0..height {
            for x in 0..width {
                out.push_str(buffer[(x, y)].symbol());
            }
            out.push('\n');
        }
        out
    }

    #[test]
    fn chrome_contains_menu_tabs_and_status() {
        let screen = render(120, 40);
        assert!(screen.contains("File"));
        assert!(screen.contains("Processes"));
        assert!(screen.contains("App history"));
        assert!(
            screen.contains("/ Filter"),
            "list tabs show the filter field"
        );
        assert!(screen.contains("CPU 27.0%"));
        assert!(screen.contains("Fewer details"));
    }

    #[test]
    fn narrow_terminal_keeps_the_tab_strip() {
        let screen = render(60, 20);
        assert!(screen.contains("Proc"));
        assert!(screen.contains("Services"));
        let wide = render(120, 20);
        assert!(wide.contains("Performance"));
        assert!(wide.contains("App history"));
    }

    #[test]
    fn below_minimum_shows_a_message() {
        let screen = render(30, 8);
        assert!(screen.contains("needs at least 40"));
        assert!(!screen.contains("Performance"));
    }

    #[test]
    fn columns_keep_a_minimum_name_width() {
        let (name, fixed) = columns(40, &[("CPU", 8), ("Memory", 10)]);
        assert_eq!(name, 22);
        assert_eq!(fixed.len(), 2);
        let (name, _) = columns(10, &[("CPU", 8)]);
        assert_eq!(name, 6);
    }
}
