use clap::Parser;
use color_eyre::Result;

use crate::{config::Config, state::State, text::get_text, ui::run_ui};

mod app;
mod character;
mod config;
mod markov;
mod state;
mod text;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?; // augment errors / panics with easy to read messages

    let config = Config::parse();

    loop {
        let text = get_text(&config)?;
        let state = State::new(text);
        let should_loop = run_ui(state, &config)?;
        if !should_loop {
            break;
        }
    }

    Ok(())
}
