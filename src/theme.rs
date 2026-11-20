//! Windows 10 palette and glyph set.

use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeName {
    Light,
    Dark,
    Nord,
    Gruvbox,
    Phosphor,
    Amber,
    TokyoNight,
    Catppuccin,
    SolarizedLight,
    RosePine,
}

impl ThemeName {
    pub const ALL: [ThemeName; 10] = [
        ThemeName::Light,
        ThemeName::Dark,
        ThemeName::Nord,
        ThemeName::Gruvbox,
        ThemeName::Phosphor,
        ThemeName::Amber,
        ThemeName::TokyoNight,
        ThemeName::Catppuccin,
        ThemeName::SolarizedLight,
        ThemeName::RosePine,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ThemeName::Light => "Windows Light",
            ThemeName::Dark => "Windows Dark",
            ThemeName::Nord => "Nord",
            ThemeName::Gruvbox => "Gruvbox Dark",
            ThemeName::Phosphor => "Phosphor",
            ThemeName::Amber => "Amber",
            ThemeName::TokyoNight => "Tokyo Night",
            ThemeName::Catppuccin => "Catppuccin Mocha",
            ThemeName::SolarizedLight => "Solarized Light",
            ThemeName::RosePine => "Rosé Pine",
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            ThemeName::Light => "light",
            ThemeName::Dark => "dark",
            ThemeName::Nord => "nord",
            ThemeName::Gruvbox => "gruvbox",
            ThemeName::Phosphor => "phosphor",
            ThemeName::Amber => "amber",
            ThemeName::TokyoNight => "tokyo-night",
            ThemeName::Catppuccin => "catppuccin",
            ThemeName::SolarizedLight => "solarized-light",
            ThemeName::RosePine => "rose-pine",
        }
    }

    pub fn index(self) -> usize {
        Self::ALL.iter().position(|name| *name == self).unwrap_or(0)
    }

    pub fn next(self) -> Self {
        Self::ALL[(self.index() + 1) % Self::ALL.len()]
    }

    pub fn previous(self) -> Self {
        Self::ALL[(self.index() + Self::ALL.len() - 1) % Self::ALL.len()]
    }

    pub fn parse(value: &str) -> Option<Self> {
        let value = value.to_ascii_lowercase();
        Self::ALL
            .into_iter()
            .find(|name| name.key() == value || name.label().eq_ignore_ascii_case(&value))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub name: ThemeName,
    pub bg: Color,
    pub surface: Color,
    /// Alternating body-row background, a few percent from `surface`.
    pub surface_alt: Color,
    /// Background for the menu bar and tab strip.
    pub chrome: Color,
    /// Background for the status bar.
    pub status: Color,
    pub text: Color,
    pub muted: Color,
    pub border: Color,
    pub accent: Color,
    /// Text drawn on an accent-filled block.
    pub on_accent: Color,
    pub selection: Color,
    pub graph_fill: Color,
    pub graph_grid: Color,
    pub good: Color,
    pub warn: Color,
    pub bad: Color,
    /// Multiplier for the chart gradient's midpoint and top stops.
    pub gradient_strength: f32,
    /// Per-resource chart colours: CPU, Memory, Disk, Network, GPU.
    pub charts: [Color; 5],
    /// Processes cell shading, palest first.
    pub heat: [Color; 5],
    pub ascii: bool,
    /// 16 colour mode: graphs keep half cell steps, fills are not mixed.
    pub coarse: bool,
}

