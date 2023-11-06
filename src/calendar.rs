use icalendar::CalendarDateTime;

struct Event {
    title: String,
    starting_time: Option<CalendarDateTime>,
    ending_time: Option<CalendarDateTime>,
    date: Option<CalendarDateTime>
}

impl Event {
    fn new(title: String, starting_time: CalendarDateTime, ending_time: CalendarDateTime, date: CalendarDateTime) {

    }
}