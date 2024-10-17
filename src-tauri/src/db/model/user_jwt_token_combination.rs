use uuid::Uuid;

pub struct UserJwtTokenCombination {
    pub(crate) user_uuid: Uuid,
    pub(crate) jwt_token_uuid: Uuid,
}

impl UserJwtTokenCombination {
    pub fn new(user_uuid: Uuid, jwt_token_uuid: Uuid) -> Self {
        Self {
            user_uuid,
            jwt_token_uuid,
        }
    }
}
