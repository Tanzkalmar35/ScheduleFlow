use std::path::Component;

use crate::db::model::{calendar::Calendar, property::Property};

pub struct SimpleCalendar {
    components: Vec<Component>,
    properties: Vec<Property>,
}

impl SimpleCalendar {
    /**
     *
     */
    pub(crate) fn build(calendar: Calendar) {}
}
