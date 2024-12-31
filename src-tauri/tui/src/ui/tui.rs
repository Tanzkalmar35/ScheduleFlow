use color_eyre::Result;
use crossterm::{
    event::{self, KeyCode, KeyEvent},
    terminal, ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    widgets::{Block, Borders},
    Frame, Terminal,
};
use std::io;

use super::{
    home_page_screen::HomePageScreen, login_screen::LoginScreen, signup_screen::SignupScreen,
};

enum AppState {
    LoginScreen,
    SignupScreen,
    HomePageScreen,
}

pub(crate) struct Tui {
    state: AppState,
}

impl Tui {
    fn new() -> Self {
        Self {
            state: AppState::LoginScreen,
        }
    }

    pub(crate) fn start() -> Result<()> {
        color_eyre::install()?;

        // Initialize terminal
        let mut stdout = io::stdout();
        terminal::enable_raw_mode()?;
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let tui = Tui::new();

        loop {
            terminal.draw(|f| tui.render(f))?;

            // Handle user input
            if event::poll(std::time::Duration::from_millis(100))? {
                if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                    match code {
                        //KeyCode::Char('l') => tui.state = AppState::LoginScreen,
                        //KeyCode::Char('s') => tui.state = AppState::SignupScreen,
                        //KeyCode::Char('h') => tui.state = AppState::HomePageScreen,
                        KeyCode::Char('q') => break, // Quit the application
                        _ => {}
                    }
                }
            }
        }

        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        match self.state {
            AppState::LoginScreen => self.render_login_screen(frame),
            AppState::SignupScreen => self.render_signup_screen(frame),
            AppState::HomePageScreen => self.render_home_page_screen(frame),
        }
    }

    fn render_login_screen(&self, frame: &mut Frame) {
        let block = Block::bordered()
            .title(" Login - ScheduleFlow ")
            .borders(Borders::ALL);
        frame.render_widget(block, frame.area());

        let bounds = Rect::new(2, 1, frame.area().width - 4, frame.area().height - 2);
        LoginScreen::new().render(frame, bounds);
    }

    fn render_signup_screen(&self, frame: &mut Frame) {
        let block = Block::bordered()
            .title(" Signup - ScheduleFlow")
            .borders(Borders::ALL);
        frame.render_widget(block, frame.area());

        let bounds = Rect::new(2, 1, frame.area().width - 4, frame.area().height - 2);
        SignupScreen::render(frame, bounds);
    }

    fn render_home_page_screen(&self, frame: &mut Frame) {
        let block = Block::bordered()
            .title(" Home Page - ScheduleFlow ")
            .borders(Borders::ALL);
        frame.render_widget(block, frame.area());

        let bounds = Rect::new(2, 1, frame.area().width - 4, frame.area().height - 2);
        HomePageScreen::render(frame, bounds);
    }
}
