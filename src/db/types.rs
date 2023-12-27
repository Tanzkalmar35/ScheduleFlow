use skytable::{Query, Response};

#[derive(Query, Response, Clone, Debug)]
#[derive(PartialEq)]
pub struct User {
    pub(crate) userId: i32,
    pub(crate) username: String,
    pub(crate) password: String,
}
