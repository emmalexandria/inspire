use nu_ansi_term::{Color, Style};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

impl Into<pad::Alignment> for Alignment {
    fn into(self) -> pad::Alignment {
        match self {
            Self::Left => pad::Alignment::Left,
            Self::Center => pad::Alignment::Middle,
            Self::Right => pad::Alignment::Right,
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
    pub border_style: BorderStyle,
    pub indent: Spacing,
    pub padding: (Spacing, Spacing),
    pub content_width: Spacing,
    pub content_alignment: Alignment,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            body: Style::new().fg(Color::LightRed).bold(),
            author: AuthorStyle::default(),
            border_style: BorderStyle::default(),
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
