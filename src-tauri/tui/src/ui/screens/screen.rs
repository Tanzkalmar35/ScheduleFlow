use color_eyre::eyre::Result;
use crossterm::event::KeyCode;
use ratatui::{layout::Rect, Frame};

use crate::ui::tui::Cmd;

pub(crate) trait Screen {
    fn render(&self, f: &mut Frame, bounds: Rect) -> Result<()>;
    fn handle_input(&mut self, key: KeyCode) -> Cmd;
    fn handle_cmd(&mut self, key: KeyCode) -> Cmd;
}
