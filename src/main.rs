use std::fs::OpenOptions;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use serde::Deserialize;
use serde_json::from_str;
use ureq;

// Define structs for JSON deserialization only.
#[derive(Deserialize)]
#[allow(dead_code)] // Suppress unused field warning
struct CurrencyPrice {
    usd: f64,
}

#[derive(Deserialize)]
#[allow(dead_code)] // Suppress unused field warning
struct GlobalQuote {
    #[serde(rename = "05. price")]
    price: String, // Parse as f64 when needed
}

// Define the Pricing trait with methods to fetch the price and save it to a file.
trait Pricing {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>>;
    fn save_to_file(&self, price: f64) -> std::io::Result<()>;
}

// Backoff mechanism for fetching prices with rate limit handling
fn fetch_price_with_backoff(url: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let mut attempts = 0;
    let max_attempts = 5;

    loop {
        let response = ureq::get(url).call();
        match response {
            Ok(res) => {
                let response_text = res.into_string()?;
                let parsed: serde_json::Value = serde_json::from_str(&response_text)?;
                if let Some(price) = parsed["bitcoin"]["usd"].as_f64() {
                    return Ok(price);
                } else if let Some(price) = parsed["ethereum"]["usd"].as_f64() {
                    return Ok(price);
                }
                return Err("Unexpected JSON structure".into());
            }
            Err(ureq::Error::Status(429, _)) => {
                if attempts < max_attempts {
                    attempts += 1;
                    let backoff_time = 2u64.pow(attempts) * 1000;
                    println!("Rate limit reached. Retrying in {} ms...", backoff_time);
                    std::thread::sleep(Duration::from_millis(backoff_time));
                } else {
                    return Err("Exceeded max retry attempts due to rate limiting".into());
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
}

// Implement the Pricing trait for Bitcoin
struct Bitcoin;

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        fetch_price_with_backoff("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd")
    }

    fn save_to_file(&self, price: f64) -> std::io::Result<()> {
        let mut file = OpenOptions::new().append(true).create(true).open("bitcoin.txt")?;
        writeln!(file, "{}", price)
    }
}

// Implement the Pricing trait for Ethereum
struct Ethereum;

impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        fetch_price_with_backoff("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd")
    }

    fn save_to_file(&self, price: f64) -> std::io::Result<()> {
        let mut file = OpenOptions::new().append(true).create(true).open("ethereum.txt")?;
        writeln!(file, "{}", price)
    }
}

// Implement the Pricing trait for SP500 with backoff for rate limiting
struct SP500;

impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let mut attempts = 0;
        let max_attempts = 5;

        loop {
            let response_text = ureq::get("https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=^GSPC&apikey=YOUR_API_KEY")
                .call()?
                .into_string()?;

            let response: serde_json::Value = from_str(&response_text)?;
            
            if response.as_object().map(|obj| obj.is_empty()).unwrap_or(false) {
                eprintln!("Error: Received an empty response from the API. Possibly due to rate limits.");

                if attempts < max_attempts {
                    attempts += 1;
                    let backoff_time = 2u64.pow(attempts) * 1000;
                    println!("Retrying in {} ms...", backoff_time);
                    std::thread::sleep(Duration::from_millis(backoff_time));
                } else {
                    return Err("Exceeded max retry attempts due to rate limiting".into());
                }
            } else if let Some(global_quote) = response.get("Global Quote") {
                if let Some(price) = global_quote.get("05. price").and_then(|p| p.as_str()) {
                    return Ok(price.parse::<f64>()?);
                }
            } else {
                eprintln!("Unexpected response format: {}", response_text);
                return Err("Missing or invalid 'Global Quote' field in response".into());
            }
        }
    }

    fn save_to_file(&self, price: f64) -> std::io::Result<()> {
        let mut file = OpenOptions::new().append(true).create(true).open("sp500.txt")?;
        writeln!(file, "{}", price)
    }
}

fn main() {
    // Create instances of each asset and store them in a vector as trait objects.
    let assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin),
        Box::new(Ethereum),
        Box::new(SP500),
    ];

    // Infinite loop to periodically fetch and save pricing data every 10 seconds.
    loop {
        for asset in &assets {
            match asset.fetch_price() {
                Ok(price) => {
                    asset.save_to_file(price).expect("Failed to save price");
                    println!("Fetched and saved price: {}", price);
                }
                Err(e) => eprintln!("Failed to fetch price: {}", e),
            }
        }
        sleep(Duration::from_secs(10)); // Wait for 10 seconds before the next fetch
    }
}
