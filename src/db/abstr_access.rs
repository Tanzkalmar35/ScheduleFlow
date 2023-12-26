use types::User;

use crate::db::access::{connect, insert};

#[path = "types.rs"]
mod types;

pub fn create_user(username: String, password: String) -> User {
    let user = User {
        user_id: 1,
        username,
        password,
    };
    let conn = connect().unwrap();
    insert(conn, "users".to_string(), vec![user.user_id.to_string(), user.username, user.password]);
    user
}

#[test]
pub fn test() {
    create_user("testUser".to_string(), "testPassword".to_string());
}
