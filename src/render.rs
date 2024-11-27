use nu_ansi_term::{AnsiString, AnsiStrings, Style};

use crate::display::{Output, OutputCharType, OutputString, OutputStyles};

pub fn render(output: Output) {
    let styles = output.styles.unwrap_or_default();
    for line in output.output {
        render_line(line, &styles);
    }
}

fn render_line(line: OutputString, styles: &OutputStyles) {
    let grouped = line.get_grouped();
    let mut styled_strings: Vec<AnsiString> = Vec::new();
    for group in grouped {
        let style = get_style(group.1, &styles);
        styled_strings.push(style.paint(group.0))
    }

    styled_strings.iter().for_each(|s| print!("{}", s));
    print!("\n");
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
