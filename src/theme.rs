//! Windows 10 palette and glyph set.

use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeName {
    Light,
    Dark,
}

impl ThemeName {
    pub fn label(self) -> &'static str {
        match self {
            ThemeName::Light => "Light",
            ThemeName::Dark => "Dark",
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            ThemeName::Light => ThemeName::Dark,
            ThemeName::Dark => ThemeName::Light,
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "light" => Some(ThemeName::Light),
            "dark" => Some(ThemeName::Dark),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub name: ThemeName,
    pub bg: Color,
    pub surface: Color,
    pub text: Color,
    pub muted: Color,
    pub border: Color,
    pub accent: Color,
    pub selection: Color,
    pub graph_fill: Color,
    pub graph_grid: Color,
    pub good: Color,
    pub warn: Color,
    pub bad: Color,
    pub ascii: bool,
}

pub const LIGHT: Theme = Theme {
    name: ThemeName::Light,
    bg: Color::Rgb(0xF0, 0xF0, 0xF0),
    surface: Color::Rgb(0xFF, 0xFF, 0xFF),
    text: Color::Rgb(0x00, 0x00, 0x00),
    muted: Color::Rgb(0x60, 0x5E, 0x5C),
    border: Color::Rgb(0xCC, 0xCC, 0xCC),
    accent: Color::Rgb(0x00, 0x78, 0xD7),
    selection: Color::Rgb(0xCC, 0xE8, 0xFF),
    graph_fill: Color::Rgb(0x2B, 0xA9, 0x8E),
    graph_grid: Color::Rgb(0xE0, 0xE0, 0xE0),
    good: Color::Rgb(0x10, 0x7C, 0x10),
    warn: Color::Rgb(0x9D, 0x5A, 0x00),
    bad: Color::Rgb(0xC4, 0x2B, 0x1C),
    ascii: false,
};

pub const DARK: Theme = Theme {
    name: ThemeName::Dark,
    bg: Color::Rgb(0x20, 0x20, 0x20),
    surface: Color::Rgb(0x27, 0x27, 0x27),
    text: Color::Rgb(0xFF, 0xFF, 0xFF),
    muted: Color::Rgb(0xA0, 0xA0, 0xA0),
    border: Color::Rgb(0x3A, 0x3A, 0x3A),
    accent: Color::Rgb(0x00, 0x78, 0xD7),
    selection: Color::Rgb(0x0A, 0x3D, 0x62),
    graph_fill: Color::Rgb(0x4C, 0xC2, 0xA8),
    graph_grid: Color::Rgb(0x3A, 0x3A, 0x3A),
    good: Color::Rgb(0x6C, 0xC6, 0x6C),
    warn: Color::Rgb(0xE0, 0xA0, 0x40),
    bad: Color::Rgb(0xF0, 0x70, 0x60),
    ascii: false,
};

/// Characters that differ between unicode and ASCII modes.
#[derive(Debug, Clone, Copy)]
pub struct Glyphs {
    pub hline: char,
    pub vline: char,
    pub tl: char,
    pub tr: char,
    pub bl: char,
    pub br: char,
    pub tee_left: char,
    pub tee_right: char,
    pub expanded: char,
    pub collapsed: char,
    pub sort_asc: char,
    pub sort_desc: char,
    pub checked: char,
    pub unchecked: char,
    pub block_full: char,
    pub block_upper: char,
    pub block_lower: char,
    pub bullet: char,
}

pub const UNICODE_GLYPHS: Glyphs = Glyphs {
    hline: '─',
    vline: '│',
    tl: '┌',
    tr: '┐',
    bl: '└',
    br: '┘',
    tee_left: '┤',
    tee_right: '├',
    expanded: '▾',
    collapsed: '▸',
    sort_asc: '▲',
    sort_desc: '▼',
    checked: '■',
    unchecked: '□',
    block_full: '█',
    block_upper: '▀',
    block_lower: '▄',
    bullet: '•',
};

pub const ASCII_GLYPHS: Glyphs = Glyphs {
    hline: '-',
    vline: '|',
    tl: '+',
    tr: '+',
    bl: '+',
    br: '+',
    tee_left: '+',
    tee_right: '+',
    expanded: 'v',
    collapsed: '>',
    sort_asc: '^',
    sort_desc: 'v',
    checked: 'X',
    unchecked: ' ',
    block_full: '#',
    block_upper: '"',
    block_lower: '_',
    bullet: '*',
};

impl Theme {
    pub fn from_name(name: ThemeName, ascii: bool) -> Self {
        let mut theme = match name {
            ThemeName::Light => LIGHT,
            ThemeName::Dark => DARK,
        };
        theme.ascii = ascii;
        theme
    }

