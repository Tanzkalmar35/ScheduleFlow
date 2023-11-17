use ratatui::Frame;
use ratatui::layout::{Layout, Direction, Constraint, Alignment};
use ratatui::style::{Style, Color};
use ratatui::widgets::{Block, Borders};

pub fn render(frame: &mut Frame) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.size());
    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title(" ScheduleFlow ")
            .title_alignment(Alignment::Center)
            .style(Style::new()
            .fg(Color::Yellow)), // TODO: Make the coor adjustable
        main_layout[0],
    );

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(main_layout[1]);
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .title(" INNER ")
            .title_alignment(Alignment::Center)
            .style(Style::new()
            .fg(Color::Yellow)), // TODO: Make the coor adjustable
        inner_layout[0],
    );
}