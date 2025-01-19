use chrono::{DateTime, Utc};
use ed25519_dalek::VerifyingKey;
use uuid::Uuid;

pub struct Client {
    uuid: Uuid,
    user_uuid: Uuid,
    pub_key: VerifyingKey,
    device_name: String,
    last_used: DateTime<Utc>,
    registered_at: DateTime<Utc>,
}

impl Client {
    pub fn new(name: String, user: Uuid, key: VerifyingKey) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            user_uuid: user,
            pub_key: key,
            device_name: name,
            last_used: Utc::now(),
            registered_at: Utc::now(),
        }
    }

    pub fn from(
        uuid: Uuid,
        user_uuid: Uuid,
        pub_key: VerifyingKey,
        device_name: String,
        last_used: DateTime<Utc>,
        registered_at: DateTime<Utc>,
    ) -> Self {
        Self {
            uuid,
            user_uuid,
            pub_key,
            device_name,
            last_used,
            registered_at,
        }
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn get_user_uuid(&self) -> Uuid {
        self.user_uuid
    }

    pub fn get_pub_key(&self) -> &VerifyingKey {
        &self.pub_key
    }

    pub fn get_device_name(&self) -> &str {
        &self.device_name
    }

    pub fn get_last_used(&self) -> DateTime<Utc> {
        self.last_used
    }

    pub fn get_registered_at(&self) -> DateTime<Utc> {
        self.registered_at
    }
}