    pub fn glyphs(&self) -> &'static Glyphs {
        if self.ascii {
            &ASCII_GLYPHS
        } else {
            &UNICODE_GLYPHS
        }
    }

    pub fn base(&self) -> Style {
        Style::default().fg(self.text).bg(self.bg)
    }

    pub fn surface_style(&self) -> Style {
        Style::default().fg(self.text).bg(self.surface)
    }

    pub fn muted_style(&self) -> Style {
        Style::default().fg(self.muted).bg(self.bg)
    }

    pub fn accent_style(&self) -> Style {
        Style::default().fg(self.accent).bg(self.bg)
    }

    pub fn selected_style(&self) -> Style {
        Style::default()
            .fg(self.text)
            .bg(self.selection)
            .add_modifier(Modifier::BOLD)
    }

    pub fn header_style(&self) -> Style {
        Style::default()
            .fg(self.muted)
            .bg(self.surface)
            .add_modifier(Modifier::BOLD)
    }

    pub fn danger(&self) -> Style {
        Style::default().fg(self.bad).bg(self.bg)
    }
}

/// How many colours the terminal can show.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorDepth {
    #[default]
    TrueColor,
    Ansi256,
    Ansi16,
}

impl ColorDepth {
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "truecolor" | "24bit" => Some(ColorDepth::TrueColor),
            "256" => Some(ColorDepth::Ansi256),
            "16" => Some(ColorDepth::Ansi16),
            _ => None,
        }
    }

    /// `COLORTERM` wins, then `TERM`. Unknown terminals get 256 colours, because
    /// SSH drops `COLORTERM` and nearly every modern emulator handles 256.
    pub fn detect(colorterm: Option<&str>, term: Option<&str>) -> Self {
        if colorterm
            .is_some_and(|v| matches!(v.to_ascii_lowercase().as_str(), "truecolor" | "24bit"))
        {
            return ColorDepth::TrueColor;
        }
        match term.unwrap_or("") {
            "linux" | "dumb" => ColorDepth::Ansi16,
            t if t.starts_with("vt") => ColorDepth::Ansi16,
            _ => ColorDepth::Ansi256,
        }
    }

    pub fn from_env() -> Self {
        let colorterm = std::env::var("COLORTERM").ok();
        let term = std::env::var("TERM").ok();
        Self::detect(colorterm.as_deref(), term.as_deref())
    }

    /// Map one colour to this depth.
    pub fn map(self, color: Color, theme: &Theme) -> Color {
        match (self, color) {
            (ColorDepth::TrueColor, _) | (_, Color::Reset) => color,
            (ColorDepth::Ansi256, Color::Rgb(r, g, b)) => Color::Indexed(xterm256(r, g, b)),
            (ColorDepth::Ansi16, Color::Rgb(..)) => ansi16(color, theme),
            _ => color,
        }
    }

    /// Map every cell of a drawn buffer to this depth, once per frame.
    pub fn apply(self, buf: &mut ratatui::buffer::Buffer, theme: &Theme) {
        if self == ColorDepth::TrueColor {
            return;
        }
        for cell in buf.content.iter_mut() {
            cell.fg = self.map(cell.fg, theme);
            cell.bg = self.map(cell.bg, theme);
        }
    }
}

const CUBE: [u8; 6] = [0, 95, 135, 175, 215, 255];

fn dist(a: (u8, u8, u8), b: (u8, u8, u8)) -> u32 {
    let d = |x: u8, y: u8| (x as i32 - y as i32).pow(2) as u32;
    d(a.0, b.0) + d(a.1, b.1) + d(a.2, b.2)
}

fn nearest_level(v: u8) -> usize {
    (0..6)
        .min_by_key(|&i| (CUBE[i] as i32 - v as i32).abs())
        .unwrap()
}

/// Nearest xterm 256 palette index, from the colour cube or the grey ramp.
pub fn xterm256(r: u8, g: u8, b: u8) -> u8 {
    let (ri, gi, bi) = (nearest_level(r), nearest_level(g), nearest_level(b));
    let cube = (CUBE[ri], CUBE[gi], CUBE[bi]);
    let cube_index = 16 + 36 * ri + 6 * gi + bi;
    let mean = (r as u32 + g as u32 + b as u32) / 3;
    let grey_step = (mean.saturating_sub(3) / 10).min(23) as u8;
    let grey = 8 + 10 * grey_step;
    if dist((r, g, b), (grey, grey, grey)) < dist((r, g, b), cube) {
        232 + grey_step
    } else {
        cube_index as u8
    }
}

