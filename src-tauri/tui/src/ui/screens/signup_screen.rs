use color_eyre::eyre::Result;
use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use shared::auth_util::AuthUtil;

use crate::ui::{
    tui::Cmd,
    widgets::{checkbox::CheckboxWidget, input_field::InputWidget},
};

use super::{home_page_screen::HomePageScreen, login_screen::LoginScreen, screen::Screen};

#[derive(Clone)]
pub(crate) struct SignupScreen {
    username_input: InputWidget,
    email_input: InputWidget,
    password_input: InputWidget,
    remember_me_btn: CheckboxWidget,
}

impl SignupScreen {
    pub(crate) fn new() -> Self {
        Self {
            username_input: InputWidget::new(String::from("Username"), 'u'),
            email_input: InputWidget::new(String::from("Email"), 'e'),
            password_input: InputWidget::new(String::from("Password"), 'p'),
            remember_me_btn: CheckboxWidget::new(String::from("Remember me"), 'r'),
        }
    }

    fn handle_char(&mut self, key: KeyCode) {
        if self.username_input.is_focused() {
            self.username_input.handle_input(key);
        } else if self.email_input.is_focused() {
            self.email_input.handle_input(key);
        } else if self.password_input.is_focused() {
            self.password_input.handle_input(key);
        }
    }

    fn attempt_signup(&mut self) -> Cmd {
        let signup_attempt = AuthUtil::attempt_signup(
            None,
            self.username_input.input().to_string(),
            self.email_input.input().to_string(),
            self.password_input.input().to_string(),
            self.remember_me_btn.is_checked(),
        );
        if let Ok(()) = signup_attempt {
            return Cmd::NavigateTo(Box::new(HomePageScreen::new()));
        } else if let Err(e) = signup_attempt {
            // TODO: Handle accordingly
            panic!("{}", e)
        }
        return Cmd::None;
    }
}

impl Screen for SignupScreen {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn render(&self, f: &mut Frame, bounds: Rect) -> Result<()> {
        let input_height = 3;
        let input_width = bounds.width / 2;
        let padding = 3;

        // Calculate the starting position for centering
        let total_height = input_height * 2; // Total height for both input fields
        let start_y = (bounds.height - total_height) / 2 - 3; // Vertical center
        let start_x = (bounds.width - input_width) / 2; // Horizontal center

        let username_bounds = Rect::new(start_x, start_y, input_width, input_height);
        self.username_input.render(f, username_bounds);

        let email_bounds = Rect::new(
            start_x,
            start_y + input_height + padding,
            input_width,
            input_height,
        );
        self.email_input.render(f, email_bounds);

        let password_bounds = Rect::new(
            start_x,
            start_y + (input_height + padding) * 2,
            input_width,
            input_height,
        );
        self.password_input.render(f, password_bounds);

        let checkbox_bounds = Rect::new(start_x, start_y + 6 * input_height, input_width, 1);
        self.remember_me_btn.render(f, checkbox_bounds);

        let login_info_box_bounds = Rect::new(start_x, start_y + 7 * input_height, input_width, 1);
        let paragraph = Paragraph::new("Already have an account? Press 'l' to log in");
        f.render_widget(paragraph, login_info_box_bounds);

        // Set cursor for the focused input field
        //if self.email_input.is_focused() {
        //    let cursor_x = email_bounds.x + self.email_input.cursor_position as u16 + 1; // +1 for the border
        //    let cursor_y = email_bounds.y + 1; // +1 for the border
        //    f.set_cursor(cursor_x, cursor_y);
        //} else if self.password_input.is_focused() {
        //    let cursor_x = password_bounds.x + self.password_input.cursor_position as u16 + 1; // +1 for the border
        //    let cursor_y = password_bounds.y + 1; // +1 for the border
        //    f.set_cursor(cursor_x, cursor_y);
        //}

        Ok(())
    }

    fn unfocus_all(&mut self) -> Cmd {
        self.username_input.set_focus(false);
        self.email_input.set_focus(false);
        self.password_input.set_focus(false);
        return Cmd::ChangeMode;
    }

    fn cycle_input_fields(&mut self) {
        if self.username_input.is_focused() {
            self.unfocus_all();
            self.email_input.set_focus(true);
        } else if self.email_input.is_focused() {
            self.unfocus_all();
            self.password_input.set_focus(true);
        } else if self.password_input.is_focused() {
            self.unfocus_all();
            self.username_input.set_focus(true);
        } else {
            panic!("Can't cycle input fields if not in edit mode.")
        }
    }

    fn handle_input(&mut self, key: KeyCode) -> Cmd {
        match key {
            KeyCode::Esc => return self.unfocus_all(),
            KeyCode::Enter => return self.attempt_signup(),
            KeyCode::Tab => {
                self.cycle_input_fields();
                return Cmd::None;
            }
            _ => {
                self.handle_char(key);
                return Cmd::None;
            }
        };
    }

    fn handle_cmd(&mut self, key: KeyCode) -> Cmd {
        if key == KeyCode::Char(self.username_input.key) {
            self.username_input.set_focus(true);
            return Cmd::ChangeMode;
        } else if key == KeyCode::Char(self.email_input.key) {
            self.email_input.set_focus(true);
            return Cmd::ChangeMode;
        } else if key == KeyCode::Char(self.password_input.key) {
            self.password_input.set_focus(true);
            return Cmd::ChangeMode;
        } else if key == KeyCode::Char(self.remember_me_btn.key) {
            self.remember_me_btn.toggle();
        } else if key == KeyCode::Char('l') {
            return Cmd::NavigateTo(Box::new(LoginScreen::new()));
        }

        return Cmd::None;
    }
}
