use std::{iter::repeat, time::Instant};

use crate::character::{classify_character, Character};

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
            input: "Thu  is a bear minimum ".to_string(),
            session_start: None,
            session_end: None,
            strokes: 0,
            hits: 0,
            should_loop: false,
        }
    }

    pub fn apply_char(&self, _c: char) {
        todo!("apply char")
    }

    pub fn apply_backspace(&self) {
        todo!("apply backspace")
    }

    pub fn apply_backspace_word(&self) {
        todo!("apply backspace word")
    }

    pub fn is_complete(&self) -> bool {
        self.target == self.input
    }

    pub fn has_started(&self) -> bool {
        todo!("has started")
    }

    pub fn start_clock(&self) {
        todo!("start clock")
    }

    pub fn stop_clock(&self) {
        todo!("stop clock")
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
}
