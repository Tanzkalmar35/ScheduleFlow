use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders};
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use time::Date;

type Layouts = std::rc::Rc<[ratatui::prelude::Rect]>;

pub fn render(frame: &mut Frame) {
    let main_layout: Layouts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.size());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(main_layout[1]);

    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title(" ScheduleFlow ")
            .title_alignment(Alignment::Center)
            .style(Style::new()
                .fg(Color::Yellow)), // TODO: Make the color adjustable
        main_layout[0],
    );

    render_calendar_month(frame, &inner_layout, time::Month::January, 0);
    render_calendar_month(frame, &inner_layout, time::Month::February, 1);
    render_calendar_month(frame, &inner_layout, time::Month::March, 2);
    render_calendar_month(frame, &inner_layout, time::Month::April, 3);
    render_calendar_month(frame, &inner_layout, time::Month::May, 4);
}

fn render_calendar_month(frame: &mut Frame, inner_layout: &Layouts, month: time::Month, idx: usize) {
    frame.render_widget(
        Monthly::new(
            Date::from_calendar_date(2023, month, 1).unwrap(),
            CalendarEventStore::default(),
        )
            .show_weekdays_header(Style::new().fg(Color::Yellow))
            .show_month_header(Style::default())
            .show_surrounding(Style::default()),
        inner_layout[idx],
    )
}
