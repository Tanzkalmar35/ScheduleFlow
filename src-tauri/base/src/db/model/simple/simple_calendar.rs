use serde::Serialize;
use pg_driver::PgDriver;
use crate::db::{
    db_actions::DbActions,
    model::{
        calendar::Calendar,
        property::{OwnerType, Property},
        simple::simple_component::SimpleComponent,
    },
    repository::property_repository::PropertyRepository,
};

#[derive(Serialize, Debug)]
pub struct SimpleCalendar {
    name: String,
    components: Vec<SimpleComponent>,
    properties: Vec<Property>,
}

impl SimpleCalendar {
    pub(crate) fn new(
        name: String,
        components: Vec<SimpleComponent>,
        properties: Vec<Property>,
    ) -> Self {
        Self {
            name,
            components,
            properties,
        }
    }

    /**
     * Creates a new SimpleCalendar representing an model::Calendar.
     * Assembless all dependencies of that model::Calendar to one data holder object.
     */
    pub(crate) fn build(driver: &mut PgDriver, calendar: Calendar) -> Self {
        let owned_by_calendar = format!(
            "owner_type = '{}' and owner_uuid = '{}'",
            OwnerType::CALENDAR.to_string(),
            calendar.uuid
        );

        let components = SimpleComponent::build_by_calendar(driver, &calendar);
        let properties = PropertyRepository::retrieve(driver, Some(owned_by_calendar));

        Self::new(calendar.name, components, properties)
    }
}
