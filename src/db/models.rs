use diesel::{AsChangeset, Insertable, Queryable};
use crate::schema::calendars;

#[derive(Insertable)]
#[table_name = "calendars"]
pub struct NewCalendar<'a> {
    pub title: &'a str
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Calendar {
    pub id: i32,
    pub title: String
}