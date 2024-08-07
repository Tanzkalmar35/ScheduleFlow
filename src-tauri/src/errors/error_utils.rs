use std::sync::Arc;
use std::time::Duration;
use crate::runtime_objects::get_current_window;

pub enum ErrorCode {
    ONE = 1,
    TWO = 2,
}

/// Template for custom error messages for this application.
///
/// # Params
/// * `error_code` - The error code indicating the group of errors this specific one belongs to.
/// * `message` - The error message indicating what exactly happens.
/// * `timeout` - The timeout the ErrorQueue waits before handling the error.
/// * `condition` - The condition that needs to be met for the error to be handled by the handler.
/// * `handler` - The handler specifies the action that is performed to handle the error properly.
///
/// # Important
/// The condition always needs to grant the action performed by the handler!!!
pub trait Error {

    fn error_code(&self) -> u32;
    fn message(&self) -> &String;
    fn timeout(&self) -> Duration;
    fn condition(&self) -> &Option<Box<dyn Fn() -> bool + Send>>;
    fn handler(&self) -> &Box<dyn Fn() + Send>;

    /// Initializes a custom timeout for the error.
    ///
    /// # Params
    /// * `timeout` - The custom timeout the error handler delays the handling
    fn set_timeout(&mut self, timeout: Duration);

    /// Initializes a custom error message for the error.
    ///
    /// # Params
    /// * `message` - The custom error message that will be displayed
    fn set_message(&mut self, message: String);

    /// Initializes a custom error handler for the error.
    ///
    /// # Params
    /// * `handler` - The custom ErrorHandler function that is responsible for handling the error properly.
    fn set_handler(&mut self, handler: Box<dyn Fn() + Send>);
}

/// Implements methods that handle errors in various ways.
pub struct ErrorHandler;

impl ErrorHandler {

    /// Handles the error by populating a toast message.
    ///
    /// # Params
    /// * `error` - The custom error implementation.
    pub(crate) fn populate_toast(message: String) -> Box<dyn Fn() + Send + 'static> {
        Box::new(move || {
            get_current_window().unwrap().emit("createToast", ("error", message.clone()));
        })
    }
}
