use std::{io::stdout, time::Duration};

use color_eyre::Result;
use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event}, execute};

use crate::{app::App, config::Config, state::State};

pub fn run_ui(state: State, config: &Config) -> Result<bool> {
    let mut terminal = ratatui::init();
    execute!(stdout(), EnableMouseCapture)?;

    let mut app = App::new(state, config);

    loop {
        terminal.draw(|frame| app.draw(frame))?;

        if app.should_quit {
            break;
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_code) = event::read()? {
                app.handle_key_event(key_code.code, key_code.modifiers);
            }
        }
    }

    ratatui::restore();
    execute!(stdout(), DisableMouseCapture)?;

    Ok(app.state.should_loop)
}
