use sqlx::Execute;
use uuid::Uuid;

use crate::{
    db::{
        db_actions::DbActions,
        model::{
            calendar::Calendar,
            component::{Component, ComponentType},
            property::{OwnerType, Property},
        },
        pg_driver::PgDriver,
        repository::component_repository::ComponentRepository,
    },
    errors::error_impl::database_operation_failed_error::DatabaseOperationFailedError,
    runtime_objects::get_error_queue,
};

#[derive(Debug, PartialEq)]
pub(crate) struct SimpleComponent {
    c_type: ComponentType,
    properties: Vec<Property>,
}

impl SimpleComponent {
    pub(crate) fn new(c_type: ComponentType, properties: Vec<Property>) -> Self {
        Self { c_type, properties }
    }

    pub(crate) fn empty() -> Self {
        Self {
            c_type: ComponentType::OTHER,
            properties: vec![],
        }
    }

    pub(crate) fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    pub(crate) fn build_by_calendar(driver: &mut PgDriver, calendar: &Calendar) -> Vec<Self> {
        let mut simple_components: Vec<Self> = vec![];
        let stmt = format!(
            r#"
            select c.uuid, c.c_type, p.key, p.value 
            from components c 
            inner join properties p 
            on c.uuid = p.owner_uuid 
            where c.calendar_uuid = '{}' 
            and p.owner_type = '{}'
        "#,
            calendar.uuid,
            OwnerType::COMPONENT.to_string()
        );

        let res = driver.exec(&stmt);

        if let Err(e) = res {
            println!("{}", e);
            let err = DatabaseOperationFailedError::new();
            get_error_queue().enqueue(err);
            return vec![];
        }

        let mut component_uuid_before: String = String::default();
        let mut current_component: SimpleComponent = SimpleComponent::empty();

        for row in &res.unwrap() {
            let component_uuid: Uuid = row.get("uuid");
            let property_key: String = row.get("key");
            let property_val: String = row.get("value");

            if component_uuid_before.eq(&component_uuid.to_string()) {
                // same component as before, so we just add the new property
                current_component
                    .properties
                    .push(Property::hold(property_key, property_val))
            } else {
                // new component, add the old one to the result
                if current_component != SimpleComponent::empty() {
                    simple_components.push(current_component);
                }

                let c_type: String = row.get("c_type");

                let first_property = vec![Property::hold(property_key, property_val)];
                current_component =
                    SimpleComponent::new(ComponentType::parse(&c_type), first_property);
                component_uuid_before = component_uuid.to_string();
            }
        }

        simple_components
    }
}
