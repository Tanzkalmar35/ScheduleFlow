use skytable::query;

use types::User;

use crate::db::access::{connect, insert};

#[path = "types.rs"]
mod types;

pub fn create_user(username: String, password: String) -> User {
    let user = User {
        userId: 1,
        username,
        password,
    };
    let mut conn = connect().unwrap();
    //setup(&mut conn);
    insert(&mut conn, query!("insert into example.users(?, ?, ?)", 5u64, &user.username, &user.password));
    user
}

#[test]
pub fn test() {
    create_user("test".into(), "test".into());
}
