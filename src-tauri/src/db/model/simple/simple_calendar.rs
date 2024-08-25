use std::path::Component;

use crate::db::{
    db_actions::DbActions,
    model::{
        calendar::Calendar,
        property::{OwnerType, Property},
    },
    pg_driver::PgDriver,
    repository::component_repository::ComponentRepository,
};

pub struct SimpleCalendar {
    components: Vec<Component>,
    properties: Vec<Property>,
}

impl SimpleCalendar {
    /**
     *
     */
    pub(crate) fn build(driver: &mut PgDriver, calendar: Calendar) -> Self {
        let mut components;
        let mut properties;
        let calendar_uuid_matches = format!("calendar_uuid = {}", calendar.uuid);
        let owned_by_calendar = format!(
            "owner_type = {} and owner_uuid = {}",
            OwnerType::Calendar,
            calendar.uuid
        );

        components = ComponentRepository::retrieve(driver, Some(calendar_uuid_matches))
    }
}