pub const LIGHT: Theme = Theme {
    name: ThemeName::Light,
    bg: Color::Rgb(0xF0, 0xF0, 0xF0),
    surface: Color::Rgb(0xFF, 0xFF, 0xFF),
    surface_alt: Color::Rgb(0xF7, 0xF7, 0xF7),
    chrome: Color::Rgb(0xE2, 0xE2, 0xE2),
    status: Color::Rgb(0xE8, 0xE8, 0xE8),
    text: Color::Rgb(0x00, 0x00, 0x00),
    muted: Color::Rgb(0x60, 0x5E, 0x5C),
    border: Color::Rgb(0xCC, 0xCC, 0xCC),
    accent: Color::Rgb(0x00, 0x70, 0xC0),
    on_accent: Color::Rgb(0xFF, 0xFF, 0xFF),
    selection: Color::Rgb(0xCC, 0xE8, 0xFF),
    graph_fill: Color::Rgb(0x1E, 0x8A, 0x73),
    graph_grid: Color::Rgb(0xE0, 0xE0, 0xE0),
    good: Color::Rgb(0x10, 0x7C, 0x10),
    warn: Color::Rgb(0x9D, 0x5A, 0x00),
    bad: Color::Rgb(0xC4, 0x2B, 0x1C),
    charts: [
        Color::Rgb(17, 125, 187),
        Color::Rgb(139, 18, 174),
        Color::Rgb(62, 135, 10),
        Color::Rgb(167, 79, 1),
        Color::Rgb(30, 138, 115),
    ],
    gradient_strength: 1.0,
    heat: [
        Color::Rgb(0xFF, 0xF4, 0xC4),
        Color::Rgb(0xFF, 0xE9, 0xA8),
        Color::Rgb(0xFD, 0xD8, 0x7F),
        Color::Rgb(0xF9, 0xB9, 0x6B),
        Color::Rgb(0xF1, 0x9A, 0x8F),
    ],
    ascii: false,
    coarse: false,
};

pub const DARK: Theme = Theme {
    name: ThemeName::Dark,
    bg: Color::Rgb(0x20, 0x20, 0x20),
    surface: Color::Rgb(0x27, 0x27, 0x27),
    surface_alt: Color::Rgb(0x2C, 0x2C, 0x2C),
    chrome: Color::Rgb(0x14, 0x14, 0x14),
    status: Color::Rgb(0x18, 0x18, 0x18),
    text: Color::Rgb(0xFF, 0xFF, 0xFF),
    muted: Color::Rgb(0xA0, 0xA0, 0xA0),
    border: Color::Rgb(0x3A, 0x3A, 0x3A),
    accent: Color::Rgb(0x00, 0x70, 0xC0),
    on_accent: Color::Rgb(0xFF, 0xFF, 0xFF),
    selection: Color::Rgb(0x0A, 0x3D, 0x62),
    graph_fill: Color::Rgb(0x4C, 0xC2, 0xA8),
    graph_grid: Color::Rgb(0x3A, 0x3A, 0x3A),
    good: Color::Rgb(0x6C, 0xC6, 0x6C),
    warn: Color::Rgb(0xE0, 0xA0, 0x40),
    bad: Color::Rgb(0xF0, 0x70, 0x60),
    charts: [
        Color::Rgb(76, 176, 240),
        Color::Rgb(199, 116, 232),
        Color::Rgb(126, 208, 80),
        Color::Rgb(240, 149, 74),
        Color::Rgb(76, 194, 168),
    ],
    gradient_strength: 1.0,
    heat: [
        Color::Rgb(0x3A, 0x33, 0x22),
        Color::Rgb(0x4A, 0x3D, 0x20),
        Color::Rgb(0x5A, 0x44, 0x1C),
        Color::Rgb(0x6B, 0x3A, 0x1E),
        Color::Rgb(0x74, 0x28, 0x1F),
    ],
    ascii: false,
    coarse: false,
};

pub const NORD: Theme = Theme {
    name: ThemeName::Nord,
    bg: Color::Rgb(0x2E, 0x34, 0x40),
    surface: Color::Rgb(0x3B, 0x42, 0x52),
    surface_alt: Color::Rgb(0x34, 0x3B, 0x47),
    chrome: Color::Rgb(0x24, 0x2A, 0x34),
    status: Color::Rgb(0x26, 0x2C, 0x36),
    text: Color::Rgb(0xEC, 0xEF, 0xF4),
    muted: Color::Rgb(0x9A, 0xA5, 0xB5),
    border: Color::Rgb(0x4C, 0x56, 0x6A),
    accent: Color::Rgb(0x88, 0xC0, 0xD0),
    on_accent: Color::Rgb(0x2E, 0x34, 0x40),
    selection: Color::Rgb(0x43, 0x4C, 0x5E),
    graph_fill: Color::Rgb(0x88, 0xC0, 0xD0),
    graph_grid: Color::Rgb(0x43, 0x4C, 0x5E),
    good: Color::Rgb(0xA3, 0xBE, 0x8C),
    warn: Color::Rgb(0xEB, 0xCB, 0x8B),
    bad: Color::Rgb(0xBF, 0x61, 0x6A),
    charts: [
        Color::Rgb(136, 192, 208),
        Color::Rgb(180, 142, 173),
        Color::Rgb(163, 190, 140),
        Color::Rgb(208, 135, 112),
        Color::Rgb(94, 129, 172),
    ],
    gradient_strength: 1.0,
    heat: [
        Color::Rgb(0x3A, 0x41, 0x4A),
        Color::Rgb(0x46, 0x44, 0x4A),
        Color::Rgb(0x54, 0x44, 0x4A),
        Color::Rgb(0x61, 0x40, 0x47),
        Color::Rgb(0x6E, 0x3B, 0x42),
    ],
    ascii: false,
    coarse: false,
};

