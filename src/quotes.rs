pub mod fortune;
pub mod inspire;

use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    fs::{self, DirEntry, File},
    io::{BufRead, BufReader, Read, Write},
    os::{self},
    path::PathBuf,
};

pub trait QuoteFile {
    fn read(path: PathBuf) -> std::io::Result<Self>
    where
        Self: Sized;
    fn get_quote(self) -> Option<Quote>;
}

use rand::seq::{IteratorRandom, SliceRandom};
use toml::to_string;

pub fn get_quote() -> Quote {
    let quote_files = fs::read_dir("/usr/share/inspire/quotes")
        .unwrap()
        .filter_map(|res| res.ok())
        .map(|dir_entry| dir_entry.path())
        .filter_map(|path| {
            if path.extension().map_or(false, |ext| ext == "toml") {
                return Some(path);
            }
            None
        });

    let file = quote_files.choose(&mut rand::thread_rng()).unwrap();
    let mut quotes_file_content: String = String::new();
    let quotes_file = File::open(file)
        .unwrap()
        .read_to_string(&mut quotes_file_content);
    let quotes: QuotesFile = toml::from_str(&quotes_file_content).unwrap();
    return quotes
        .quotes
        .choose(&mut rand::thread_rng())
        .unwrap()
        .clone();
}

#[derive(Serialize, Deserialize)]
pub struct QuotesFile {
    pub quotes: Vec<Quote>,
}

impl QuotesFile {
    pub fn split(&self, n: u32) -> Vec<Self> {
        let quotes = self.quotes.clone();
        let dst: Vec<&[Quote]> = quotes
            .chunks((quotes.len() as f64 / n as f64) as usize)
            .collect();

        let files = dst
            .iter()
            .map(|q| QuotesFile { quotes: q.to_vec() })
            .collect();
        return files;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Quote {
    pub quote: String,
    pub author: String,
}

impl Quote {
    pub fn new(quote: String, author: String) -> Self {
        Self { quote, author }
    }
}
