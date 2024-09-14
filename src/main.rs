
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Create an array of 10 integer numbers
    let numbers = [3, 5, 7, 10, 15, 18, 22, 30, 35, 40];

    // Use a for loop to iterate through the array
    for &num in numbers.iter() {
        // Check if the number is divisible by both 3 and 5
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            // Check if the number is divisible by 3
            println!("Fizz");
        } else if num % 5 == 0 {
            // Check if the number is divisible by 5
            println!("Buzz");
        } else if is_even(num) {
            // Check if the number is even
            println!("{} is even", num);
        } else {
            // If none of the above, it is odd
            println!("{} is odd", num);
        }
    }

    // Use a while loop to find and print the sum of all numbers in the array
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Use a loop to find and print the largest number in the array
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}
