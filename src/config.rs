#[derive(Debug)]
pub struct Config {
    /// ANSI color for incorrectly typed character
    pub fg_miss: u8,

    /// ANSI color for untyped character
    pub fg_empty: u8,

    /// ANSI color for results line
    pub fg_results: u8,
}

impl Config {
    pub fn new() -> Self {
        Self {
            fg_miss: 1,
            fg_empty: 8,
            fg_results: 1,
        }
    }
}

