use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Clone, PartialEq)]
pub struct JwtToken {
    pub token: String,
    pub user_uuid: Uuid,
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
