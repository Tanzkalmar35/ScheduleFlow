use crate::errors::error_messages::NO_DB_CONNECTION_ERR;
use crate::errors::error_utils::{Error, ErrorCode, ErrorHandler};
use crate::runtime_objects::get_app_handle;
use std::time::Duration;

/// Indicates that the database connection could not be established.
pub struct NoDatabaseConnectionError {
    error_code: u32,
    message: String,
    timeout: Duration,
    condition: Option<Box<dyn Fn() -> bool + Send>>,
    handler: Box<dyn Fn() + Send>,
}

impl NoDatabaseConnectionError {
    /// Initializes a new NoDatabaseConnectionError with its default params.
    ///
    /// # Default params
    /// * `error_code` - 1, database related errors.
    /// * `message` - error_messages::NO_DB_CONNECTION_ERR.
    /// * `timeout` - 0 seconds, no default delay before population.
    /// * `condition` - get_current_window().is_some(), the current window needs to be set.
    /// * `handler` - ErrorHandler::populate_toast(), populates a toast in the frontend indicating the error.
    pub fn new() -> Self {
        NoDatabaseConnectionError {
            error_code: 1,
            message: NO_DB_CONNECTION_ERR.to_string(),
            timeout: Duration::from_secs(0),
            condition: Some(Box::new(|| get_app_handle().is_some())),
            handler: ErrorHandler::populate_toast(NO_DB_CONNECTION_ERR),
        }
    }
}

impl Error for NoDatabaseConnectionError {
    fn error_code(&self) -> u32 {
        self.error_code
    }

    fn message(&self) -> &String {
        &self.message
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }

    fn condition(&self) -> &Option<Box<dyn Fn() -> bool + Send>> {
        &self.condition
    }

    fn handler(&self) -> &Box<dyn Fn() + Send> {
        &self.handler
    }

    fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    fn set_message(&mut self, message: String) {
        self.message = message;
    }

    fn set_handler(&mut self, handler: Box<dyn Fn() + Send>) {
        self.handler = handler;
    }
}
