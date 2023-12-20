// @generated automatically by Diesel CLI.

diesel::table! {
    calendars (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
    }
}
