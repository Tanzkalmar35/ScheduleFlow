use time::Date;

use ratatui::Frame;
use ratatui::layout::{Layout, Direction, Constraint, Alignment};
use ratatui::style::{Style, Color};
use ratatui::widgets::calendar::{Monthly, CalendarEventStore};
use ratatui::widgets::{Block, Borders};

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

    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title(" ScheduleFlow ")
            .title_alignment(Alignment::Center)
            .style(Style::new()
            .fg(Color::Yellow)), // TODO: Make the coor adjustable
        main_layout[0],
    );
    render_calendar_month(frame, &main_layout, time::Month::April, 1);
    render_calendar_month(frame, &main_layout, time::Month::May, 1);
    render_calendar_month(frame, &main_layout, time::Month::June, 1);
}

fn render_calendar_month(frame: &mut Frame, main_layout: &Layouts, month: time::Month, idx: usize) {
    frame.render_widget(
        Monthly::new(
            Date::from_calendar_date(2023, month, 1).unwrap(),
            CalendarEventStore::default(),
        )
        .show_weekdays_header(Style::default())
        .show_month_header(Style::default())
        .show_surrounding(Style::default()),
        main_layout[idx]
    )
}