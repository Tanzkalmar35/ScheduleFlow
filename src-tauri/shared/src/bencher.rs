use std::time::Instant;

/// A simple utility class that makes it easier to benchmark processes.
pub struct Bencher {
    instant: Option<Instant>,
    msg: &'static str,
}

impl Bencher {
    /// Creates a new Bencher instance without any custom message printed, so the default message is used.
    pub fn new() -> Self {
        Self {
            instant: None,
            msg: "",
        }
    }

    /// Creates a new Bencher instance with a custom message that will be printed after the bench.
    ///
    /// # Params
    ///
    /// * `msg` - The benching message printed out afterwards, formatted {}{}, msg, time
    pub fn new_msg(msg: &'static str) -> Self {
        Self { instant: None, msg }
    }

    /// Starts the benchmarking process.
    pub fn start(&mut self) {
        self.instant = Some(Instant::now());
    }

    /// Stops the benchmarking process and prints out an according message.
    pub fn stop(&mut self) {
        if let Some(instant) = self.instant {
            let elapsed = instant.elapsed();
            if self.msg.is_empty() {
                log::info!("Process finished in {:.2?}", elapsed);
            } else {
                log::info!("{}: {:.2?}", self.msg, elapsed);
            }
        } else {
            panic!("Attempted to stop a bench that did not start.")
        }

        self.instant = None;
    }
}
