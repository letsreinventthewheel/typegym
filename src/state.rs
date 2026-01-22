use std::time::Instant;

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
    pub should_loop: bool
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
}
