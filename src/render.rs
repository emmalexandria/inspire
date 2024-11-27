use nu_ansi_term::{AnsiString, AnsiStrings, Color, Style};

use crate::display::{Output, OutputCharType, OutputString, OutputStyles};

pub fn render(output: Output) {
    let styles = output.styles.unwrap_or_default();
    let filtered_styles = filter_styles(styles, output.color, output.attrs);
    for line in output.output {
        render_line(line, &filtered_styles);
    }
}

fn filter_styles(mut styles: OutputStyles, color: bool, attrs: bool) -> OutputStyles {
    if !color {
        remove_colors(&mut styles.author);
        styles.wrapper.iter_mut().for_each(remove_colors);
        remove_colors(&mut styles.padding);
        remove_colors(&mut styles.border);
        remove_colors(&mut styles.content);
    }
    if !attrs {
        remove_attributes(&mut styles.author);
        remove_attributes(&mut styles.padding);
        remove_attributes(&mut styles.border);
        remove_attributes(&mut styles.content);
        styles.wrapper.iter_mut().for_each(remove_attributes);
    }

    styles
}

fn remove_colors(style: &mut Style) {
    style.background = None;
    style.foreground = None;
}

fn remove_attributes(style: &mut Style) {
    style.is_blink = false;
    style.is_bold = false;
    style.is_italic = false;
    style.is_hidden = false;
    style.is_dimmed = false;
    style.is_underline = false;
    style.is_strikethrough = false;
    style.is_reverse = false;
}

fn render_line(line: OutputString, styles: &OutputStyles) {
    let grouped = line.get_grouped();
    let mut styled_strings: Vec<AnsiString> = Vec::new();
    for group in grouped {
        let style = get_style(group.1, &styles);
        styled_strings.push(style.paint(group.0))
    }

    styled_strings.iter().for_each(|s| print!("{}", s));
    print!("\n")
}

fn get_style(c_type: OutputCharType, styles: &OutputStyles) -> Style {
    match c_type {
        OutputCharType::Border => styles.border,
        OutputCharType::Padding => styles.padding,
        OutputCharType::Content => styles.content,
        OutputCharType::Author => styles.author,
        OutputCharType::Wrapper(i) => styles.wrapper[i],
    }
}