pub const GRUVBOX: Theme = Theme {
    name: ThemeName::Gruvbox,
    bg: Color::Rgb(0x28, 0x28, 0x28),
    surface: Color::Rgb(0x32, 0x30, 0x2F),
    surface_alt: Color::Rgb(0x2C, 0x2A, 0x29),
    chrome: Color::Rgb(0x1C, 0x1C, 0x1C),
    status: Color::Rgb(0x20, 0x20, 0x20),
    text: Color::Rgb(0xEB, 0xDB, 0xB2),
    muted: Color::Rgb(0xA8, 0x99, 0x84),
    border: Color::Rgb(0x50, 0x49, 0x45),
    accent: Color::Rgb(0xFA, 0xBD, 0x2F),
    on_accent: Color::Rgb(0x28, 0x28, 0x28),
    selection: Color::Rgb(0x50, 0x49, 0x45),
    graph_fill: Color::Rgb(0x8E, 0xC0, 0x7C),
    graph_grid: Color::Rgb(0x3C, 0x38, 0x36),
    good: Color::Rgb(0xB8, 0xBB, 0x26),
    warn: Color::Rgb(0xFA, 0xBD, 0x2F),
    bad: Color::Rgb(0xFB, 0x49, 0x34),
    charts: [
        Color::Rgb(131, 165, 152),
        Color::Rgb(211, 134, 155),
        Color::Rgb(184, 187, 38),
        Color::Rgb(254, 128, 25),
        Color::Rgb(250, 189, 47),
    ],
    gradient_strength: 1.0,
    heat: [
        Color::Rgb(0x3A, 0x33, 0x24),
        Color::Rgb(0x4A, 0x3C, 0x22),
        Color::Rgb(0x5A, 0x43, 0x20),
        Color::Rgb(0x6A, 0x40, 0x26),
        Color::Rgb(0x78, 0x35, 0x2E),
    ],
    ascii: false,
    coarse: false,
};

pub const PHOSPHOR: Theme = Theme {
    name: ThemeName::Phosphor,
    bg: Color::Rgb(0x0A, 0x0F, 0x0A),
    surface: Color::Rgb(0x0F, 0x16, 0x0F),
    surface_alt: Color::Rgb(0x16, 0x22, 0x16),
    chrome: Color::Rgb(0x02, 0x05, 0x02),
    status: Color::Rgb(0x02, 0x0E, 0x02),
    text: Color::Rgb(0x7D, 0xF9, 0xA5),
    muted: Color::Rgb(0x4E, 0x9A, 0x6A),
    border: Color::Rgb(0x1E, 0x3A, 0x26),
    accent: Color::Rgb(0x33, 0xFF, 0x99),
    on_accent: Color::Rgb(0x04, 0x14, 0x0A),
    selection: Color::Rgb(0x14, 0x43, 0x2A),
    graph_fill: Color::Rgb(0x33, 0xFF, 0x99),
    graph_grid: Color::Rgb(0x16, 0x30, 0x1F),
    good: Color::Rgb(0x33, 0xFF, 0x99),
    warn: Color::Rgb(0xFF, 0xCE, 0x54),
    bad: Color::Rgb(0xFF, 0x6B, 0x6B),
    charts: [
        Color::Rgb(51, 255, 153),
        Color::Rgb(125, 249, 165),
        Color::Rgb(0, 204, 102),
        Color::Rgb(182, 255, 92),
        Color::Rgb(51, 204, 204),
    ],
    gradient_strength: 1.5,
    heat: [
        Color::Rgb(0x12, 0x30, 0x1C),
        Color::Rgb(0x1B, 0x3D, 0x22),
        Color::Rgb(0x4A, 0x3B, 0x1A),
        Color::Rgb(0x5C, 0x2E, 0x18),
        Color::Rgb(0x6B, 0x1F, 0x1F),
    ],
    ascii: false,
    coarse: false,
};

