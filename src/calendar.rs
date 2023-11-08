use icalendar::CalendarDateTime;

struct Event {
    title: String,
    starting_time: Option<CalendarDateTime>,
    ending_time: Option<CalendarDateTime>,
    date: Option<CalendarDateTime>
}

impl Event {
    /// Creates a new event
    fn new(title: String, starting_time: CalendarDateTime, ending_time: CalendarDateTime, date: CalendarDateTime) -> Event {
        Event {
            title,
            starting_time: Some(starting_time),
            ending_time: Some(ending_time),
            date: Some(date)
        }
    }
}
