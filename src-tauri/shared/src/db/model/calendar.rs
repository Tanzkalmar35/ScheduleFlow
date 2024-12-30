use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Calendar {
    pub uuid: Uuid,
    pub name: String,
}

impl Calendar {
    pub fn new(name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
        }
    }

    pub fn from(uuid: Uuid, name: String) -> Self {
        Self { uuid, name }
    }
}