pub const AMBER: Theme = Theme {
    name: ThemeName::Amber,
    bg: Color::Rgb(0x0F, 0x0B, 0x06),
    surface: Color::Rgb(0x16, 0x10, 0x0A),
    surface_alt: Color::Rgb(0x22, 0x18, 0x0D),
    chrome: Color::Rgb(0x06, 0x04, 0x02),
    status: Color::Rgb(0x18, 0x10, 0x06),
    text: Color::Rgb(0xFF, 0xC4, 0x6B),
    muted: Color::Rgb(0xB0, 0x8A, 0x4A),
    border: Color::Rgb(0x3A, 0x2A, 0x14),
    accent: Color::Rgb(0xFF, 0xB0, 0x00),
    on_accent: Color::Rgb(0x14, 0x10, 0x0A),
    selection: Color::Rgb(0x3A, 0x2A, 0x10),
    graph_fill: Color::Rgb(0xFF, 0xB0, 0x00),
    graph_grid: Color::Rgb(0x2E, 0x22, 0x0F),
    good: Color::Rgb(0x9B, 0xD7, 0x6B),
    warn: Color::Rgb(0xFF, 0xB0, 0x00),
    bad: Color::Rgb(0xFF, 0x6B, 0x5A),
    charts: [
        Color::Rgb(255, 176, 0),
        Color::Rgb(255, 140, 0),
        Color::Rgb(255, 210, 74),
        Color::Rgb(224, 108, 43),
        Color::Rgb(255, 107, 90),
    ],
    gradient_strength: 1.5,
    heat: [
        Color::Rgb(0x2A, 0x22, 0x13),
        Color::Rgb(0x3A, 0x2C, 0x12),
        Color::Rgb(0x4A, 0x35, 0x10),
        Color::Rgb(0x5A, 0x3A, 0x14),
        Color::Rgb(0x6B, 0x2E, 0x1A),
    ],
    ascii: false,
    coarse: false,
};

pub const TOKYO_NIGHT: Theme = Theme {
    name: ThemeName::TokyoNight,
    bg: Color::Rgb(0x1A, 0x1B, 0x26),
    surface: Color::Rgb(0x1F, 0x23, 0x35),
    surface_alt: Color::Rgb(0x25, 0x28, 0x3A),
    chrome: Color::Rgb(0x10, 0x11, 0x1A),
    status: Color::Rgb(0x14, 0x15, 0x1E),
    text: Color::Rgb(0xC0, 0xCA, 0xF5),
    muted: Color::Rgb(0x7A, 0xA2, 0xF7),
    border: Color::Rgb(0x29, 0x2E, 0x42),
    accent: Color::Rgb(0x7A, 0xA2, 0xF7),
    on_accent: Color::Rgb(0x1A, 0x1B, 0x26),
    selection: Color::Rgb(0x2A, 0x35, 0x60),
    graph_fill: Color::Rgb(0x7D, 0xCF, 0xFF),
    graph_grid: Color::Rgb(0x29, 0x2E, 0x42),
    good: Color::Rgb(0x9E, 0xCE, 0x6A),
    warn: Color::Rgb(0xE0, 0xAF, 0x68),
    bad: Color::Rgb(0xF7, 0x76, 0x8E),
    charts: [
        Color::Rgb(0x7A, 0xA2, 0xF7),
        Color::Rgb(0xBB, 0x9A, 0xF7),
        Color::Rgb(0x9E, 0xCE, 0x6A),
        Color::Rgb(0xE0, 0xAF, 0x68),
        Color::Rgb(0x7D, 0xCF, 0xFF),
    ],
    gradient_strength: 1.0,
    heat: [
        Color::Rgb(0x2A, 0x2E, 0x42),
        Color::Rgb(0x34, 0x31, 0x4A),
        Color::Rgb(0x45, 0x33, 0x4D),
        Color::Rgb(0x55, 0x30, 0x49),
        Color::Rgb(0x63, 0x2C, 0x46),
    ],
    ascii: false,
    coarse: false,
};

