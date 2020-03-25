/// A logging manager.
#[derive(Clone)]
pub struct Logger {}

impl Logger {
    /// Handles the logging of information.
    pub fn log_info(&self, message: String) {
        println!("{}", message);
    }

    /// Handles the logging of errors.
    pub fn log_error(&self, error: String) {
        eprintln!("{}", error);
    }
}