use serde::{Deserialize, Serialize};
use std::{fs, io::Write, os};
use toml::to_string;

pub const QUOTES_TOML: &str = include_str!("../quotes.toml");

#[derive(Serialize, Deserialize)]
pub struct QuotesFile {
    pub quotes: Vec<Quote>,
}

#[derive(Serialize, Deserialize)]
pub struct Quote {
    pub quote: String,
    pub author: String,
}

impl Quote {
    pub fn new(quote: String, author: String) -> Self {
        Self { quote, author }
    }
}
