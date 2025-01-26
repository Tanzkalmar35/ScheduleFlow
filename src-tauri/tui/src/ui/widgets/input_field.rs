use crossterm::event::KeyCode;
use ratatui::{
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph},
};

#[derive(Clone)]
pub(crate) struct InputWidget {
    title: String,
    pub(crate) input: String,
    pub(crate) cursor_position: usize,
    pub(crate) key: char, // Key to focus this input
    focused: bool,        // Whether this input is focused
}

impl InputWidget {
    pub(crate) fn new(title: String, key: char) -> Self {
        Self {
            title: " ".to_owned() + &title + " - " + &key.to_string() + " ",
            input: String::new(),
            cursor_position: 0,
            key,
            focused: false,
        }
    }

    pub(crate) fn set_focus(&mut self, focus: bool) {
        self.focused = focus;
    }

    pub(crate) fn is_focused(&self) -> bool {
        return self.focused;
    }

    pub(crate) fn handle_input(&mut self, key: KeyCode) {
        if self.focused {
            match key {
                KeyCode::Char(c) => {
                    self.input.insert(self.cursor_position, c);
                    self.cursor_position += 1;
                }
                KeyCode::Backspace => {
                    if self.cursor_position > 0 {
                        self.input.remove(self.cursor_position - 1);
                        self.cursor_position -= 1;
                    }
                }
                KeyCode::Left => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.cursor_position < self.input.len() {
                        self.cursor_position += 1;
                    }
                }
                // If this causes pain in the future, sorry me :C
                _ => {}
            }
        }
    }

    pub(crate) fn render(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let input_paragraph = Paragraph::new(Text::from(Span::raw(&self.input)))
            .style(if self.focused {
                Style::default().fg(Color::Yellow) // Change color if focused
            } else {
                Style::default().fg(Color::White)
            })
            .block(Block::default().title(&*self.title).borders(Borders::ALL));

        frame.render_widget(input_paragraph, area);

        // Set cursor position
        if self.is_focused() {
            let cursor_x = area.x + self.cursor_position as u16 + 1; // +1 for the border
            let cursor_y = area.y + 1; // +1 for the border
            frame.set_cursor(cursor_x, cursor_y);
        }
    }

    pub(crate) fn input(&self) -> &str {
        &self.input
    }
}
