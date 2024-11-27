use std::borrow::BorrowMut;
use std::char;
use std::fmt::Display;

use crate::quote::Quote;
use crate::style::{Spacing, StyleConfig};
use nu_ansi_term::{AnsiString, AnsiStrings, Style};
use pad::{Alignment, PadStr};
use textwrap::{wrap, Options};

#[derive(Clone, Copy, PartialEq)]
pub enum OutputCharType {
    Border,
    Padding,
    Content,
    Author,
    Wrapper(usize),
}

#[derive(PartialEq, Clone)]
pub struct OutputString {
    bytes: Vec<(char, OutputCharType)>,
}

impl OutputString {
    fn from_str<S: AsRef<str>>(s: S, char_type: OutputCharType) -> Self {
        let chars: Vec<char> = s.as_ref().chars().collect();
        let output_bytes = chars.iter().map(|c| return (*c, char_type)).collect();
        Self {
            bytes: output_bytes,
        }
    }

    pub fn get_grouped(&self) -> Vec<(String, OutputCharType)> {
        let mut result: Vec<(String, OutputCharType)> = Vec::new();
        let mut current_group = String::new();
        let mut current_type = None;

        for (char, c_type) in &self.bytes {
            if Some(c_type) == current_type {
                current_group.push(char::from(*char))
            } else {
                if let Some(e) = current_type.take() {
                    result.push((current_group, *e));
                }

                current_group = char::from(*char).to_string();
                current_type = Some(c_type)
            }
        }

        if let Some(e) = current_type {
            result.push((current_group, *e))
        }

        return result;
    }
}

#[derive(Default, Clone)]
pub struct OutputStyles {
    pub border: Style,
    pub padding: Style,
    pub content: Style,
    pub author: Style,
    pub wrapper: Vec<Style>,
}

#[derive(Default)]
pub struct Output {
    pub color: bool,
    pub attrs: bool,
    pub center: bool,
    content_width: Spacing,
    pub output: Vec<OutputString>,
    pub styles: Option<OutputStyles>,
}

impl Output {
    pub fn new(color: bool, attrs: bool, center: bool) -> Self {
        Self {
            color,
            attrs,
            center,
            content_width: 0,
            output: Vec::new(),
            styles: None,
        }
    }

    pub fn make_output(&mut self, quote: &Quote, style: &StyleConfig) {
        self.styles = Some(OutputStyles {
            author: style.author.style,
            content: style.body,
            border: style.border_style.style,
            padding: Style::default(),
            wrapper: Vec::new(),
        });
        self.layout_quote(quote, style);
        self.apply_padding(style);
        self.apply_border(style);
    }

    fn apply_border(&mut self, style: &StyleConfig) {
        let chars = style.border_style.chars;
        self.output.iter_mut().for_each(|l| {
            l.bytes.insert(0, (chars.vertical, OutputCharType::Border));
            l.bytes.push((chars.vertical, OutputCharType::Border))
        });
        let horizontal_middle = chars
            .horizontal
            .to_string()
            .repeat(self.content_width + (style.padding.0 * 2));
        self.output.insert(
            0,
            OutputString::from_str(
                chars.corners[0].to_string() + &horizontal_middle + &chars.corners[1].to_string(),
                OutputCharType::Border,
            ),
        );
        self.output.push(OutputString::from_str(
            chars.corners[2].to_string() + &horizontal_middle + &chars.corners[3].to_string(),
            OutputCharType::Border,
        ));
    }

    fn apply_padding(&mut self, style: &StyleConfig) {
        for _ in 0..style.padding.1 {
            let padding_str =
                OutputString::from_str(" ".repeat(self.content_width), OutputCharType::Padding);
            self.output.insert(0, padding_str.clone());
            self.output.push(padding_str)
        }
        self.output.iter_mut().for_each(|l| {
            for _ in 0..style.padding.0 {
                l.bytes.insert(0, (' ', OutputCharType::Padding));
                l.bytes.push((' ', OutputCharType::Padding));
            }
        });
    }

    fn layout_quote(&mut self, quote: &Quote, style: &StyleConfig) {
        let (wrapped_content, content_width) = Self::layout_content(&quote.quote, style);
        self.content_width = content_width;
        let wrapped_author = Self::layout_author(&quote.author, style, content_width);

        wrapped_content.iter().for_each(|l| {
            self.output
                .push(OutputString::from_str(l, OutputCharType::Content))
        });
        for _ in 0..style.author.gap {
            self.output.push(OutputString::from_str(
                " ".repeat(content_width),
                OutputCharType::Padding,
            ))
        }
        wrapped_author.iter().for_each(|l| {
            self.output
                .push(OutputString::from_str(l, OutputCharType::Author))
        });
    }

    fn layout_content(content: &String, style: &StyleConfig) -> (Vec<String>, Spacing) {
        let wrapped_lines = wrap_text_to_width(content, style.content_width);
        let max_width = wrapped_lines.iter().fold(0, |length, line| {
            if line.len() > length {
                return line.len();
            }
            return length;
        });

        let padded_lines = wrapped_lines
            .iter()
            .map(|l| l.pad_to_width_with_alignment(max_width, style.content_alignment.into()))
            .collect();
        return (padded_lines, max_width);
    }

    fn layout_author(author: &String, style: &StyleConfig, content_len: Spacing) -> Vec<String> {
        let author_with_prefix = style.author.prefix.clone() + &author;
        let author_lines = wrap_text_to_width(author_with_prefix, content_len);
        if style.content_alignment == crate::style::Alignment::Center {
            let padded_lines: Vec<String> = author_lines
                .iter()
                .map(|l| l.pad_to_width_with_alignment(content_len, style.content_alignment.into()))
                .collect();
            return padded_lines;
        }

        author_lines
            .iter()
            .map(|l| {
                let indent_line = " ".repeat(style.author.indent) + l;
                indent_line.pad_to_width(content_len)
            })
            .collect()
    }
}

fn wrap_text_to_width<S: ToString>(content: S, width: Spacing) -> Vec<String> {
    if width == 0 {
        return vec![content.to_string()];
    }
    return wrap(&content.to_string(), Options::new(width as usize))
        .iter()
        .map(|s| s.to_string())
        .collect();
}
