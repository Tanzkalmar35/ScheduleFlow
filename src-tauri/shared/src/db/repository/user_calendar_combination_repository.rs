use crate::{
    current::get_error_queue,
    db::{
        db_actions::{DbActions, Table},
        model::{
            calendar::Calendar, user::User, user_calendar_combination::UserCalendarCombination,
        },
        repository::{calendar_repository::CalendarRepository, user_repository::UserRepository},
    },
    errors::{
        error_impl::database_operation_failed_error::DatabaseOperationFailedError,
        error_utils::Error,
    },
};
use pg_driver::PgDriver;
use postgres::Row;
use uuid::Uuid;

pub struct UserCalendarCombinationRepository;

impl UserCalendarCombinationRepository {
    /// Retrieves all calendars the given user has access to.
    ///
    /// # Arguments
    ///
    /// * `driver` - The database driver.
    /// * `user` - The user to query.
    ///
    /// # Examples
    ///
    /// ```
    /// let calendars_of_user = get_calendars_of_user(driver().lock().unwrap().deref_mut(), user);
    /// ```
    pub fn get_calendars_of_user(driver: &mut PgDriver, user: &User) -> Vec<Calendar> {
        let mut res: Vec<Calendar> = vec![];
        let stmt = format!(
            r#"
            select c.uuid, c.name from users_calendars uc 
            INNER JOIN calendars c 
            ON uc.calendar_uuid = c.uuid 
            where uc.user_uuid = '{}'
            "#,
            user.get_uuid()
        );
        let query_res = Self::query(driver, stmt);

        if let Err(e) = query_res {
            let mut err = DatabaseOperationFailedError::new();
            err.set_message(format!("Could not retrieve calendars of user: {}", e));
            get_error_queue().enqueue(err);
            return vec![];
        }

        for calendar_row in query_res.unwrap() {
            res.push(Calendar::from(
                calendar_row.get("uuid"),
                calendar_row.get("name"),
            ));
        }

        res
    }

    /// Retrieves all users that have access to the calendar matchig the given uuid.
    ///
    /// # Arguments
    ///
    /// * `driver` - The database driver.
    /// * `calendar_uuid` - The uuid of the calendar to query.
    ///
    /// # Examples
    ///
    /// ```
    /// let calenar_users = get_users_of_calendar(driver().lock().unwrap().deref_mut(), calendar_uuid);
    /// ```
    pub fn get_users_of_calendar(driver: &mut PgDriver, calendar_uuid: Uuid) -> Vec<User> {
        let mut res: Vec<User> = vec![];
        let stmt = format!(
            r#"
            SELECT u.uuid, u.email, u.password, u.username 
            FROM users_calendars uc
            INNER JOIN users u
            ON uc.user_uuid = u.uuid
            WHERE uc.calendar_uuid = '{}'
            "#,
            calendar_uuid
        );

        let query_res = Self::query(driver, stmt);
        if let Err(e) = query_res {
            let mut err = DatabaseOperationFailedError::new();
            err.set_message(format!(
                "Could not retrieve users of calendar uuid={} because of: {}",
                calendar_uuid, e
            ));
            get_error_queue().enqueue(err);
            return vec![];
        }

        for user_row in query_res.unwrap() {
            res.push(User::from(
                user_row.get("uuid"),
                user_row.get("username"),
                user_row.get("email"),
                user_row.get("password"),
            ))
        }

        res
    }
}

impl Table<UserCalendarCombination> for UserCalendarCombinationRepository {
    fn get_name() -> String {
        format!(
            "{}_{}",
            UserRepository::get_name(),
            CalendarRepository::get_name()
        )
    }

    fn get_fk_uuid_name() -> String {
        unimplemented!()
    }

    fn get_fmt_cols() -> String {
        format!(
            "{}, {}",
            UserRepository::get_fk_uuid_name(),
            CalendarRepository::get_fk_uuid_name()
        )
    }

    fn get_fmt_cols_no_id() -> String {
        unimplemented!()
    }

    fn get_fmt_vals(model: &UserCalendarCombination) -> String {
        format!("'{}', '{}'", model.user_uuid, model.calendar_uuid)
    }

    fn get_fmt_vals_no_id(_model: &UserCalendarCombination) -> String {
        unimplemented!()
    }
}

impl DbActions<UserCalendarCombination, Self> for UserCalendarCombinationRepository {
    fn store(driver: &mut PgDriver, model: &UserCalendarCombination) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    fn update(_driver: &mut PgDriver, _model: &UserCalendarCombination) -> anyhow::Result<()> {
        unimplemented!(
            "You can't update a combination entry, as it does only consist of two uuids."
        )
    }

    fn remove(driver: &mut PgDriver, model: &UserCalendarCombination) -> anyhow::Result<()> {
        let col_name: String = UserRepository::get_fk_uuid_name();
        let col_value: String = model.user_uuid.to_string();
        Self::delete_spec_col(driver, col_name, col_value)
    }

    fn retrieve(driver: &mut PgDriver, condition: Option<String>) -> Vec<UserCalendarCombination> {
        let mut res: Vec<UserCalendarCombination> = vec![];

        let rows: Vec<Row> = Self::read(driver, Self::get_name().as_str(), condition);

        for row in rows {
            let user_uuid: String = row.get(UserRepository::get_fk_uuid_name().as_str());
            let calendar_uuid: String = row.get(CalendarRepository::get_fk_uuid_name().as_str());
            res.push(UserCalendarCombination::new(
                user_uuid.parse().unwrap(),
                calendar_uuid.parse().unwrap(),
            ))
        }

        res
    }
}
