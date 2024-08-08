use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub(crate) uuid: Uuid,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) email: String,
}

impl User {
    /// Creates a new user and prepares the raw values into values ready to be stored.
    pub(crate) fn new(username: String, email: String, password: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            username,
            password,
            email,
        }
    }

    /// Creates a User object from values that already exist in the db.
    pub(crate) fn from(uuid: Uuid, username: String, email: String, password: String) -> Self {
        Self {
            uuid,
            username,
            password,
            email,
        }
    }
}
