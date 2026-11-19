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
    let (r, g, b) = rgb(color);
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
