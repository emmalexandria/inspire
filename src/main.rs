use clap::{arg, builder::EnumValueParser, command, value_parser, Arg, ArgAction, Command};
use display::Output;
use quote::Quote;
use render::render;
use style::StyleConfig;
use wrappers::Wrappers;
mod config;
mod display;
mod quote;
mod render;
mod style;
mod wrappers;

fn main() {
    let matches = command!()
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
                .help("Set the style to use from the configuration file")
                .required(false),
        )
        .arg(
            Arg::new("renderer")
                .short('r')
                .long("renderer")
                .help("Set the output renderer")
                .required(false)
                .value_parser(EnumValueParser::<Wrappers>::new()),
        )
        .arg(
            Arg::new("no-color")
                .long("no-color")
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

    let mut output = Output::new(true, true, false);
    output.make_output(&Quote::new("Please pay attention very carefully, because this is the truest thing a stranger will ever say to you: In the face of such hopelessness as our eventual, unavoidable death, there is little sense in not at least TRYING to accomplish all your wildest dreams in life.".to_string(), "Kevin Smith".to_string()), &StyleConfig::default());
    render(output);
}