pub const CATPPUCCIN: Theme = Theme {
    name: ThemeName::Catppuccin,
    bg: Color::Rgb(0x1E, 0x1E, 0x2E),
    surface: Color::Rgb(0x31, 0x32, 0x44),
    surface_alt: Color::Rgb(0x2A, 0x2A, 0x3C),
    chrome: Color::Rgb(0x12, 0x12, 0x1D),
    status: Color::Rgb(0x18, 0x18, 0x26),
    text: Color::Rgb(0xCD, 0xD6, 0xF4),
    muted: Color::Rgb(0xA6, 0xAD, 0xC8),
    border: Color::Rgb(0x45, 0x47, 0x5A),
    accent: Color::Rgb(0x89, 0xB4, 0xFA),
    on_accent: Color::Rgb(0x1E, 0x1E, 0x2E),
    selection: Color::Rgb(0x45, 0x47, 0x5A),
    graph_fill: Color::Rgb(0x89, 0xB4, 0xFA),
    graph_grid: Color::Rgb(0x31, 0x32, 0x44),
    good: Color::Rgb(0xA6, 0xE3, 0xA1),
    warn: Color::Rgb(0xF9, 0xE2, 0xAF),
    bad: Color::Rgb(0xF3, 0x8B, 0xA8),
    charts: [
        Color::Rgb(0x89, 0xB4, 0xFA),
        Color::Rgb(0xCB, 0xA6, 0xF7),
        Color::Rgb(0xA6, 0xE3, 0xA1),
        Color::Rgb(0xF9, 0xE2, 0xAF),
        Color::Rgb(0x94, 0xE2, 0xD5),
    ],
    gradient_strength: 1.0,
    heat: [
        Color::Rgb(0x3A, 0x35, 0x48),
        Color::Rgb(0x45, 0x38, 0x4E),
        Color::Rgb(0x51, 0x3B, 0x4E),
        Color::Rgb(0x5C, 0x3C, 0x4C),
        Color::Rgb(0x68, 0x3B, 0x49),
    ],
    ascii: false,
    coarse: false,
};

pub const SOLARIZED: Theme = Theme {
    name: ThemeName::SolarizedLight,
    bg: Color::Rgb(0xFD, 0xF6, 0xE3),
    surface: Color::Rgb(0xFF, 0xFB, 0xF0),
    surface_alt: Color::Rgb(0xF7, 0xF1, 0xDE),
    chrome: Color::Rgb(0xE4, 0xDE, 0xCB),
    status: Color::Rgb(0xF2, 0xEC, 0xD9),
    text: Color::Rgb(0x07, 0x36, 0x42),
    muted: Color::Rgb(0x58, 0x6E, 0x75),
    border: Color::Rgb(0xD5, 0xCD, 0xB6),
    accent: Color::Rgb(0x1C, 0x6F, 0xA8),
    on_accent: Color::Rgb(0xFD, 0xF6, 0xE3),
    selection: Color::Rgb(0xCF, 0xE3, 0xEA),
    graph_fill: Color::Rgb(0x2A, 0xA1, 0x98),
    graph_grid: Color::Rgb(0xE4, 0xDD, 0xC8),
    good: Color::Rgb(0x56, 0x72, 0x00),
    warn: Color::Rgb(0x8A, 0x62, 0x00),
    bad: Color::Rgb(0xDC, 0x32, 0x2F),
    charts: [
        Color::Rgb(0x26, 0x8B, 0xD2),
        Color::Rgb(0x6C, 0x71, 0xC4),
        Color::Rgb(0x85, 0x99, 0x00),
        Color::Rgb(0xCB, 0x4B, 0x16),
        Color::Rgb(0x2A, 0xA1, 0x98),
    ],
    gradient_strength: 1.0,
    heat: [
        Color::Rgb(0xFF, 0xF0, 0xC4),
        Color::Rgb(0xFF, 0xE3, 0xA6),
        Color::Rgb(0xF9, 0xCE, 0x86),
        Color::Rgb(0xF2, 0xB2, 0x71),
        Color::Rgb(0xE8, 0x94, 0x78),
    ],
    ascii: false,
    coarse: false,
};

