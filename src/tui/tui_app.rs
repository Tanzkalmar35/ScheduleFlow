#[derive(Debug, Default)]
pub struct TuiApp {
    pub should_quit: bool,
}

impl TuiApp {
    /// Constructs a new instance of TuiApp.
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set the running to false to quit the application immediately
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
