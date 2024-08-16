use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::model::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct Calendar {
    pub(crate) uuid: Uuid,
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }

    pub fn from(uuid: Uuid) -> Self {
        Self { uuid }
    }
}

impl Model for Calendar {}
