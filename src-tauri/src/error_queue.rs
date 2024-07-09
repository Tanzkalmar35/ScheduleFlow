use std::collections::VecDeque;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

use crate::runtime_objects::get_current_window;

#[derive(Clone)]
pub struct Error {
    message: String,
    population_condition: Option<Box<dyn Fn() -> bool + Send>>,
    population_action: Box<dyn Fn() + Send>,
    timeout: Duration,
}

impl Error {
    /**
     * Creates a new default Error object with all dummy fields.
     */
    pub(crate) fn default() -> Self {
        Self {
            message: String::new(),
            population_condition: None,
            population_action: Box::new(|| {}),
            timeout: Duration::from_secs(0),
        }
    }

    /**
     * Initializes the error message of the error.
     *
     * # Params
     * - message: The error message to be displayed - Should describe why the error occurred.
     */
    pub(crate) fn with_message(mut self, message: String) -> Self {
        self.message = message;
        self
    }

    /**
     * Initializes the initial timeout of the error.
     *
     * # Params
     * - timeout: The initial timeout the error handler waits before attempting to process the error.
     */
    pub(crate) fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /**
     * Initializes the condition, on which the error gets populated in the first place.
     *
     * # Params
     * - condition: The condition that decides if the error is ready to be handled
     */
    pub(crate) fn with_condition<C>(mut self, condition: C) -> Self
    where
        C: Fn() -> bool + Send,
    {
        self.population_condition = Some(Box::new(condition));
        self
    }

    /**
     * Initializes the action, which is performed to handle the error.
     *
     * # Params
     * - action: The action that handles the error.
     */
    pub(crate) fn with_action<A>(mut self, action: A) -> Self
    where
        A: Fn() + Send,
    {
        self.population_action = Box::new(action);
        self
    }
}

pub struct ErrorHandler;

impl ErrorHandler {
    pub(crate) fn populate_toast(error: Error) -> Box<dyn Fn() + Send> {
        Box::new(|| {
            get_current_window().unwrap().emit("createToast", ("error", &error.message));
        })
    }
}

pub struct ErrorQueue {
    queue: Arc<Mutex<VecDeque<Error>>>,
    tx: mpsc::Sender<()>,
}

impl ErrorQueue {
    pub(crate) fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let queue = Arc::new(Mutex::new(VecDeque::<Error>::new()));
        let queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            loop {
                rx.recv().unwrap();
                let mut queue = queue_clone.lock().unwrap();
                while let Some(mut err) = queue.pop_front() {
                    thread::sleep(err.timeout);

                    if (err.population_condition.is_some()) {
                        if (err.population_condition).as_ref().unwrap()() {
                            err.population_action;
                        } else {
                            err.timeout = Duration::from_secs(1);
                            queue.push_back(err);
                        }
                    } else {
                        err.population_action;
                    }
                }
            }
        });
        drop(handle); // Ensure the thread is running detached
        ErrorQueue { queue, tx }
    }

    pub(crate) fn enqueue(&self, err: Error) {
        self.queue.lock().unwrap().push_back(err);
        self.tx.send(()).unwrap(); // Send a dummy message to the receiver to wake the thread up
    }
}
