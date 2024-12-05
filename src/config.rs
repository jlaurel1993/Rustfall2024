#[derive(Clone)] // Add this line
pub struct Config {
    pub num_threads: usize,
    pub request_timeout: u64,
    pub max_retries: usize,
}

impl Config {
    pub fn new() -> Self {
        Self {
            num_threads: 4,
            request_timeout: 5,
            max_retries: 3,
        }
    }
}
