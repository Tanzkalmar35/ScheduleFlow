use uuid::Uuid;

use super::property;

pub struct CalendarPropertyCombination {
    pub(crate) calendar_uuid: Uuid,
    pub(crate) property_uuid: Uuid,
}

impl CalendarPropertyCombination {
    pub fn new(calendar_uuid: Uuid, property_uuid: Uuid) -> Self {
        Self {
            calendar_uuid,
            property_uuid,
        }
    }
}
