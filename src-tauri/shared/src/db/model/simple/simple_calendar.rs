use crate::db::{
    db_actions::DbActions,
    model::{
        calendar::Calendar,
        property::{OwnerType, Property},
        simple::simple_component::SimpleComponent,
    },
    repository::{
        property_repository::PropertyRepository,
        user_calendar_combination_repository::UserCalendarCombinationRepository,
    },
};
use pg_driver::PgDriver;
use serde::Serialize;

use super::simple_user::SimpleUser;

#[derive(Serialize, Debug, Clone)]
pub struct SimpleCalendar {
    name: String,
    components: Vec<SimpleComponent>,
    properties: Vec<Property>,
    users: Vec<SimpleUser>,
}

impl SimpleCalendar {
    /// Creates a new SimpleCalendar object from its dependencies.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the calendar to convert to.
    /// * `components` - The component dependencies of the calendar.
    /// * `properties` - The properties defining the calendar.
    /// * `users` - A list of users that have access to this calendar.
    ///
    /// # Examples
    ///
    /// ```
    /// let simple_calendar = SimpleCalendar::new(
    ///     calendar.get_name(),
    ///     simple_components,
    ///     calenar.get_properties(),
    ///     simple_users_with_access
    /// );
    /// ```
    pub fn new(
        name: String,
        components: Vec<SimpleComponent>,
        properties: Vec<Property>,
        users: Vec<SimpleUser>,
    ) -> Self {
        Self {
            name,
            components,
            properties,
            users,
        }
    }

    /// Creates a new SimpleCalendar representing an model::Calendar.
    /// Assembless all dependencies of that model::Calendar to one data holder object.
    ///
    /// # Arguments
    ///
    /// * `driver` - The driver to user for database access ops.
    /// * `calendar` - The calendar to map.
    ///
    /// # Examples
    ///
    /// ```
    /// let simple_calendar = SimpleCalendar::build(
    ///     driver().lock().unwrap().deref_mut(),
    ///     calendar
    /// );
    /// println!("Converted calendar: {}", simple_calendar);
    /// ```
    pub fn build(driver: &mut PgDriver, calendar: Calendar) -> Self {
        let owned_by_calendar = format!(
            "owner_type = '{}' and owner_uuid = '{}'",
            OwnerType::CALENDAR.to_string(),
            calendar.uuid
        );

        let components = SimpleComponent::build_by_calendar(driver, &calendar);
        let properties = PropertyRepository::retrieve(driver, Some(owned_by_calendar));
        let users = UserCalendarCombinationRepository::get_users_of_calendar(driver, calendar.uuid);
        let mut simple_users = vec![];

        // Convert users to SimpleUsers
        for user in users {
            simple_users.push(SimpleUser::new(
                user.get_username().to_string(),
                user.get_email().to_string(),
            ));
        }

        Self::new(calendar.name, components, properties, simple_users)
    }
}
