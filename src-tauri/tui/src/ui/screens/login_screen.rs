use color_eyre::eyre::Result;
use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use shared::auth_util::AuthUtil;

use crate::ui::tui::Cmd;

use super::{
    super::widgets::input_field::InputWidget, home_page_screen::HomePageScreen, screen::Screen,
};

#[derive(Clone)]
pub(crate) struct LoginScreen {
    email: InputWidget,
    password: InputWidget,
}

impl LoginScreen {
    pub(crate) fn new() -> Self {
        Self {
            email: InputWidget::new(String::from("Email"), 'e'),
            password: InputWidget::new(String::from("Password"), 'p'),
        }
    }
}

impl Screen for LoginScreen {
    fn render(&self, f: &mut Frame, bounds: Rect) -> Result<()> {
        let input_height = 3;
        let input_width = bounds.width / 2;
        let padding = 3;

        // Calculate the starting position for centering
        let total_height = input_height * 2; // Total height for both input fields
        let start_y = (bounds.height - total_height) / 2 - 3; // Vertical center
        let start_x = (bounds.width - input_width) / 2; // Horizontal center

        let email_bounds = Rect::new(start_x, start_y, input_width, input_height);
        self.email.render(f, email_bounds);

        let password_bounds = Rect::new(
            start_x,
            start_y + input_height + padding,
            input_width,
            input_height,
        );
        self.password.render(f, password_bounds);

        // Set cursor for the focused input field
        if self.email.is_focused() {
            let cursor_x = email_bounds.x + self.email.cursor_position as u16 + 1; // +1 for the border
            let cursor_y = email_bounds.y + 1; // +1 for the border
            f.set_cursor(cursor_x, cursor_y);
        } else if self.password.is_focused() {
            let cursor_x = password_bounds.x + self.password.cursor_position as u16 + 1; // +1 for the border
            let cursor_y = password_bounds.y + 1; // +1 for the border
            f.set_cursor(cursor_x, cursor_y);
        }

        Ok(())
    }

    fn handle_input(&mut self, key: KeyCode) -> Cmd {
        if key == KeyCode::Esc {
            self.email.set_focus(false);
            self.password.set_focus(false);
            return Cmd::ChangeMode;
        } else if key == KeyCode::Enter {
            let login_attempt = AuthUtil::attempt_login(
                None,
                self.email.input().to_string(),
                self.password.input().to_string(),
                true,
            );
            if let Ok(()) = login_attempt {
                return Cmd::NavigateTo(Box::new(HomePageScreen::new()));
            } else if let Err(e) = login_attempt {
                panic!("{}", e)
            }
            return Cmd::None;
        }

        if self.email.is_focused() {
            self.email.handle_input(key);
        } else if self.password.is_focused() {
            self.password.handle_input(key);
        }

        Cmd::None
    }

    fn handle_cmd(&mut self, key: KeyCode) -> Cmd {
        if key == KeyCode::Char(self.email.key) {
            self.email.set_focus(true);
            return Cmd::ChangeMode;
        } else if key == KeyCode::Char(self.password.key) {
            self.password.set_focus(true);
            return Cmd::ChangeMode;
        }

        return Cmd::None;
    }
}