pub const ROSE_PINE: Theme = Theme {
    name: ThemeName::RosePine,
    bg: Color::Rgb(0x19, 0x17, 0x24),
    surface: Color::Rgb(0x1F, 0x1D, 0x2E),
    surface_alt: Color::Rgb(0x23, 0x21, 0x36),
    chrome: Color::Rgb(0x10, 0x0E, 0x19),
    status: Color::Rgb(0x13, 0x11, 0x1C),
    text: Color::Rgb(0xE0, 0xDE, 0xF4),
    muted: Color::Rgb(0x90, 0x8C, 0xAA),
    border: Color::Rgb(0x40, 0x3D, 0x52),
    accent: Color::Rgb(0xC4, 0xA7, 0xE7),
    on_accent: Color::Rgb(0x19, 0x17, 0x24),
    selection: Color::Rgb(0x40, 0x3D, 0x52),
    graph_fill: Color::Rgb(0x9C, 0xCF, 0xD8),
    graph_grid: Color::Rgb(0x26, 0x23, 0x3A),
    good: Color::Rgb(0x9C, 0xCF, 0xD8),
    warn: Color::Rgb(0xF6, 0xC1, 0x77),
    bad: Color::Rgb(0xEB, 0x6F, 0x92),
    charts: [
        Color::Rgb(0xC4, 0xA7, 0xE7),
        Color::Rgb(0xEB, 0xBC, 0xBA),
        Color::Rgb(0x9C, 0xCF, 0xD8),
        Color::Rgb(0xF6, 0xC1, 0x77),
        Color::Rgb(0xEB, 0x6F, 0x92),
    ],
    gradient_strength: 1.0,
    heat: [
        Color::Rgb(0x2A, 0x24, 0x40),
        Color::Rgb(0x35, 0x2A, 0x45),
        Color::Rgb(0x43, 0x2F, 0x47),
        Color::Rgb(0x53, 0x33, 0x45),
        Color::Rgb(0x61, 0x35, 0x43),
    ],
    ascii: false,
    coarse: false,
};

/// Percent of the system total at which each heat step starts.
pub const HEAT_STEPS: [f32; 5] = [1.0, 3.0, 8.0, 20.0, 40.0];

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
    /// Bottom-anchored blocks, one to eight eighths high.
    pub eighths: [char; 8],
    /// Optional angular fill glyphs: `▟`, `▙`, `◢`, `◣`. None in ASCII.
    pub block_rise: Option<char>,
    pub block_fall: Option<char>,
    pub slope_rise: Option<char>,
    pub slope_fall: Option<char>,
    /// Filter field: clear mark and text cursor.
    pub clear: char,
    pub cursor: char,
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
    eighths: ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'],
    block_rise: Some('▟'),
    block_fall: Some('▙'),
    slope_rise: Some('◢'),
    slope_fall: Some('◣'),
    clear: '×',
    cursor: '▏',
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
    eighths: ['_', '_', '_', '_', '#', '#', '#', '#'],
    block_rise: None,
    block_fall: None,
    slope_rise: None,
    slope_fall: None,
    clear: 'x',
    cursor: '|',
};

