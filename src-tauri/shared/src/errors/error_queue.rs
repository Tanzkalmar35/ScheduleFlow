use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::errors::error_utils::Error;

#[derive(Clone)]
pub struct ErrorQueue {
    queue: Arc<Mutex<VecDeque<Box<dyn Error + Send>>>>,
    tx: mpsc::Sender<()>,
}

impl ErrorQueue {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let queue = Arc::new(Mutex::new(VecDeque::<Box<dyn Error + Send>>::new()));
        let queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || loop {
            rx.recv().unwrap();
            let mut queue = queue_clone.lock().unwrap();
            while let Some(mut err) = queue.pop_front() {
                thread::sleep(err.timeout());

                if err.condition().is_some() {
                    if err.condition().as_ref().unwrap()() {
                        println!("An error occured: {}", err.message());
                        err.handler()();
                    } else {
                        err.set_timeout(Duration::from_secs(1));
                        queue.push_back(err);
                    }
                } else {
                    err.handler()();
                }
            }
        });
        drop(handle); // Ensure the thread is running detached
        ErrorQueue { queue, tx }
    }

    pub fn enqueue(&self, err: impl Error + Send + 'static) {
        self.queue.lock().unwrap().push_back(Box::new(err));
        self.tx.send(()).unwrap(); // Send a dummy message to the receiver to wake the thread up
    }
}
