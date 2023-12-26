use skytable::{Query, Response};

#[derive(Query, Response)]
pub(crate) struct User {
    pub(crate) user_id: i32,
    pub(crate) username: String,
    pub(crate) password: String,
}