impl Theme {
    pub fn from_name(name: ThemeName, ascii: bool) -> Self {
        let mut theme = match name {
            ThemeName::Light => LIGHT,
            ThemeName::Dark => DARK,
            ThemeName::Nord => NORD,
            ThemeName::Gruvbox => GRUVBOX,
            ThemeName::Phosphor => PHOSPHOR,
            ThemeName::Amber => AMBER,
            ThemeName::TokyoNight => TOKYO_NIGHT,
            ThemeName::Catppuccin => CATPPUCCIN,
            ThemeName::SolarizedLight => SOLARIZED,
            ThemeName::RosePine => ROSE_PINE,
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

    /// Background for a body row: selection wins, otherwise alternate.
    pub fn row_style(&self, selected: bool, index: usize) -> Style {
        if selected {
            return self.selected_style();
        }
        let bg = if index.is_multiple_of(2) {
            self.surface
        } else {
            self.surface_alt
        };
        Style::default().fg(self.text).bg(bg)
    }

    pub fn tone_color(&self, tone: crate::model::Tone) -> Color {
        match tone {
            crate::model::Tone::Normal => self.text,
            crate::model::Tone::Good => self.good,
            crate::model::Tone::Muted => self.muted,
            crate::model::Tone::Warn => self.warn,
            crate::model::Tone::Danger => self.bad,
        }
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

    /// Shade for a cell holding `share` percent of the system total, or `None`
    /// below the first step. 16 colour mode keeps a single step at 15 percent.
    pub fn heat_for(&self, share: f32) -> Option<Color> {
        if self.coarse {
            return (share >= 20.0).then_some(self.heat[4]);
        }
        HEAT_STEPS
            .iter()
            .rposition(|step| share >= *step)
            .map(|index| self.heat[index])
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

    pub fn label(self) -> &'static str {
        match self {
            ColorDepth::TrueColor => "Truecolor",
            ColorDepth::Ansi256 => "256 colours",
            ColorDepth::Ansi16 => "16 colours",
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

/// The 16 named ANSI colours, in palette order. crossterm still writes them as
/// `38;5;n`, which the Linux console has parsed since kernel 3.16.
const NAMED16: [Color; 16] = [
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
    let named = NAMED16;
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
    let light = !is_dark(theme.bg);
    let roles: [(Color, Color, Color); 14] = [
        (theme.heat[4], Color::Yellow, Color::Red),
        (theme.surface_alt, Color::White, Color::Black),
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
    NAMED16
        .into_iter()
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

/// Convert a linear-light channel back to sRGB.
fn encode(linear: f32) -> u8 {
    let l = linear.clamp(0.0, 1.0);
    let c = if l <= 0.003_130_8 {
        l * 12.92
    } else {
        1.055 * l.powf(1.0 / 2.4) - 0.055
    };
    (c * 255.0).round().clamp(0.0, 255.0) as u8
}

/// Blend in linear light. A half-way mix of black and white is mid-grey by
/// luminance rather than the darker sRGB average. Non-RGB colours return `a`.
pub fn mix_linear(a: Color, b: Color, toward_b: f32) -> Color {
    match (a, b) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let t = toward_b.clamp(0.0, 1.0);
            let blend = |x: u8, y: u8| {
                let xl = channel(x);
                let yl = channel(y);
                encode(xl + (yl - xl) * t)
            };
            Color::Rgb(blend(r1, r2), blend(g1, g2), blend(b1, b2))
        }
        _ => a,
    }
}

/// True when a background is closer to black than to white.
pub fn is_dark(color: Color) -> bool {
    luminance(color) < 0.5
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

    fn all_themes() -> Vec<Theme> {
        ThemeName::ALL
            .into_iter()
            .map(|name| Theme::from_name(name, false))
            .collect()
    }

    #[test]
    fn linear_mix_keeps_the_midpoint_light() {
        let mid = mix_linear(Color::Rgb(0, 0, 0), Color::Rgb(255, 255, 255), 0.5);
        let Color::Rgb(r, _, _) = mid else {
            panic!("expected rgb");
        };
        assert!(r > 160, "linear mid is lighter than the sRGB average: {r}");
        assert_eq!(mix_linear(Color::Red, Color::Blue, 0.5), Color::Red);
    }

    #[test]
    fn text_is_readable_in_both_themes() {
        for theme in all_themes() {
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
                contrast(theme.text, theme.surface_alt) >= 4.5,
                "{:?} text on surface_alt",
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
            assert!(
                contrast(theme.text, theme.chrome) >= 4.5,
                "{:?} text on chrome",
                theme.name
            );
            assert!(
                contrast(theme.text, theme.status) >= 4.5,
                "{:?} text on status",
                theme.name
            );
            assert!(
                contrast(theme.on_accent, theme.accent) >= 4.5,
                "{:?} on_accent is {}",
                theme.name,
                contrast(theme.on_accent, theme.accent)
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
    fn heat_steps_follow_the_thresholds() {
        let mut theme = LIGHT;
        assert_eq!(theme.heat_for(0.9), None);
        assert_eq!(theme.heat_for(1.0), Some(theme.heat[0]));
        assert_eq!(theme.heat_for(2.9), Some(theme.heat[0]));
        assert_eq!(theme.heat_for(3.0), Some(theme.heat[1]));
        assert_eq!(theme.heat_for(7.9), Some(theme.heat[1]));
        assert_eq!(theme.heat_for(8.0), Some(theme.heat[2]));
        assert_eq!(theme.heat_for(20.0), Some(theme.heat[3]));
        assert_eq!(theme.heat_for(40.0), Some(theme.heat[4]));
        assert_eq!(theme.heat_for(100.0), Some(theme.heat[4]));
        theme.coarse = true;
        assert_eq!(theme.heat_for(19.9), None);
        assert_eq!(theme.heat_for(20.0), Some(theme.heat[4]));
    }

    #[test]
    fn every_theme_has_five_distinct_visible_chart_colours() {
        for theme in all_themes() {
            for i in 0..5 {
                for j in i + 1..5 {
                    assert_ne!(
                        theme.charts[i], theme.charts[j],
                        "{:?} repeats a chart colour",
                        theme.name
                    );
                }
                assert!(
                    contrast(theme.charts[i], theme.surface) >= 2.0,
                    "{:?} chart {i} is {}",
                    theme.name,
                    contrast(theme.charts[i], theme.surface)
                );
            }
        }
    }

    #[test]
    fn stripes_are_visible_in_every_theme() {
        for theme in all_themes() {
            let (a, b) = (indexed_rgb(theme.surface), indexed_rgb(theme.surface_alt));
            let step = [(a.0, b.0), (a.1, b.1), (a.2, b.2)]
                .into_iter()
                .map(|(x, y)| (x as i32 - y as i32).abs())
                .max()
                .unwrap();
            assert!(step >= 5, "{:?} stripe step is {step}", theme.name);
        }
    }

    #[test]
    fn bands_are_visible_against_the_background() {
        for theme in all_themes() {
            for (name, band) in [("chrome", theme.chrome), ("status", theme.status)] {
                let (bg, band) = (indexed_rgb(theme.bg), indexed_rgb(band));
                let step = [(bg.0, band.0), (bg.1, band.1), (bg.2, band.2)]
                    .into_iter()
                    .map(|(a, b)| (a as i32 - b as i32).abs())
                    .max()
                    .unwrap();
                assert!(
                    step >= 6,
                    "{:?} {name} is {step} channel steps from bg",
                    theme.name
                );
            }
            assert_ne!(theme.chrome, theme.status, "{:?}", theme.name);
        }
    }

    #[test]
    fn stripes_collapse_at_sixteen_colours() {
        for theme in all_themes() {
            assert_eq!(
                ColorDepth::Ansi16.map(theme.surface_alt, &theme),
                ColorDepth::Ansi16.map(theme.surface, &theme),
                "{:?}",
                theme.name
            );
        }
    }

    #[test]
    fn rows_alternate_and_selection_wins() {
        for theme in all_themes() {
            assert_eq!(theme.row_style(false, 0).bg, Some(theme.surface));
            assert_eq!(theme.row_style(false, 1).bg, Some(theme.surface_alt));
            assert_eq!(theme.row_style(false, 2).bg, Some(theme.surface));
            assert_eq!(theme.row_style(true, 1).bg, Some(theme.selection));
            assert_ne!(theme.surface, theme.surface_alt);
        }
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
        for theme in all_themes() {
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
        for theme in all_themes() {
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
                    ("text/surface_alt", theme.text, theme.surface_alt),
                    ("text/chrome", theme.text, theme.chrome),
                    ("text/status", theme.text, theme.status),
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
                // 16 colour mode only ever draws the deepest step.
                let first = if depth == ColorDepth::Ansi16 { 4 } else { 0 };
                for (step, heat) in theme.heat.iter().enumerate().skip(first) {
                    let ratio = contrast(m(theme.text), m(*heat));
                    assert!(
                        ratio >= 4.5,
                        "{:?} {depth:?} heat {step} is {ratio}",
                        theme.name
                    );
                }
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
            glyphs.clear,
            glyphs.cursor,
        ];
        for glyph in all.iter().chain(glyphs.eighths.iter()) {
            assert!(glyph.is_ascii(), "{glyph:?} is not ascii");
        }
        assert!(
            !glyphs.eighths.contains(&' '),
            "an ascii graph step is blank"
        );
        assert!(
            glyphs.block_rise.is_none()
                && glyphs.block_fall.is_none()
                && glyphs.slope_rise.is_none()
                && glyphs.slope_fall.is_none(),
            "no angular glyphs in ascii mode"
        );
        assert!(!UNICODE_GLYPHS.expanded.is_ascii());
    }

    #[test]
    fn theme_names_toggle_and_parse() {
        assert_eq!(ThemeName::Light.next(), ThemeName::Dark);
        assert_eq!(ThemeName::Light.previous(), ThemeName::RosePine);
        assert_eq!(
            ThemeName::ALL
                .iter()
                .filter(|n| ThemeName::parse(n.key()) == Some(**n))
                .count(),
            10
        );
        assert_eq!(ThemeName::parse("dark"), Some(ThemeName::Dark));
        assert_eq!(ThemeName::parse("nope"), None);
    }
}
