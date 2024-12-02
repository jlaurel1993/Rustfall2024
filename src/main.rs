use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    const NUM_THREADS: usize = 5;
    const INCREMENTS_PER_THREAD: usize = 10;

    // Shared counter
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Spawn threads
    for i in 1..=NUM_THREADS {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            println!("Thread {} started", i);
            for _ in 0..INCREMENTS_PER_THREAD {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
            println!("Thread {} completed", i);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print final counter value
    println!("All threads completed");
    println!("Final counter value: {}", *counter.lock().unwrap());
}
