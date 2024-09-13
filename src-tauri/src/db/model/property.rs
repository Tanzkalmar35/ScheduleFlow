use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct Property {
    uuid: Uuid,
    key: String,
    val: String,
}

impl Property {
    pub(crate) fn new(key: String, val: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            key,
            val,
        }
    }

    pub(crate) fn from(uuid: Uuid, key: String, val: String) -> Self {
        Self { uuid, key, val }
    }

    pub(crate) fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub(crate) fn get_key(&self) -> &String {
        &self.key
    }

    pub(crate) fn set_key(&mut self, key: String) {
        self.key = key;
    }

    pub(crate) fn get_val(&self) -> &String {
        &self.val
    }

    pub(crate) fn set_val(&mut self, val: String) {
        self.val = val;
    }

    pub(crate) fn hold(key: String, val: String) -> Self {
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
            _ => String::default(),
        }
    }
}
