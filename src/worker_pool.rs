use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct WorkerPool<T> {
    threads: Vec<thread::JoinHandle<()>>,
    sender: Sender<T>,
}

impl<T: Send + 'static> WorkerPool<T> {
    pub fn new<F>(num_threads: usize, worker_func: F) -> (Self, Arc<Mutex<mpsc::Receiver<T>>>)
    where
        F: Fn(T) + Send + 'static + Clone,
    {
        let (sender, receiver) = mpsc::channel();
        let mut threads = Vec::new();

        let receiver = Arc::new(Mutex::new(receiver)); // Wrap the receiver in Arc<Mutex>

        for _ in 0..num_threads {
            let worker_receiver = receiver.clone(); // Clone the Arc for thread safety
            let worker_func = worker_func.clone();

            threads.push(thread::spawn(move || {
                while let Ok(job) = worker_receiver.lock().unwrap().recv() {
                    worker_func(job);
                }
            }));
        }

        (Self { threads, sender }, receiver) // Return the Arc<Mutex<Receiver>>
    }

    pub fn sender(&self) -> Sender<T> {
        self.sender.clone()
    }
}

impl<T> Drop for WorkerPool<T> {
    fn drop(&mut self) {
        for thread in self.threads.drain(..) {
            thread.join().expect("Thread failed to join");
        }
    }
}
