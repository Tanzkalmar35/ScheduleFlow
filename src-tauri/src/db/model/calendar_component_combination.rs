use uuid::Uuid;

use super::{calendar::Calendar, component::Component};

pub struct CalendarComponentCombination {
    pub(crate) calendar_uuid: Uuid,
    pub(crate) component_uuid: Uuid,
}

impl CalendarComponentCombination {
    pub fn new(calendar_uuid: Uuid, component_uuid: Uuid) -> Self {
        Self {
            calendar_uuid,
            component_uuid,
        }
    }
}
