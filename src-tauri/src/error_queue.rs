use std::collections::VecDeque;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

use crate::runtime_objects::get_current_window;

pub struct Error {
    pub(crate) message: String,
    population_condition: Box<dyn Fn() -> bool + Send>,
    timeout: Duration,
}

impl Error {
    /**
     * Initializes a new error
     */
    pub(crate) fn new(
        message: String,
        population_condition: Box<dyn Fn() -> bool + Send>,
        timeout: Duration) -> Self {
        Self { message, population_condition, timeout }
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

                    if (err.population_condition)() {
                        get_current_window().unwrap().emit("createToast", ("error", err.message));
                    } else {
                        err.timeout = Duration::from_secs(1);
                        queue.push_back(err);
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
