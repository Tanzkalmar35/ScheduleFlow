use icalendar::{Calendar, CalendarComponent, Component, Event, Property, Todo, Venue};
use uuid::Uuid;

use crate::db::db_actions::{DbActions, Table};
use crate::db::pg_driver::PgDriver;
use crate::db::tables::table_calendars::CalendarDAO;
use crate::db::tables::table_combinations::TableCombination;
use crate::db::tables::table_components::{ComponentDAO, ComponentType};
use crate::db::tables::table_properties::PropertyDAO;

/// Acts like an adapter between the icalendar crate and my DAO objects
///
/// Contains all the logic for converting from one to another.
pub(crate) struct ICalendarAdapter;

/// Converts data from the base class to the entity class.
impl ICalendarAdapter {
    /// Bundles a CalendarDAO with all its Components and Properties into one icalendar::Calendar object.
    pub fn bundle_calendar(driver: &mut PgDriver, from: CalendarDAO) -> Calendar {
        let mut cal = Self::build_calendar(driver, &from);
        let components = Self::build_components(driver, from.uuid);

        for component in components {
            cal.push(component);
        }

        cal
    }

    /// Builds an icalendar::Calendar out of the CalendarDAO and appends all its properties.
    fn build_calendar(driver: &mut PgDriver, cal: &CalendarDAO) -> Calendar {
        let mut res = Calendar::new();
        let properties;

        let properties_uuids = Self::get_properties_of::<CalendarDAO>(driver, cal.uuid);
        let uuid = properties_uuids.iter().map(|prop| prop.uuid2).collect();
        properties = Self::get_properties(driver, uuid);

        for property in properties {
            let prop = Property::new(property.key.as_str(), property.val.as_str());
            res.append_property(prop);
        }

        res
    }

    /// Retrieves the property entries of a list of property uuids.
    fn get_properties(driver: &mut PgDriver, properties_uuids: Vec<Uuid>) -> Vec<PropertyDAO> {
        let mut res: Vec<PropertyDAO> = Vec::new();
        for property in properties_uuids {
            res.push(PropertyDAO::retrieve_single(
                driver,
                Some(format!("uuid = '{}'", property)),
            ));
        }
        res
    }

    /// Builds the components of a calendar and also appends all the properties of the components.
    fn build_components(driver: &mut PgDriver, cal_uuid: Uuid) -> Vec<CalendarComponent> {
        let mut res: Vec<CalendarComponent> = Vec::new();
        let condition = format!("calendar_uuid = '{}'", cal_uuid);
        let query_res: Vec<TableCombination<CalendarDAO, ComponentDAO>> =
            TableCombination::retrieve(driver, Some(condition));

        let mut components: Vec<ComponentDAO> = vec![];

        for combination in query_res {
            let condition = format!("uuid = '{}'", combination.uuid2);
            components.push(ComponentDAO::retrieve_single(driver, Some(condition)));
        }

        for mut component in components {
            let mut properties: Vec<PropertyDAO> = vec![];
            let properties_uuids = Self::get_properties_of::<ComponentDAO>(driver, component.uuid);

            for property in properties_uuids {
                let property_uuid = property.uuid2;
                properties.push(PropertyDAO::retrieve_single(
                    driver,
                    Some(format!("uuid = '{}'", property_uuid)),
                ));
            }

            res.push(Self::build_component_from_props(
                &mut component,
                &mut properties,
            ));
        }

        res
    }

    /// Creates an icalendar::Component (Event, ...) out of the components and adds the properties.
    fn build_component_from_props(
        component: &mut ComponentDAO,
        properties: &mut Vec<PropertyDAO>,
    ) -> CalendarComponent {
        match component.c_type {
            ComponentType::EVENT => {
                let mut event = Event::new();
                for property in properties {
                    event.add_property(property.key.as_str(), property.val.as_str());
                }
                CalendarComponent::Event(event)
            }
            ComponentType::TODO => {
                let mut todo = Todo::new();
                for property in properties {
                    todo.add_property(property.key.as_str(), property.val.as_str());
                }
                CalendarComponent::Todo(todo)
            }
            ComponentType::VENUE => {
                let mut venue = Venue::new();
                for property in properties {
                    venue.add_property(property.key.as_str(), property.val.as_str());
                }
                CalendarComponent::Venue(venue)
            }
            _ => unimplemented!("TODO"),
        }
    }

    /// Retrieves the property of an object that has a TableCombination with properties, for example
    /// the table components_properties.
    fn get_properties_of<T: Table>(
        driver: &mut PgDriver,
        of: Uuid,
    ) -> Vec<TableCombination<T, PropertyDAO>> {
        TableCombination::<T, PropertyDAO>::retrieve(
            driver,
            Some(format!("{} = '{}'", T::get_fk_uuid_name(), of)),
        )
    }
}

#[cfg(test)]
mod tests {
    todo!("Test adapter");
}

//pub struct CalendarDaoAdapter;

//impl CalendarDaoAdapter {
//
//    /// Builds a CalendarDAO object from a Calendar object.
//    pub fn bundle_calendar_dao(driver: &mut PgDriver, from: Calendar) -> CalendarDAO {
//        let mut cal = Self::build_calendar_dao(driver, &from);
//        let components = Self::build_components_dao(driver, from);
//
//        for component in components {
//            cal.push(component);
//        }
//
//        cal
//    }
//
//    pub fn build_calendar_dao(driver: &mut PgDriver, cal: &Calendar) -> CalendarDAO {
//        let mut res = CalendarDAO::new();
//        let properties = cal.properties();
//
//        for property in properties {
//            let prop = PropertyDAO::new(
//                property.name().to_string(),
//                property.value().to_string(),
//                res.uuid,
//            );
//            prop.store(driver);
//        }
//
//        res
//    }
//}
