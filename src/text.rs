use color_eyre::Result;
use rand::{seq::IndexedRandom, RngExt};

use crate::config::Config;

const TEXT: &'static str =
    "This is a bare minimum example.   There are many approaches to running an application loop, so
this is not meant to be prescriptive. It is only meant to demonstrate the basic setup and
teardown of a terminal application.";

#[derive(Debug)]
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
    // TODO: MarkovChain(String),
}

pub fn get_text(config: &Config) -> Result<String> {
    match config.text_source {
        TextSource::Static => Ok(TEXT.to_string()),
        TextSource::GenerateNonsense => generate_nonsense(),
        TextSource::GenerateWeightedNonsense => generate_weighted_nonsense(),
        TextSource::File(ref path) => read_lines_from_file(path),
    }
}

fn generate_nonsense() -> Result<String> {
    let contents = std::fs::read_to_string("data/words.txt")?;
    let words: Vec<_> = contents.lines().collect();

    let mut rng = rand::rng();
    let selected_words: Vec<_> = words.sample(&mut rng, 100).copied().collect();
    let lines: Vec<_> = selected_words
        .chunks(10)
        .map(|chunk| chunk.join(" "))
        .collect();

    Ok(lines.join("\n"))
}

fn generate_weighted_nonsense() -> Result<String> {
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
        .sample_weighted(&mut rng, 100, |item| item.1)?
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
