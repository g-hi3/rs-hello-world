use std::{
    sync::{
        Arc,
        mpsc,
        mpsc::Receiver,
        mpsc::Sender,
        Mutex},
    thread,
    thread::{JoinHandle}};

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        // If the operating system can't create any more threads, `spawn` will panic.
        // To avoid panicking, you can use `Builder::spawn` instead.
        let thread = thread::spawn(move || loop {
            // A "chained" call like this immediately drops any temporary values in between, as soon as the `let` finishes.
            // The Rust book explains that using `while let Ok(job) = receiver.[...].recv()` wouldn't work, because `while let` does not drop the values until the end of the associated block.
            let message = receiver
                // A mutex might be in a so-called "poisoned" state, if another thread that holds it panicked and didn't release it.
                .lock()
                .unwrap()
                .recv();
            match message {
                Ok(job) => job(),
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            };
        });
        Worker { id, thread: Some(thread) }
    }
}

pub struct PoolCreationError(usize);
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>
}

impl ThreadPool {
    // To generate documentation, use `cargo doc`.
    /// Creates a new `ThreadPool` with `size` as the number of available threads.
    ///
    /// # Panics
    ///
    /// This function will panic, when `size = 0`.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for worker_id in 0..size {
            workers.push(Worker::new(worker_id, Arc::clone(&receiver)))
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            Ok(Self::new(size))
        } else {
            Err(PoolCreationError(size))
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread
                    .join()
                    .unwrap();
            }
        }
    }
}