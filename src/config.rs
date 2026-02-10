use crate::text::TextSource;

#[derive(Debug)]
pub struct Config {
    /// ANSI color for incorrectly typed character
    pub fg_miss: u8,

    /// ANSI color for untyped character
    pub fg_empty: u8,

    /// ANSI color for results line
    pub fg_results: u8,

    /// The way we get text for our practice sessions
    pub text_source: TextSource,
}

impl Config {
    pub fn new() -> Self {
        Self {
            fg_miss: 1,
            fg_empty: 8,
            fg_results: 1,
            text_source: TextSource::File("src/main.rs".to_string()),
        }
    }
}

