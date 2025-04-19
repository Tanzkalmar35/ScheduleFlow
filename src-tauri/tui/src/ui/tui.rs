use std::{boxed::Box, io, ops::DerefMut};

use color_eyre::Result;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    text::{Line, Text},
    widgets::Paragraph,
    Frame, Terminal,
};
use shared::{auth_util::AuthUtil, current::driver};

use crate::constants;

use super::screens::{
    home_page_screen::HomePageScreen, login_screen::LoginScreen, screen::Screen,
    signup_screen::SignupScreen,
};

enum AppState {
    LoginScreen,
    SignupScreen,
    HomePageScreen,
}

#[derive(PartialEq)]
enum Mode {
    EDIT,
    NORMAL,
}

pub(crate) enum Cmd {
    ChangeMode,
    NavigateTo(Box<dyn Screen>),
    None,
}

impl PartialEq for Cmd {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Cmd::ChangeMode, Cmd::ChangeMode) => true,
            (Cmd::None, Cmd::None) => true,
            (Cmd::NavigateTo(_screen_a), Cmd::NavigateTo(_screen_b)) => {
                todo!("At this point of time, I don't think I need this. If I'm wrong, sorry future me :-C")
            }
            _ => false,
        }
    }
}

pub(crate) struct Tui {
    state: AppState,
    current_mode: Mode,
    login_screen: LoginScreen,
    signup_screen: SignupScreen,
    home_page_screen: HomePageScreen,
}

impl Tui {
    fn new() -> Self {
        Self {
            state: AppState::SignupScreen,
            current_mode: Mode::NORMAL,
            login_screen: LoginScreen::new(),
            signup_screen: SignupScreen::new(),
            home_page_screen: HomePageScreen::new(),
        }
    }

    fn get_active_window(&mut self) -> Box<&mut dyn Screen> {
        match self.state {
            AppState::SignupScreen => Box::new(&mut self.signup_screen),
            AppState::LoginScreen => Box::new(&mut self.login_screen),
            AppState::HomePageScreen => Box::new(&mut self.home_page_screen),
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

        let mut tui = Tui::new();

        // TODO: Check db conn

        //if AuthUtil::is_valid_session(driver().lock().unwrap().deref_mut()) {
        //    tui.state = AppState::HomePageScreen;
        //}

        loop {
            terminal.draw(|f| {
                let bounds = Rect::new(2, 1, f.area().width - 4, f.area().height - 2);
                tui.render(f, bounds);
            })?;

            // Handle user input
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match tui.current_mode {
                        Mode::EDIT => match tui.get_active_window().handle_input(key.code) {
                            Cmd::ChangeMode => tui.change_mode(),
                            Cmd::NavigateTo(screen) => tui.navigate_to(&screen),
                            Cmd::None => {}
                            _ => {}
                        },
                        Mode::NORMAL => {
                            match key.code {
                                KeyCode::Char('q') => break, // Quit the application
                                _ => {
                                    if let Cmd::NavigateTo(screen) =
                                        tui.get_active_window().handle_cmd(key.code)
                                    {
                                        tui.navigate_to(&screen);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        terminal::disable_raw_mode()?;
        execute!(terminal.backend_mut(), terminal::Clear(ClearType::All))?;
        Ok(())
    }

    fn render(&self, frame: &mut Frame, bounds: Rect) -> Box<dyn Screen + 'static> {
        match self.state {
            AppState::LoginScreen => self.render_login_screen(frame, bounds),
            AppState::SignupScreen => self.render_signup_screen(frame, bounds),
            AppState::HomePageScreen => {
                self.home_page_screen.render(frame, bounds);
                Box::new(self.home_page_screen.clone())
            }
        }
    }

    fn render_signup_screen(&self, frame: &mut Frame, bounds: Rect) -> Box<dyn Screen + 'static> {
        let mode = if self.current_mode == Mode::NORMAL {
            "Normal"
        } else {
            "Edit"
        };

        // Render ascii banner
        let ascii: Vec<Line> = constants::BANNER
            .lines()
            .map(|line| Line::from(line))
            .collect();
        let ascii_art = Paragraph::new(Text::from(ascii)).alignment(Alignment::Center);
        let ascii_art_bounds = Rect::new(0, 0, bounds.width, constants::BANNER_HEIGHT as u16);
        frame.render_widget(ascii_art, ascii_art_bounds);

        let info_text_bounds = Rect::new(0, constants::BANNER_HEIGHT as u16 + 2, bounds.width, 1);
        let info_text = Paragraph::new(Text::from(
            "Sign up - Press 'h' in normal mode for help, 'q' to quit - Current mode: ".to_owned()
                + mode,
        ))
        .alignment(Alignment::Center);

        frame.render_widget(info_text, info_text_bounds);

        // Render the rest of the login screen (The input fields)
        self.signup_screen
            .render(frame, bounds)
            .expect("Could not render login screen");
        Box::new(self.signup_screen.clone())
    }

    fn render_login_screen(&self, frame: &mut Frame, bounds: Rect) -> Box<dyn Screen + 'static> {
        let mode = if self.current_mode == Mode::NORMAL {
            "Normal"
        } else {
            "Edit"
        };

        // Render ascii banner
        let ascii: Vec<Line> = constants::BANNER
            .lines()
            .map(|line| Line::from(line))
            .collect();
        let ascii_art = Paragraph::new(Text::from(ascii)).alignment(Alignment::Center);
        let ascii_art_bounds = Rect::new(0, 0, bounds.width, constants::BANNER_HEIGHT as u16);
        frame.render_widget(ascii_art, ascii_art_bounds);

        let info_text_bounds = Rect::new(0, constants::BANNER_HEIGHT as u16 + 2, bounds.width, 1);
        let info_text = Paragraph::new(Text::from(
            "Login - Press 'h' in normal mode for help, 'q' to quit - Current mode: ".to_owned()
                + mode,
        ))
        .alignment(Alignment::Center);

        frame.render_widget(info_text, info_text_bounds);

        // Render the rest of the login screen (The input fields)
        self.login_screen
            .render(frame, bounds)
            .expect("Could not render login screen");
        Box::new(self.login_screen.clone())
    }

    fn change_mode(&mut self) {
        if self.current_mode == Mode::NORMAL {
            self.current_mode = Mode::EDIT;
        } else if self.current_mode == Mode::EDIT {
            self.current_mode = Mode::NORMAL;
        }
    }

    fn navigate_to(&mut self, screen: &Box<dyn Screen>) {
        if let Some(_) = screen.as_any().downcast_ref::<LoginScreen>() {
            self.state = AppState::LoginScreen;
            return;
        }
        if let Some(_) = screen.as_any().downcast_ref::<SignupScreen>() {
            self.state = AppState::SignupScreen;
            return;
        }
        if let Some(_) = screen.as_any().downcast_ref::<HomePageScreen>() {
            self.state = AppState::HomePageScreen;
            return;
        }
        panic!("WTF");
    }
}
