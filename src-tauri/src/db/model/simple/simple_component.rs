use sqlx::Execute;

use crate::db::{
    db_actions::DbActions,
    model::{
        calendar::Calendar,
        component::{Component, ComponentType},
        property::{OwnerType, Property},
    },
    pg_driver::PgDriver,
    repository::component_repository::ComponentRepository,
};

pub(crate) struct SimpleComponent {
    c_type: ComponentType,
    properties: Vec<Property>,
}

impl SimpleComponent {
    pub(crate) fn new(c_type: ComponentType, properties: Vec<Property>) {
        Self { c_type, properties }
    }

    pub(crate) fn build_by_calendar(driver: &mut PgDriver, calendar: &Calendar) -> Vec<Self> {
        let mut properties: Vec<Property>;
        let mut simple_components: Vec<Self>;
        let stmt = sqlx::query(
            r#"
            select c.c_type, p.key, p.value 
            from components c 
            inner join properties p 
            on c.uuid = p.owner_uuid 
            where c.calendar_uuid = $1 
            and p.owner_type = $2
        "#,
        )
        .bind(calendar.uuid)
        .bind(OwnerType::Component)
        .sql();

        let res = driver.exec(stmt);

        if let Err(e) = res {
            println!("{}", e)
        }

        for row in res.unwrap() {
            let c_type = row.get("c_type");
            let property_key = row.get("key");
            let property_val = row.get("value");

            properties.push(Property::new(property_key, property_val));
            simple_components.push(Self::new(c_type, properties));
        }
    }
}
