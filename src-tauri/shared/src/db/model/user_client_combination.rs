use uuid::Uuid;

pub struct UserClientCombination {
    pub user_uuid: Uuid,
    pub client_uuid: Uuid,
}

impl UserClientCombination {
    pub fn new(user_uuid: Uuid, client_uuid: Uuid) -> Self {
        Self {
            user_uuid,
            client_uuid,
        }
    }
}
