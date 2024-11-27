use clap::{arg, builder::EnumValueParser, command, value_parser, Arg, ArgAction, Command};
use config::{Config, StyleConfig};
use display::Output;
use quotes::{get_quote, Quote, QuoteFile, QuotesFile};
use render::render;
use std::{fs::File, io::Write, path::PathBuf};
use wrappers::Wrappers;
mod config;
mod display;
mod quotes;
mod render;
mod wrappers;

fn main() {
    let matches = command!()
        .arg(
            Arg::new("fortune")
                .short('f')
                .long("fortune")
                .action(ArgAction::SetTrue)
                .help("Read installed fortunes from /usr/share/fortune"),
        )
        .arg(
            Arg::new("center")
                .short('c')
                .long("center")
                .action(ArgAction::SetTrue)
                .help("Center the output within the terminal"),
        )
        .arg(
            Arg::new("style")
                .short('s')
                .long("style")
                .help("Set the style to use from the config file")
                .required(false)
                .default_value("default")
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("wrapper")
                .short('w')
                .long("wrapper")
                .help("Set the output wrapper")
                .required(false)
                .value_parser(EnumValueParser::<Wrappers>::new()),
        )
        .arg(
            Arg::new("no-colors")
                .long("no-colors")
                .action(ArgAction::SetTrue)
                .help("Disable color in the output"),
        )
        .arg(
            Arg::new("no-attrs")
                .long("no-attrs")
                .action(ArgAction::SetTrue)
                .help("Disable attributes (e.g. bold) in the output"),
        )
        .get_matches();

    let no_attrs = matches.get_flag("no-attrs");
    let no_colors = matches.get_flag("no-colors");
    let center = matches.get_flag("center");
    let wrapper = matches.get_one::<Wrappers>("wrapper");

    let quote = quotes::fortune::FortuneFile::read(PathBuf::from("./art"))
        .unwrap()
        .get_quote()
        .unwrap();
    let mut output = Output::new(!no_colors, !no_attrs, center);
    output.make_output(&quote, &StyleConfig::default());

    if wrapper.is_some() {
        output = wrapper.unwrap().wrap(output, &StyleConfig::default())
    }

    render(output);
}
