use crate::config::Config;
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};

#[derive(Debug)] // Added Debug
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

pub fn check_website(url: &str, config: &Config) -> WebsiteStatus {
    let start = Instant::now();

    for _ in 0..config.max_retries {
        let response = ureq::get(url)
            .timeout(Duration::from_secs(config.request_timeout))
            .call();

        let response_time = start.elapsed();

        match response {
            Ok(res) => {
                return WebsiteStatus {
                    url: url.to_string(),
                    status: Ok(res.status()),
                    response_time,
                    timestamp: Utc::now(),
                };
            }
            Err(err) => {
                eprintln!("Error for {}: {:?}", url, err);
            }
        }
    }

    WebsiteStatus {
        url: url.to_string(),
        status: Err("Failed after retries".to_string()),
        response_time: start.elapsed(),
        timestamp: Utc::now(),
    }
}
