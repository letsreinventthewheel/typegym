use core::str::FromStr;

use color_eyre::Result;
use rand::{seq::IndexedRandom, RngExt};

use crate::{config::Config, markov::MarkovChain};

const TEXT: &'static str =
    "This is a bare minimum example.   There are many approaches to running an application loop, so
this is not meant to be prescriptive. It is only meant to demonstrate the basic setup and
teardown of a terminal application.";

#[derive(Debug, Clone)]
pub enum TextSource {
    /// Hard coded text, always the same
    Static,

    /// Generate nonsense from a list of words
    GenerateNonsense,

    /// Generate nonsense from a list of weighted words
    GenerateWeightedNonsense,

    /// Grab lines from a file
    File(String),

    /// Generate text using Markov Chain
    MarkovChain(String),
}

impl FromStr for TextSource {
    type Err = String;

    fn from_str(source: &str) -> core::result::Result<Self, Self::Err> {
        match source {
            "static" => Ok(TextSource::Static),
            "nonsense" => Ok(TextSource::GenerateNonsense),
            "weighted" => Ok(TextSource::GenerateWeightedNonsense),
            _ => {
                if let Some(path) = source.strip_prefix("file:") {
                    if path.is_empty() {
                        Err("file source expects file path, .e.g file:data/words.txt".to_string())
                    } else {
                        Ok(TextSource::File(path.to_string()))
                    }
                } else if let Some(path) = source.strip_prefix("markov:") {
                    if path.is_empty() {
                        Err("markov source expects file path, .e.g markov:data/markov.txt".to_string())
                    } else {
                        Ok(TextSource::MarkovChain(path.to_string()))
                    }
                } else {
                    Err("expected static, nonsense, weighted, file:<path> or markov:<path>".to_string())
                }
            }
        }
    }
}

pub fn get_text(config: &Config) -> Result<String> {
    let text = match config.text_source {
        TextSource::Static => Ok(TEXT.to_string()),
        TextSource::GenerateNonsense => generate_nonsense(config.max_words),
        TextSource::GenerateWeightedNonsense => generate_weighted_nonsense(config.max_words),
        TextSource::File(ref path) => read_lines_from_file(path),
        TextSource::MarkovChain(ref path) => generate_markov_chain(path, config.max_words),
    }?;

    Ok(if config.reflow { reflow(&text, config.width) } else { text })
}

fn reflow(text: &str, width: usize) -> String {
    let mut lines = Vec::new();
    let mut line = String::new();
    let mut line_width = 0;

    for word in text.split_whitespace() {
        let word_width = word.chars().count();

        if line.is_empty() {
            line.push_str(word);
            line_width = word_width;
        } else if line_width + 1 + word_width <= width {
            line.push(' ');
            line.push_str(word);
            line_width += 1 + word_width;
        } else {
            lines.push(line);
            line = word.to_string();
            line_width = word_width;
        }
    }

    if !line.is_empty() {
        lines.push(line);
    }

    lines.join("\n")
}

fn generate_nonsense(max_words: usize) -> Result<String> {
    let contents = std::fs::read_to_string("data/words.txt")?;
    let words: Vec<_> = contents.lines().collect();

    let mut rng = rand::rng();
    let selected_words: Vec<_> = words.sample(&mut rng, max_words).copied().collect();
    let lines: Vec<_> = selected_words
        .chunks(10)
        .map(|chunk| chunk.join(" "))
        .collect();

    Ok(lines.join("\n"))
}

fn generate_weighted_nonsense(max_words: usize) -> Result<String> {
    let contents = std::fs::read_to_string("data/words_weighted.txt")?;
    let mut words = Vec::new();

    for line in contents.lines() {
        let parts: Vec<_> = line.split(" ").collect();
        assert_eq!(parts.len(), 2);
        let weight = parts[1].parse::<u32>()?;
        words.push((parts[0], weight));
    }

    let mut rng = rand::rng();
    let selected_words: Vec<_> = words
        .sample_weighted(&mut rng, max_words, |item| item.1)?
        .map(|item| item.0)
        .collect();

    let lines: Vec<_> = selected_words
        .chunks(10)
        .map(|chunk| chunk.join(" "))
        .collect();

    Ok(lines.join("\n"))
}

fn read_lines_from_file(path: &str) -> Result<String> {
    let contents = std::fs::read_to_string(path)?;
    let lines: Vec<_> = contents.lines().collect();

    if lines.is_empty() {
        return Ok("".to_string());
    }

    let max_lines = lines.len().min(5);
    let max_start_index = lines.len() - max_lines;
    let mut rng = rand::rng();
    let start_index = if max_start_index > 0 {
        rng.random_range(0..=max_start_index)
    } else {
        0
    };

    let selected_lines = &lines[start_index..start_index + max_lines];
    Ok(selected_lines.join("\n"))
}

fn generate_markov_chain(path: &str, max_words: usize) -> Result<String> {
    let contents = std::fs::read_to_string(path)?;
    let chain = MarkovChain::build(&contents);
    Ok(chain.generate(max_words))
}
