use std::collections::HashMap;

use rand::seq::IndexedRandom;

type Key = (String, String);

#[derive(Debug, Default)]
pub struct MarkovChain {
    pub transitions: HashMap<Key, Vec<String>>,
    pub starters: Vec<Key>,
}

impl MarkovChain {
    pub fn build(text: &str) -> Self {
        let mut chain = Self::default();

        for sentence in sentences(text) {
            let words: Vec<_> = sentence
                .split_whitespace()
                .map(cleanup_string)
                .filter(|w| !w.is_empty())
                .collect();

            if words.len() < 3 {
                continue;
            }

            chain.starters.push((words[0].clone(), words[1].clone()));

            for window in words.windows(3) {
                let key = (window[0].clone(), window[1].clone());
                chain
                    .transitions
                    .entry(key)
                    .or_default()
                    .push(window[2].clone());
            }
        }

        chain
    }

    pub fn generate(&self) -> String {
        let mut rng = rand::rng();
        let mut words: Vec<String> = Vec::new();

        while words.len() < 100 {
            let Some(starter) = self.starters.choose(&mut rng) else {
                return Default::default();
            };

            let (mut previous, mut current) = starter.clone();

            words.push(previous.clone());
            words.push(current.clone());

            loop {
                let Some(choices) = self.transitions.get(&(previous, current.clone())) else {
                    break;
                };

                let Some(next) = choices.choose(&mut rng) else {
                    break;
                };

                words.push(next.clone());
                previous = current;
                current = next.clone();
            }
        }

        let lines: Vec<_> = words.chunks(10).map(|chunk| chunk.join(" ")).collect();
        lines.join("\n")
    }
}

fn cleanup_string(string: &str) -> String {
    let new_string: String = string
        .chars()
        .map(|c| match c {
            '“' => '"',
            '”' => '"',
            '’' => '\'',
            '‘' => '\'',
            '—' => '-',
            ch => ch
        })
        .collect();

    new_string
        .trim_matches(|c| matches!(c, ' ' | '"' | '\'' | '_' | '(' | ')' | '[' | ']'))
        .to_string()
}

fn sentences(text: &str) -> Vec<String> {
    text.split_inclusive(|c| c == '.' || c == '!' || c == '?')
        .map(cleanup_string)
        .filter(|s| !s.is_empty())
        .collect()
}
