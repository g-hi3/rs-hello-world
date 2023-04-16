use std::thread;
use std::sync::atomic::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::time::Duration;

fn main() {
    // The `spawn()` function creates a new thread.
    // Threads in Rust are 1:1 operating system threads, unlike in other languages where processes can run in the same thread.
    // There are libraries that implement threading in a different way.
    // It's possible that this thread isn't run at all.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread");
            // From what I understand, these thread functions match the names of the C API.
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Joining the thread here would cause the code to print 1..9 first, then 1..5.

    for i in 1..5 {
        println!("Hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    // `join()` blocks the current thread until the process in `handle` finishes.
    handle.join().unwrap();

    let v = vec![1, 2, 3];
    // The keyword `move` is required for this closure, because `spawn()` requires the lifetime to be `'static`.
    // This means, ownership must be moved to the thread to make sure all captured variables will be valid.
    thread::spawn(move || {
        println!("Here's a vector {v:?}");
    });

    // Rust takes inspiration from the Go language for communication between threads.
    // It tells us, that threads should share data by communicating, instead of communicate by sharing data.
    // Rust implements channels for this.
    // A channel has an input stream and an output stream and is considered closed, when either stream is dropped.

    // MPSC is short for multiple producers, single receiver.
    let (transmitter, receiver) = mpsc::channel();
    // To create another producer, we can call the `clone()` method.
    let other_transmitter = transmitter.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            // After sending, the value is moved to the receiving thread.
            // This makes sure, that the value is not accessed when it might have already been modified or dropped.
            transmitter.send(x).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            other_transmitter.send(x).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // `recv()` will block the thread until a value is received.
    // When the thread exits, an error will be returned.
    // `try_recv()` will immediately return the `Result<T, E>`, giving the value if one is available, or an error if not.
    let received = receiver.recv().unwrap();
    println!("Received {received}");

    // The receiver also acts as an iterator, providing values received.
    for received in receiver {
        println!("Received {received}");
    }

    // Mutex is short for mutual exclusion.
    // They are used to manage access to data between multiple threads.
    // A thread needs to acquire the mutex lock in order to access the data.
    // As soon as its done, it has to unlock the mutex, so other threads can access the data again.

    // Arc is short for atomic reference count.
    // Using atomic operations comes with some performance overhead that would be unnecessary in single-threaded contexts.
    // Just as `RefCell<T>` can create memory leaks, it's possible for `Mutex<T>` to create deadlocks.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // `lock()` blocks the current thread, until the data is available.
            // In this case, `lock()` returns a `LockResult<MutexGuard<i32>>`.
            // The `MutexGuard<T>` is a smart pointer that releases the lock, when it is dropped.
            let mut num = m.lock().unwrap();
            *num = 6;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result = {:?}", *counter.lock().unwrap());
}