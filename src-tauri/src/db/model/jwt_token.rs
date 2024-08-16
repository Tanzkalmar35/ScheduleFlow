use serde::Serialize;
use uuid::Uuid;

use super::model::Model;

#[derive(Serialize, Clone, PartialEq)]
pub(crate) struct JwtToken {
    pub(crate) token: String,
    pub(crate) user_uuid: Uuid,
}

impl JwtToken {
    pub fn new(token: String, user_uuid: Uuid) -> Self {
        Self { token, user_uuid }
    }

    pub fn empty() -> Self {
        Self {
            token: String::new(),
            user_uuid: Uuid::default(),
        }
    }
}

impl Model for JwtToken {}
