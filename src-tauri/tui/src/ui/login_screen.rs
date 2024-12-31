use ratatui::{
    layout::{Direction, Layout, Rect},
    prelude::Constraint,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub(crate) struct LoginScreen {
    email: String,
    password: String,
}

impl LoginScreen {
    pub(crate) fn new() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
        }
    }

    pub(crate) fn render(&self, f: &mut Frame, bounds: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3), // Padding above
                Constraint::Length(3), // Input 1
                Constraint::Length(1), // Padding between inputs
                Constraint::Length(3), // Input 2
            ])
            .split(f.area());

        // Render Input 1
        let input1_paragraph =
            Paragraph::new(&*self.email).block(Block::default().borders(Borders::NONE));

        f.render_widget(input1_paragraph, layout[1]);

        // Render Input 2
        let input2_paragraph =
            Paragraph::new(&*self.password).block(Block::default().borders(Borders::NONE));

        f.render_widget(input2_paragraph, layout[3]);
    }
}
