mod config;
mod worker_pool;
mod website_checker;

use config::Config;
use website_checker::check_website;
use worker_pool::WorkerPool;

fn main() {
    // Initialize configuration
    let config = Config::new();

    // List of URLs to monitor
    let urls = vec![
        "https://www.google.com",
        "https://www.github.com",
        "https://www.nonexistentwebsite.xyz",
    ];

    // Create a worker pool
    let (pool, _receiver) = WorkerPool::new(config.num_threads, move |url: String| {
        let status = check_website(&url, &config);
        println!("{:?}", status); // Print the website status
    });

    // Send URLs to the worker pool
    for url in urls {
        pool.sender()
            .send(url.to_string())
            .expect("Failed to send URL to worker pool");
    }

    // Drop the pool to ensure all threads are joined before program exits
    drop(pool);
}
