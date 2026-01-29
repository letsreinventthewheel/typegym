use std::{iter::repeat, time::Instant};

use crate::character::{Character, classify_character};

pub type Line = Vec<Character>;
pub type Page = Vec<Line>;

#[derive(Debug)]
pub struct State {
    /// The text user needs to type
    pub target: String,

    /// The text user has already typed in
    pub input: String,

    /// The time at which practice session started (first keystroke)
    pub session_start: Option<Instant>,

    /// The time at which practice session finished (was complete)
    pub session_end: Option<Instant>,

    /// The number of keystrokes user did
    pub strokes: u64,

    /// The number of correct keystrokes
    pub hits: u64,

    /// A flag indicating whether we want to start another practice session
    pub should_loop: bool,
}

impl State {
    pub fn new(text: String) -> Self {
        Self {
            target: text,
            input: "".to_string(),
            session_start: None,
            session_end: None,
            strokes: 0,
            hits: 0,
            should_loop: false,
        }
    }

    pub fn apply_char(&mut self, c: char) {
        let target_count = self.target.chars().count();
        let input_count = self.input.chars().count();

        if input_count >= target_count {
            return;
        }

        let to_append = if c.is_whitespace() {
            let ws: String = self
                .target
                .chars()
                .skip(input_count)
                .take_while(|c| c.is_whitespace())
                .collect();
            if ws.is_empty() { " ".to_string() } else { ws }
        } else {
            c.to_string()
        };

        self.input.push_str(&to_append);

        self.strokes += 1;
        if self.is_error_free() {
            self.hits += 1;
        }
    }

    pub fn apply_backspace(&mut self) {
        if self.input.is_empty() {
            return;
        }

        let matching_count = self.input
            .chars()
            .zip(self.target.chars())
            .collect::<Vec<_>>()
            .iter()
            .rev()
            .take_while(|(i, t)| i.is_whitespace() && t.is_whitespace())
            .count();

        let to_remove = matching_count.max(1);
        let new_len = self.input.chars().count().saturating_sub(to_remove);
        self.input = self.input.chars().take(new_len).collect();
    }

    pub fn apply_backspace_word(&mut self) {
        if self.input.is_empty() {
            return;
        }

        let mut count = 0;
        let mut seen_non_whitespace = false;

        for c in self.input.chars().rev() {
            if !seen_non_whitespace && c.is_whitespace() {
                // Part 1: removing all trailing whitespaces
                count += 1;
            } else if !c.is_whitespace() {
                // Part 2: remove the word (all non-whitespace characters)
                count += 1;
                seen_non_whitespace = true;
            } else if seen_non_whitespace && c.is_whitespace() {
                // Part 3: remove single whitespace before the word, then stop
                count += 1;
                break;
            }
        }

        let new_len = self.input.chars().count().saturating_sub(count);
        self.input = self.input.chars().take(new_len).collect();
    }

    fn is_error_free(&self) -> bool {
        self.target.starts_with(&self.input)
    }

    pub fn is_complete(&self) -> bool {
        self.target == self.input
    }

    pub fn has_started(&self) -> bool {
        self.session_start.is_some()
    }

    pub fn start_clock(&mut self) {
        self.session_start = Some(Instant::now());
    }

    pub fn stop_clock(&mut self) {
        self.session_end = Some(Instant::now());
    }

    fn build_line(&self, target: &str, input: &str) -> Line {
        let target_chars = target.chars().map(Some).chain(repeat(None));
        let input_chars = input.chars().map(Some).chain(repeat(None));
        let max_length = target.chars().count().max(input.chars().count());

        target_chars
            .zip(input_chars)
            .take(max_length)
            .map(|(t, i)| classify_character(t, i))
            .collect()
    }

    pub fn build_page(&self) -> Page {
        let target_lines: Vec<_> = self.target.lines().collect();
        let mut input_lines: Vec<_> = self.input.lines().collect();

        while input_lines.len() < target_lines.len() {
            input_lines.push("");
        }

        target_lines
            .iter()
            .zip(input_lines.iter())
            .map(|(&target, &input)| self.build_line(target, input))
            .collect()
    }

    pub fn cursor_row(&self) -> usize {
        self.input.chars().filter(|&c| c == '\n').count()
    }

    pub fn cursor_col(&self) -> usize {
        self.input.chars().rev().take_while(|&c| c != '\n').count()
    }

    pub fn cursor(&self) -> (usize, usize) {
        (self.cursor_row(), self.cursor_col())
    }

    fn elapsed_seconds(&self) -> f64 {
        match (self.session_start, self.session_end) {
            (Some(start), Some(end)) => end.duration_since(start).as_secs_f64(),
            _ => 0.0
        }
    }

    fn chars_count(&self) -> usize {
        let mut count = 0;
        let mut in_whitespace_group = false;

        for c in self.target.chars() {
            if c.is_whitespace() {
                if !in_whitespace_group {
                    count += 1;
                    in_whitespace_group = true;
                }
            } else {
                count += 1;
                in_whitespace_group = false;
            }
        }

        count
    }

    pub fn wpm(&self) -> f64 {
        let seconds = self.elapsed_seconds();
        if seconds == 0.0 {
            return 0.0;
        }

        let chars = self.chars_count() as f64;
        (chars / 5.0) / (seconds / 60.0)
    }

    pub fn accuracy(&self) -> f64 {
        if self.strokes == 0 {
            return 1.0;
        }

        self.hits as f64 / self.strokes as f64
    }
}
