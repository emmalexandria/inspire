use clap::{builder::PossibleValue, ValueEnum};
use nu_ansi_term::AnsiString;

use crate::{config::StyleConfig, display::Output};

#[derive(Clone, Copy)]
pub enum Wrappers {
    CatSign,
}

impl ValueEnum for Wrappers {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Wrappers::CatSign => Some(PossibleValue::new("catsign")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[Self::CatSign]
    }
}

impl Wrappers {
    pub fn wrap(&self, output: Output, config: &StyleConfig) -> Output {
        match self {
            Wrappers::CatSign => todo!(),
        }
    }
}

fn default_wrapper(output: Output, config: &StyleConfig) -> Output {
    return output;
}

fn border<'a>(quote_output: Vec<AnsiString<'a>>, config: &StyleConfig) -> Vec<AnsiString<'a>> {
    let output_width = quote_output
        .iter()
        .fold(0, |len, line| return len.max(line.as_str().len()));
    let corners = config.border.chars.corners;
    let vertical = config.border.chars.vertical;
    let horizontal = config.border.chars.horizontal;
    let first_line = String::from(
        corners[0].to_string()
            + &horizontal.to_string().repeat(output_width)
            + &corners[1].to_string(),
    );
    let last_line = String::from(
        corners[2].to_string()
            + &horizontal.to_string().repeat(output_width)
            + &corners[3].to_string(),
    );

    return quote_output;
}
