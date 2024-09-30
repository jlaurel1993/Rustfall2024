use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write, BufReader};

struct Car {
    make: String,
    model: String,
    year: u32,
}

fn main() {
    let mut buffer = String::new();

    // Getting car make
    print!("Enter the car make: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let make = buffer.trim().to_string();
    buffer.clear();

    // Getting car model
    print!("Enter the car model: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    buffer.clear();

    // Getting car year
    print!("Enter the car year: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year: u32 = buffer.trim().parse().expect("Please enter a valid year");
    
    // Creating the car struct
    let car = Car { make, model, year };

    // Saving car info to user_info.txt
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("user_info.txt")
        .unwrap();

    writeln!(file, "Car Make: {}", car.make).unwrap();
    writeln!(file, "Car Model: {}", car.model).unwrap();
    writeln!(file, "Car Year: {}", car.year).unwrap();
    writeln!(file, "--------------------------").unwrap();

    // Reading from user_info.txt and printing to screen
    let file = File::open("user_info.txt").unwrap();
    let reader = BufReader::new(file);

    println!("\nContents of user_info.txt:");
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
