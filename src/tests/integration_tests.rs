#[cfg(test)]
mod tests {
    use super::*;
    use crate::website_checker::check_website;
    use crate::config::Config;

    #[test]
    fn test_successful_request() {
        let config = Config::new();
        let status = check_website("https://www.google.com", &config);
        assert!(status.status.is_ok());
    }

    #[test]
    fn test_failed_request() {
        let config = Config::new();
        let status = check_website("https://nonexistentwebsite.xyz", &config);
        assert!(status.status.is_err());
    }

    #[test]
    fn test_timeout_request() {
        let mut config = Config::new();
        config.request_timeout = 1; // 1-second timeout
        let status = check_website("https://httpbin.org/delay/10", &config);
        assert!(status.status.is_err());
    }
}
