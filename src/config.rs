use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    styles: Vec<StyleConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            styles: vec![StyleConfig::default()],
        }
    }
}

use nu_ansi_term::{Color, Style};

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    MiddleRight,
}

impl From<pad::Alignment> for Alignment {
    fn from(value: pad::Alignment) -> Self {
        match value {
            pad::Alignment::Left => Alignment::Left,
            pad::Alignment::Right => Alignment::Right,
            pad::Alignment::Middle => Alignment::Center,
            pad::Alignment::MiddleRight => Alignment::MiddleRight,
        }
    }
}

impl Into<pad::Alignment> for Alignment {
    fn into(self) -> pad::Alignment {
        match self {
            Alignment::Left => pad::Alignment::Left,
            Alignment::Center => pad::Alignment::Middle,
            Alignment::Right => pad::Alignment::Right,
            Alignment::MiddleRight => pad::Alignment::MiddleRight,
        }
    }
}

pub type Spacing = usize;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct StyleConfig {
    pub name: String,
    pub body: Style,
    pub author: AuthorStyle,
    pub border: BorderStyle,
    pub indent: Spacing,
    pub padding: (Spacing, Spacing),
    pub content_width: Spacing,
    pub content_alignment: Alignment,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            name: String::from("default"),
            body: Style::new().fg(Color::LightRed).bold(),
            author: AuthorStyle::default(),
            border: BorderStyle::default(),
            indent: 0,
            padding: (4, 2),
            content_width: 80,
            content_alignment: Alignment::Left,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BorderStyle {
    pub chars: BorderChars,
    pub style: Style,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(default)]
pub struct BorderChars {
    pub vertical: char,
    pub horizontal: char,
    pub corners: [char; 4],
}

impl Default for BorderChars {
    fn default() -> Self {
        Self {
            horizontal: '─',
            vertical: '│',
            corners: ['┌', '┐', '└', '┘'],
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct AuthorStyle {
    pub style: Style,
    pub gap: Spacing,
    pub indent: Spacing,
    pub prefix: String,
}

impl Default for AuthorStyle {
    fn default() -> Self {
        Self {
            style: Style::new().italic().dimmed(),
            gap: 0,
            indent: 4,
            prefix: String::from("—"),
        }
    }
}
