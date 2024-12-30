use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    uuid: Uuid,
    username: String,
    password: String,
    email: String,
}

impl User {
    /// Creates a new user and prepares the raw values into values ready to be stored.
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            username,
            password,
            email,
        }
    }

    /// Creates a User object from values that already exist in the db.
    pub fn from(uuid: Uuid, username: String, email: String, password: String) -> Self {
        Self {
            uuid,
            username,
            password,
            email,
        }
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn get_password(&self) -> &String {
        &self.password
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }
}
