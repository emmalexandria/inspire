use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Seek},
    path::PathBuf,
};

use super::{Quote, QuoteFile};
use rand::seq::SliceRandom;

pub struct FortuneFile {
    delimiter: char,
    quotes: Vec<Quote>,
}

impl QuoteFile for FortuneFile {
    fn read(path: std::path::PathBuf) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let delimiter = Self::identify_delimiter(path.clone())?;

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines();

        let mut chunk: Vec<String> = vec![];
        let mut quotes: Vec<Quote> = vec![];

        for line in lines {
            if line.is_err() {
                continue;
            }
            let line_text = line.unwrap();
            if line_text.trim() == delimiter.to_string() {
                quotes.push(Self::process_chunk(&chunk));
                chunk.clear();
            } else {
                chunk.push(line_text)
            }
        }

        Ok(Self { delimiter, quotes })
    }

    fn get_quote(self) -> Option<Quote> {
        return self.quotes.choose(&mut rand::thread_rng()).cloned();
    }
}

impl FortuneFile {
    fn identify_delimiter(path: PathBuf) -> std::io::Result<char> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut chars: Vec<char> = reader
            .lines()
            .into_iter()
            .filter_map(|l| l.ok())
            .filter_map(|l| {
                let trimmed = l.trim();
                if trimmed.len() == 1 {
                    return Some(trimmed.to_owned());
                }
                return None;
            })
            .filter_map(|l| l.chars().next())
            .collect();
        chars.sort();
        let mut unique_chars = chars.clone();
        unique_chars.dedup();
        if unique_chars.len() == 1 {
            return Ok(unique_chars[0]);
        }

        return Ok('%');
    }

    fn process_chunk(chunk: &Vec<String>) -> Quote {
        let mut content = Vec::new();
        let mut author: Vec<String> = Vec::new();
        let mut unsure: Vec<String> = Vec::new();
        let mut after_author = false;
        chunk.iter().enumerate().for_each(|(i, l)| {
            let trimmed = l.trim();
            if trimmed.is_empty() {
                return;
            }
            let author_line = trimmed[0..2] == *"--";
            if author_line || after_author {
                if after_author && author_line {
                    content.append(&mut unsure);
                }
                unsure.push(l.clone());

                after_author = true;
            } else {
                content.push(l.clone());
            }
        });

        author.append(
            &mut unsure
                .iter()
                .map(|l| {
                    let trimmed = l.trim();
                    if trimmed[0..2] == *"--" {
                        return trimmed[2..].trim().to_owned();
                    } else {
                        return trimmed.to_owned();
                    }
                })
                .collect(),
        );

        return Quote::new(content.join(" "), author.join(" "));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_file() {}

    #[test]
    fn test_process_chunk_author_twoline() {
        let chunk = vec![
            "Actresses will happen in the best regulated families.".to_string(),
            "\t\t-- Addison Mizner and Oliver Herford, 'The Entirely".to_string(),
            "\t\tNew Cynic's Calendar', 1905".to_string(),
        ];

        let quote = FortuneFile::process_chunk(&chunk);
        assert_eq!(
            quote,
            Quote::new(
                "Actresses will happen in the best regulated families.".to_string(),
                "Addison Mizner and Oliver Herford, 'The Entirely New Cynic's Calendar', 1905"
                    .to_string()
            )
        )
    }

    #[test]
    fn test_process_chunk_complicated() {
        let chunk = vec![
            "A 'critic' is a man who creates nothing and thereby feels qualified to".to_owned(),
            "judge the work of creative men. There is logic in this; he is unbiased".to_owned(),
            "-- he hates all creative people equally.".to_owned(),
            "\t\t-- Robert Heinlein".to_owned(),
        ];

        let quote = FortuneFile::process_chunk(&chunk);
        assert_eq!(quote, Quote::new("A 'critic' is a man who creates nothing and thereby feels qualified to judge the work of creative men. There is logic in this; he is unbiased -- he hates all creative people equally.".to_owned(), "Robert Heinlein".to_owned()));
    }
}
