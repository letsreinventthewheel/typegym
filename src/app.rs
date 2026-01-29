use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{character::Character, config::Config, state::State};

#[derive(Debug)]
pub struct App<'a> {
    pub config: &'a Config,
    pub state: State,
    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(state: State, config: &'a Config) -> Self {
        Self {
            config,
            state,
            should_quit: false,
        }
    }

    fn render_character(&self, character: &Character) -> Span<'_> {
        match character {
            Character::Hit(c) => Span::raw(c.to_string()),
            Character::Miss(c) => Span::styled(
                if *c == ' ' {
                    '_'.to_string()
                } else {
                    c.to_string()
                },
                Style::default()
                    .fg(Color::Indexed(self.config.fg_miss))
                    .add_modifier(Modifier::BOLD),
            ),
            Character::Empty(c) => Span::styled(
                c.to_string(),
                Style::default().fg(Color::Indexed(self.config.fg_empty)),
            ),
        }
    }

    pub fn draw(&self, frame: &mut Frame<'_>) {
        let page = self.state.build_page();

        let mut lines = vec![];
        for line in page.iter() {
            let spans: Vec<_> = line
                .iter()
                .map(|character| self.render_character(character))
                .collect();
            lines.push(Line::from(spans));
        }

        if self.state.is_complete() {
            let wpm = self.state.wpm();
            let accuracy = self.state.accuracy() * 100.0;

            let results = format!("{:.0} words per minute ~ {:.0}% accuracy", wpm, accuracy);
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                results,
                Style::default()
                    .fg(Color::Indexed(self.config.fg_results))
                    .add_modifier(Modifier::BOLD),
            )));

            lines.push(Line::from(""));
            lines.push(Line::from("Press Enter to quit, Esc to start new session"));
        }

        let text_height = lines.len() as u16;
        let vertical_margin = frame.area().height.saturating_sub(text_height) / 2;
        let max_line_width = lines
            .iter()
            .map(|line| line.width() as u16)
            .max()
            .unwrap_or(0);
        let horizontal_margin = frame.area().width.saturating_sub(max_line_width) / 2;

        let vertical = Layout::vertical([
            Constraint::Length(vertical_margin),
            Constraint::Length(text_height),
            Constraint::Min(0),
        ]);

        let [_, middle_area, _] = vertical.areas(frame.area());

        let horizontal = Layout::horizontal([
            Constraint::Length(horizontal_margin),
            Constraint::Length(max_line_width),
            Constraint::Min(0),
        ]);

        let [_, centered_area, _] = horizontal.areas(middle_area);

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, centered_area);

        if !self.state.is_complete() {
            let (cursor_row, cursor_col) = self.state.cursor();
            let cursor_x = centered_area.x + cursor_col as u16;
            let cursor_y = centered_area.y + cursor_row as u16;
            frame.set_cursor_position((cursor_x, cursor_y));
        }
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
