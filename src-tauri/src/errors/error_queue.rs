use std::{fmt, thread};
use std::collections::VecDeque;
use std::sync::{Arc, mpsc, Mutex};
use std::time::Duration;
use crate::errors::error_utils::Error;
use crate::runtime_objects::get_current_window;

pub struct ErrorQueue {
    queue: Arc<Mutex<VecDeque<dyn Error>>>,
    tx: mpsc::Sender<()>,
}

impl ErrorQueue {
    pub(crate) fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let queue = Arc::new(Mutex::new(VecDeque::<dyn Error>::new()));
        let queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            loop {
                rx.recv().unwrap();
                let mut queue = queue_clone.lock().unwrap();
                while let Some(mut err) = queue.pop_front() {
                    thread::sleep(err.timeout());

                    if (err.condition().is_some()) {
                        if (err.condition()).as_ref().unwrap()() {
                            err.handler();
                        } else {
                            err.set_timeout(Duration::from_secs(1));
                            queue.push_back(err);
                        }
                    } else {
                        err.handler();
                    }
                }
            }
        });
        drop(handle); // Ensure the thread is running detached
        ErrorQueue { queue, tx }
    }

    pub(crate) fn enqueue(&self, err: impl Error) {
        self.queue.lock().unwrap().push_back(err);
        self.tx.send(()).unwrap(); // Send a dummy message to the receiver to wake the thread up
    }
}
