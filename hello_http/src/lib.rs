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
    handle: JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        // If the operating system can't create any more threads, `spawn` will panic.
        // To avoid panicking, you can use `Builder::spawn` instead.
        let handle = thread::spawn(move || loop {
            // A "chained" call like this immediately drops any temporary values in between, as soon as the `let` finishes.
            // The Rust book explains that using `while let Ok(job) = receiver.[...].recv()` wouldn't work, because `while let` does not drop the values until the end of the associated block.
            let job = receiver
                // A mutex might be in a so-called "poisoned" state, if another thread that holds it panicked and didn't release it.
                .lock()
                .unwrap()
                .recv()
                .unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });
        Worker { id, handle }
    }
}

pub struct PoolCreationError(usize);
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>
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

        ThreadPool { workers, sender }
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
            .send(job)
            .unwrap();
    }
}