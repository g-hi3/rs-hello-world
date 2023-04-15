use std::thread;
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
}