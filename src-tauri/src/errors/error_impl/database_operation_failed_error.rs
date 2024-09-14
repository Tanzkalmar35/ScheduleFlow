use crate::errors::{
    error_messages::QUERY_FAILED_ERR,
    error_utils::{Error, ErrorHandler},
};
use std::time::Duration;

pub struct DatabaseOperationFailedError {
    error_code: u32,
    message: String,
    timeout: Duration,
    condition: Option<Box<dyn Fn() -> bool + Send>>,
    handler: Box<dyn Fn() + Send>,
}

impl DatabaseOperationFailedError {
    /// Initializes a new NoDatabaseConnectionError with its default params.
    ///
    /// # Default params
    /// * `error_code` - 1, database related errors.
    /// * `message` - error_messages::DB_OPERATION_FAILED.
    /// * `timeout` - 0 seconds, no default delay before population.
    /// * `condition` - None, gets handled no matter what.
    /// * `handler` - ErrorHandler::QUERY_FAILED_ERR.
    pub fn new() -> Self {
        Self {
            error_code: 1,
            message: QUERY_FAILED_ERR.to_string(),
            timeout: Duration::from_secs(0),
            condition: None,
            handler: ErrorHandler::panic(QUERY_FAILED_ERR),
        }
    }
}

impl Error for DatabaseOperationFailedError {
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
