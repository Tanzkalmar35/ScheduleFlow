use tauri::{Emitter, Manager};

use crate::runtime_objects::get_app_handle;
use std::time::Duration;

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
    /// * `message` - The error message to be displayed on the toast message.
    pub(crate) fn populate_toast(message: &'static str) -> Box<dyn Fn() + Send + 'static> {
        Box::new(move || {
            get_app_handle()
                .unwrap()
                .app_handle()
                .emit("createToast", ("error", message)).expect("Could not create toast notification");
        })
    }

    /// Handles the error by not handling it. Does a intentional app crash.
    /// This error should not occur. If it does, its 99.99% my fault. (It can be due to cosmic rays though)
    ///
    /// # Params
    /// * `message` - The message the app will show in the logs after crashed.
    pub(crate) fn panic(message: &'static str) -> Box<dyn Fn() + Send + 'static> {
        Box::new(move || panic!("{}", message))
    }
}
