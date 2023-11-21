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

    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title(" ScheduleFlow ")
            .title_alignment(Alignment::Center)
            .style(Style::new()
                .fg(Con)), // TODO: Make the color adjustable
        main_layout[0],
    );

    let cal_cols = create_tui_cols(&main_layout, 1);

    render_calendar(frame, &cal_cols);
}

/// Renders the full and final calendar widget.
///
/// ## Arguments
///
/// frame - The ratatui frame to render the widget on.
///
/// parent_layout - The layout to render the widget on.
fn render_calendar(frame: &mut Frame, parent_layout: &Layouts) {
    let cal_row_1 = create_tui_row(&parent_layout, 0);
    let cal_row_2 = create_tui_row(&parent_layout, 1);
    let cal_row_3 = create_tui_row(&parent_layout, 2);

    render_calendar_month(frame, &cal_row_1, time::Month::January, 0);
    render_calendar_month(frame, &cal_row_1, time::Month::February, 1);
    render_calendar_month(frame, &cal_row_1, time::Month::March, 2);
    render_calendar_month(frame, &cal_row_1, time::Month::April, 3);

    render_calendar_month(frame, &cal_row_2, time::Month::May, 0);
    render_calendar_month(frame, &cal_row_2, time::Month::June, 1);
    render_calendar_month(frame, &cal_row_2, time::Month::July, 2);
    render_calendar_month(frame, &cal_row_2, time::Month::August, 3);

    render_calendar_month(frame, &cal_row_3, time::Month::September, 0);
    render_calendar_month(frame, &cal_row_3, time::Month::October, 1);
    render_calendar_month(frame, &cal_row_3, time::Month::November, 2);
    render_calendar_month(frame, &cal_row_3, time::Month::December, 3);
}

/// Creates a tui row with 4 columns and attaches it to a layout.
///
/// ## Arguments
///
/// parent_layout - The layout to render the widget on.
///
/// idx - The index of the layout to render the widget on.
fn create_tui_row(parent_layout: &Layouts, idx: usize) -> Layouts {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(parent_layout[idx])
}

/// Creates a tui column with 3 rows and attaches it to a layout.
///
/// ## Arguments
///
/// parent_layout - The layout to render the widget on.
///
/// idx - The index of the layout to render the widget on
fn create_tui_cols(parent_layout: &Layouts, idx: usize) -> Layouts {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(parent_layout[idx])
}

/// Renders a calendar month tui widget.
///
/// ## Arguments
///
/// frame - The ratatui frame to render the widget on.
///
/// parent_layout - The layout to render the widget on.
///
/// month - The month to render.
///
/// idx - The index of the layout to render the widget on.
fn render_calendar_month(frame: &mut Frame, parent_layout: &Layouts, month: time::Month, idx: usize) {
    frame.render_widget(
        Monthly::new(
            Date::from_calendar_date(2023, month, 1).unwrap(),
            CalendarEventStore::default(),
        )
            .show_weekdays_header(Style::new().fg(Color::Yellow))
            .show_month_header(Style::default())
            .show_surrounding(Style::default()),
        parent_layout[idx],
    )
}
