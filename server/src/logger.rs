#[derive(Clone)]
pub struct Logger {}

impl Logger {
    pub fn log_info(&self, message: String) {
        println!("{}", message);
    }

    pub fn log_error(&self, error: String) {
        eprintln!("{}", error);
    }
}