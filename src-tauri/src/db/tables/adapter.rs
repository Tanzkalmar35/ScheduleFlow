use icalendar::{Component, Event, Todo};
use icalendar::CalendarComponent::Venue;
use uuid::Uuid;

use crate::db_actions::{DbActions, Table};
use crate::pg_driver::PgDriver;
use crate::table_calendars::CalendarDAO;
use crate::table_combinations::TableCombination;
use crate::table_components::{ComponentDAO, ComponentType};
use crate::table_properties::PropertyDAO;

/// Acts like an adapter between the icalendar crate and my DAO objects
///
/// Contains all the logic for converting from one to another.
struct ICalendarAdapter;

/// Converts data from the base class to the entity class.
impl ICalendarAdapter {
    ///
    pub fn build_calendar(driver: &mut PgDriver, from: CalendarDAO) {
        let components = Self::build_components(driver, from.uuid);
    }

    fn build_calendar_properties(driver: &mut PgDriver, mut cal: CalendarDAO) -> CalendarDAO {
        let mut properties;
        let properties_uuids = Self::get_properties_from(driver, cal.uuid);
        let uuid = properties_uuids.iter().map(|prop| {return prop.uuid2}).collect::<Vec<Uuid>>();
        Self::get_properties(driver, uuid);
        cal
    }

    fn get_properties(driver: &mut PgDriver, properties_uuids: Vec<Uuid>) -> Vec<PropertyDAO> {
        let mut res: Vec<PropertyDAO> = Vec::new();
        for property in properties_uuids {
            res.push(PropertyDAO::retrieve_single(
                driver,
                vec!["key".to_string(), "value".to_string()],
                Some(format!("uuid = '{}'", property)),
            ));
        }
        res
    }

    fn build_components(driver: &mut PgDriver, cal_uuid: Uuid) -> Vec<dyn Component> {
        let mut res: Vec<dyn Component> = Vec::new();
        let condition = format!("calendar_uuid = '{}'", cal_uuid);
        let query_res = ComponentDAO::retrieve(driver, vec!["*".to_string()], Some(condition));

        for mut component in query_res {
            let mut properties: Vec<PropertyDAO> = vec![];
            let properties_uuids = Self::get_properties_from(driver, component.uuid);

            for property in properties_uuids {
                let property_uuid = property.uuid2;
                properties.push(
                    PropertyDAO::retrieve_single(
                        driver,
                        vec!["key".to_string(), "value".to_string()],
                        Some(format!("uuid = '{}'", property_uuid)),
                    )
                );
            }

            res.push(Self::build_component_from_props(&mut component, &mut properties));
        }

        res
    }

    fn build_component_from_props(component: &mut ComponentDAO, properties: &mut Vec<PropertyDAO>) -> Vec<dyn Component> {
        let mut res: Vec<dyn Component> = Vec::new();

        match component.c_type {
            ComponentType::EVENT => {
                let mut event = Event::new();
                for property in properties {
                    event.add_property(property.key.as_str(), property.val.as_str());
                }
                res.push(event);
            }
            ComponentType::TODO => {
                let mut todo = Todo::new();
                for property in properties {
                    todo.add_property(property.key.as_str(), property.val.as_str());
                }
                res.push(todo);
            }
            ComponentType::VENUE => {
                let mut venue = Venue::new();
                for property in properties {
                    venue.add_property(property.key.as_str(), property.val.as_str());
                }
                res.push(venue);
            }
            _ => unimplemented!("TODO")
        }

        res
    }

    fn get_properties_from<T: Table>(driver: &mut PgDriver, of: Uuid) -> Vec<TableCombination<T, PropertyDAO>> {
        TableCombination::<T, PropertyDAO>::retrieve(
            driver,
            vec!["property_uuid".to_string()],
            Some(format!("component_uuid = '{}'", of)),
        )
    }
}
