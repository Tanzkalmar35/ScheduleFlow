use ratatui::Frame;
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, BorderType, Paragraph};

pub fn render(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!(
            "\nPress `Esc`, `Ctrl-C` or `q` to stop running.\n"
        ))
            .block(
                Block::default()
                    .title(" Calendar app ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
        frame.size(),
    )
}
