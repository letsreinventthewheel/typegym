use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{widgets::Paragraph, Frame};

use crate::{config::Config, state::State};

#[derive(Debug)]
pub struct App {
    pub state: State,
    pub should_quit: bool
}

impl App {
    pub fn new(state: State, _config: &Config) -> Self {
        Self {
            state,
            should_quit: false,
        }
    }

    pub fn draw(&self, frame: &mut Frame<'_>) {
        let paragraph = Paragraph::new(self.state.target.clone());
        frame.render_widget(paragraph, frame.area());
    }

    pub fn handle_key_event(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        if self.state.is_complete() {
            match key {
                KeyCode::Enter => {
                    self.state.should_loop = false;
                    self.should_quit = true;
                }
                KeyCode::Esc => {
                    self.state.should_loop = true;
                    self.should_quit = true;
                }
                _ => {
                    // ignore the rest
                }
            }
            return;
        }

        // Ctrl+key takes precedence
        if modifiers.contains(KeyModifiers::CONTROL) {
            match key {
                KeyCode::Char('c') => {
                    self.state.should_loop = false;
                    self.should_quit = true;
                }
                KeyCode::Char('w') => {
                    self.state.apply_backspace_word();
                }
                _ => {
                    // ignore all other Ctrl+key combinations
                }
            }
            return;
        }

        match key {
            KeyCode::Char(c) => {
                if !self.state.has_started() {
                    self.state.start_clock();
                }

                self.state.apply_char(c);

                if self.state.is_complete() {
                    self.state.stop_clock();
                }
            }
            KeyCode::Enter => {
                if !self.state.has_started() {
                    self.state.start_clock();
                }

                self.state.apply_char('\n');

                if self.state.is_complete() {
                    self.state.stop_clock();
                }
            }
            KeyCode::Backspace => {
                self.state.apply_backspace();
            }
            KeyCode::Esc => {
                self.state.should_loop = true;
                self.should_quit = true;
            }
            _ => {
                // ignore all the rest
            }
        }
    }
}
