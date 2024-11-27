use serde::{Deserialize, Serialize};
use std::{fs, io::Write, os};
use toml::to_string;

#[derive(Serialize, Deserialize)]
pub struct QuotesFile {
    quotes: Vec<Quote>,
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
