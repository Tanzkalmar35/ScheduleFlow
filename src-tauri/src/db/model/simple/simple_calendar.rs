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

use super::simple_component::SimpleComponent;

pub struct SimpleCalendar {
    components: Vec<SimpleComponent>,
    properties: Vec<Property>,
}

impl SimpleCalendar {
    /**
     *
     */
    pub(crate) fn build(driver: &mut PgDriver, calendar: Calendar) -> Self {
        let calendar_uuid_matches = format!("calendar_uuid = {}", calendar.uuid);
        let owned_by_calendar = format!(
            "owner_type = {:?} and owner_uuid = {}",
            OwnerType::CALENDAR,
            calendar.uuid
        );

        let components = SimpleComponent::build_by_calendar(driver, &calendar);

        todo!()
    }
}
