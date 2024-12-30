use serde::Serialize;
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize)]
pub struct Property {
    uuid: Uuid,
    key: String,
    val: String,
}

impl Property {
    pub fn new(key: String, val: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            key,
            val,
        }
    }

    pub fn from(uuid: Uuid, key: String, val: String) -> Self {
        Self { uuid, key, val }
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn set_key(&mut self, key: String) {
        self.key = key;
    }

    pub fn get_val(&self) -> &String {
        &self.val
    }

    pub fn set_val(&mut self, val: String) {
        self.val = val;
    }

    pub fn hold(key: String, val: String) -> Self {
        Self {
            uuid: Uuid::nil(),
            key,
            val,
        }
    }
}

#[derive(Debug, Type)]
pub enum OwnerType {
    CALENDAR,
    COMPONENT,
}

impl OwnerType {
    pub fn to_string(&self) -> String {
        match self {
            OwnerType::CALENDAR => String::from("Calendar"),
            OwnerType::COMPONENT => String::from("Component"),
        }
    }
}
