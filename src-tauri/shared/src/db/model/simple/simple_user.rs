use serde::Serialize;

/// SimpleUser objects represent the same data as 'normal' users, but without any access functions, so
/// SimpleUser objects can really only be used for holding data.
#[derive(Serialize, Debug, Clone)]
pub struct SimpleUser {
    username: String,
    email: String,
}

impl SimpleUser {
    /// Creates a new SimpleUser object for simple data representation.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user.
    /// * `password` - The password of the user.
    /// * `email` - The email of the user.
    ///
    /// # Examples
    ///
    /// ```
    /// let simple_user = SimpleUser::new(user.get_username(), user.email());
    /// ```
    pub fn new(username: String, email: String) -> Self {
        Self { username, email }
    }

    /// Returns the username of this SimpleUser object.
    ///
    /// # Examples
    ///
    /// ```
    /// let simple_user = SimpleUser::new(...);
    /// println!("Username: {}", simple_user.get_username());
    /// ```
    pub fn get_username(&self) -> &String {
        return &self.username;
    }

    /// Returns the email of this SimpleUser object.
    ///
    /// # Examples
    ///
    /// ```
    /// let simple_user = SimpleUser::new(...);
    /// println!("Email: {}", simple_user.get_email());
    /// ```
    pub fn get_email(&self) -> &String {
        return &self.email;
    }
}