/// RGB of an indexed or named colour as xterm draws it by default.
pub fn indexed_rgb(color: Color) -> (u8, u8, u8) {
    let base: [(u8, u8, u8); 16] = [
        (0, 0, 0),
        (205, 0, 0),
        (0, 205, 0),
        (205, 205, 0),
        (0, 0, 238),
        (205, 0, 205),
        (0, 205, 205),
        (229, 229, 229),
        (127, 127, 127),
        (255, 0, 0),
        (0, 255, 0),
        (255, 255, 0),
        (92, 92, 255),
        (255, 0, 255),
        (0, 255, 255),
        (255, 255, 255),
    ];
    let named = [
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::Gray,
        Color::DarkGray,
        Color::LightRed,
        Color::LightGreen,
        Color::LightYellow,
        Color::LightBlue,
        Color::LightMagenta,
        Color::LightCyan,
        Color::White,
    ];
    match color {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Indexed(i) if i < 16 => base[i as usize],
        Color::Indexed(i) if i >= 232 => {
            let v = 8 + 10 * (i - 232);
            (v, v, v)
        }
        Color::Indexed(i) => {
            let i = i - 16;
            (
                CUBE[(i / 36) as usize],
                CUBE[(i / 6 % 6) as usize],
                CUBE[(i % 6) as usize],
            )
        }
        other => named
            .iter()
            .position(|c| *c == other)
            .map_or((0, 0, 0), |i| base[i]),
    }
}

/// 16 colour mapping. Theme roles map by hand so greys stay apart; anything
/// else takes the nearest named colour.
fn ansi16(color: Color, theme: &Theme) -> Color {
    let light = theme.name == ThemeName::Light;
    let roles: [(Color, Color, Color); 12] = [
        (theme.selection, Color::Cyan, Color::Blue),
        (theme.text, Color::Black, Color::White),
        (theme.bg, Color::White, Color::Black),
        (theme.surface, Color::White, Color::Black),
        (theme.muted, Color::Black, Color::Gray),
        (theme.accent, Color::Blue, Color::LightBlue),
        (theme.border, Color::Gray, Color::DarkGray),
        (theme.graph_grid, Color::Gray, Color::DarkGray),
        (theme.graph_fill, Color::Green, Color::LightGreen),
        (theme.good, Color::Green, Color::LightGreen),
        (theme.warn, Color::Yellow, Color::LightYellow),
        (theme.bad, Color::Red, Color::LightRed),
    ];
    if let Some((_, l, d)) = roles.iter().find(|(c, _, _)| *c == color) {
        return if light { *l } else { *d };
    }
    let target = rgb(color);
    (0..16u8)
        .map(Color::Indexed)
        .min_by_key(|c| dist(indexed_rgb(*c), target))
        .unwrap_or(color)
}

fn rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => (0, 0, 0),
    }
}

fn channel(value: u8) -> f32 {
    let c = value as f32 / 255.0;
    if c <= 0.039_28 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// WCAG relative luminance.
fn luminance(color: Color) -> f32 {
    let (r, g, b) = indexed_rgb(color);
    0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b)
}

