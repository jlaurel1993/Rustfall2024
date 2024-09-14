
// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// Function to convert Celsius to Fahrenheit (can be used later if needed)
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    
    let mut fahrenheit: f64 = 32.0;

    // Convert it to Celsius using your function and print the result
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{}°F is {}°C", fahrenheit, celsius);

    
    for _ in 1..=5 { 
        fahrenheit += 1.0;
        let celsius = fahrenheit_to_celsius(fahrenheit);
        println!("{}°F is {}°C", fahrenheit, celsius);
    }
}
