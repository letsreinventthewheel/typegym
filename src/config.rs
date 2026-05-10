use clap::Parser;
use crate::text::TextSource;

#[derive(Debug, Parser)]
pub struct Config {
    /// ANSI color for incorrectly typed character
    #[arg(long, default_value_t = 1)]
    pub fg_miss: u8,

    /// ANSI color for untyped character
    #[arg(long, default_value_t = 8)]
    pub fg_empty: u8,

    /// ANSI color for results line
    #[arg(long, default_value_t = 1)]
    pub fg_results: u8,

    /// The way we get text for our practice sessions
    #[arg(
        long,
        default_value = "markov:data/markov.txt",
        value_name = "SOURCE",
        help = "Text source: static, nonsense, weighted, file:<path> or markov:<path>"
    )]
    pub text_source: TextSource,

    /// Maximum number of words in generated paragraph
    #[arg(long, default_value_t = 100)]
    pub max_words: usize,

    /// Maximum number of characters per line in reflowed text
    #[arg(long, default_value_t = 80, requires = "reflow")]
    pub width: usize,

    /// Reflow text using target width
    #[arg(long, default_value_t = false)]
    pub reflow: bool,
}