/// WCAG contrast ratio between two colours.
pub fn contrast(a: Color, b: Color) -> f32 {
    let (la, lb) = (luminance(a), luminance(b));
    let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_is_readable_in_both_themes() {
        for theme in [LIGHT, DARK] {
            assert!(
                contrast(theme.text, theme.bg) >= 4.5,
                "{:?} text on bg is {}",
                theme.name,
                contrast(theme.text, theme.bg)
            );
            assert!(
                contrast(theme.text, theme.surface) >= 4.5,
                "{:?} text on surface",
                theme.name
            );
            assert!(
                contrast(theme.muted, theme.bg) >= 4.5,
                "{:?} muted on bg is {}",
                theme.name,
                contrast(theme.muted, theme.bg)
            );
            assert!(
                contrast(theme.text, theme.selection) >= 4.5,
                "{:?} selection",
                theme.name
            );
            assert!(
                contrast(theme.accent, theme.bg) >= 3.0,
                "{:?} accent",
                theme.name
            );
        }
    }

    #[test]
    fn depth_detection() {
        use ColorDepth::*;
        assert_eq!(
            ColorDepth::detect(Some("truecolor"), Some("xterm")),
            TrueColor
        );
        assert_eq!(ColorDepth::detect(Some("24bit"), None), TrueColor);
        assert_eq!(ColorDepth::detect(None, Some("linux")), Ansi16);
        assert_eq!(ColorDepth::detect(None, Some("vt100")), Ansi16);
        assert_eq!(ColorDepth::detect(None, Some("dumb")), Ansi16);
        assert_eq!(ColorDepth::detect(None, Some("xterm-kitty")), Ansi256);
        assert_eq!(ColorDepth::detect(None, Some("xterm-256color")), Ansi256);
        assert_eq!(ColorDepth::detect(None, None), Ansi256);
        assert_eq!(ColorDepth::parse("256"), Some(Ansi256));
        assert_eq!(ColorDepth::parse("auto"), None);
    }

    #[test]
    fn xterm256_hits_exact_entries() {
        assert_eq!(xterm256(0, 0, 0), 16);
        assert_eq!(xterm256(255, 255, 255), 231);
        assert_eq!(xterm256(0x80, 0x80, 0x80), 244);
        assert_eq!(xterm256(0, 95, 215), 26);
        for i in 16..=255u8 {
            let (r, g, b) = indexed_rgb(Color::Indexed(i));
            assert_eq!(indexed_rgb(Color::Indexed(xterm256(r, g, b))), (r, g, b));
        }
    }

    #[test]
    fn no_rgb_survives_a_reduced_depth() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;
        for theme in [LIGHT, DARK] {
            for depth in [ColorDepth::Ansi256, ColorDepth::Ansi16] {
                let mut buf = Buffer::empty(Rect::new(0, 0, 4, 1));
                buf[(0, 0)].set_fg(theme.text).set_bg(theme.bg);
                buf[(1, 0)].set_fg(theme.muted).set_bg(theme.selection);
                buf[(2, 0)].set_fg(Color::Rgb(0x12, 0x34, 0x56));
                depth.apply(&mut buf, &theme);
                for cell in buf.content.iter() {
                    assert!(
                        !matches!(cell.fg, Color::Rgb(..)),
                        "{depth:?} {:?}",
                        cell.fg
                    );
                    assert!(
                        !matches!(cell.bg, Color::Rgb(..)),
                        "{depth:?} {:?}",
                        cell.bg
                    );
                }
            }
        }
    }

    #[test]
    fn text_is_readable_at_every_depth() {
        for theme in [LIGHT, DARK] {
            for depth in [
                ColorDepth::TrueColor,
                ColorDepth::Ansi256,
                ColorDepth::Ansi16,
            ] {
                let m = |c| depth.map(c, &theme);
                for (name, fg, bg) in [
                    ("text/bg", theme.text, theme.bg),
                    ("text/surface", theme.text, theme.surface),
                    ("muted/bg", theme.muted, theme.bg),
                    ("text/selection", theme.text, theme.selection),
                ] {
                    let ratio = contrast(m(fg), m(bg));
                    assert!(ratio >= 4.5, "{:?} {depth:?} {name} is {ratio}", theme.name);
                }
                assert_ne!(
                    m(theme.selection),
                    m(theme.bg),
                    "{:?} {depth:?}",
                    theme.name
                );
            }
        }
    }

    #[test]
    fn ascii_glyphs_are_all_ascii() {
        let glyphs = ASCII_GLYPHS;
        let all = [
            glyphs.hline,
            glyphs.vline,
            glyphs.tl,
            glyphs.tr,
            glyphs.bl,
            glyphs.br,
            glyphs.expanded,
            glyphs.collapsed,
            glyphs.sort_asc,
            glyphs.sort_desc,
            glyphs.checked,
            glyphs.unchecked,
            glyphs.block_full,
            glyphs.block_upper,
            glyphs.block_lower,
            glyphs.bullet,
        ];
        for glyph in all {
            assert!(glyph.is_ascii(), "{glyph:?} is not ascii");
        }
        assert!(!UNICODE_GLYPHS.expanded.is_ascii());
    }

    #[test]
    fn theme_names_toggle_and_parse() {
        assert_eq!(ThemeName::Light.toggle(), ThemeName::Dark);
        assert_eq!(ThemeName::parse("dark"), Some(ThemeName::Dark));
        assert_eq!(ThemeName::parse("nope"), None);
    }
}
