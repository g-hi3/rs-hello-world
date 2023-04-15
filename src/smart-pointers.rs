// References are considered a kind of pointer.
// Smart pointers carry additional data.
// Smart pointers in Rust also own the data in many cases, instead of just borrowing it.
// `String` and `Vec<T>` are some kinds of smart pointers.
// Smart pointers implement the `Deref` and `Drop` traits.
// With the `Deref` trait, we can customize what happens when the dereference operator is used.
// With the `Drop` trait, we can customize what happens when an instance goes out of scope.

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

fn main() {
    // A box is used to store data on the heap instead of the stack.
    // In contrast to Java and C#, this does not come with a performance overhead.
    // It can be used for moving a lot of data without it being copied (only the reference on the stack is being moved).
    // It can also be used for expecting a Trait but not caring about the implementation (Box<dyn Trait>).
    let b = Box::new(5);
    println!("b = {b}");

    // A box can be used to create infinitely recursive data types.
    let list = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3,
                Box::new(List::Nil))))));

    // The reference operator (&) creates a reference to the value.
    let x = 5;
    let y = &x;
    let z = Box::new(x);

    // The dereference operator (*) resolves the value pointed to by the reference.
    // From what I understand, reference coercion will be applied, when calling methods on references.
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // The dereference operator can also be used on a `Box<T>`.
    // Behind the scenes, the compiler does *(z.deref()) here.
    assert_eq!(5, *z);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // If Rust didn't use deref coercion, this code would look like this (thank goodness):
    //     hello(&(*m)[..]);

    let c = CustomSmartPointer {
        data: String::from("my stuff")
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };
    println!("CustomSmartPointers created.");

    // Rust doesn't allow you to call `drop()` manually.
    // Instead we need to use the `std::mem::drop()` function.
    // Usually, this isn't necessary.
    // The Rust book uses the example of locks, that must be released early by dropping.
    // The compiler calls the drop function a destructor.
    drop(d);
    println!("Dropped second CustomSmartPointer early.");

    // Rc is an abbreviation for reference counting.
    // `Rc<T>` keeps track of the number of references to a value.
    // The Rust book names graphs as an example where values might have multiple owners.
    // For multithreaded code, use `Arc<T>` instead.

    let a = Rc::new(
        List::RcCons(
            5,
            Rc::new(
                List::RcCons(
                    10,
                    Rc::new(
                        List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // Rust convention is to use `Rc::clone(&a)` instead of `a.clone()` in this case.
    // This is because usually `a.clone()` creates a deep copy of the data.
    let b = List::RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = List::RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // Interior mutability is a design pattern in Rust that allows mutation of immutable data structures.
    // We use unsafe code to do this.
    // Unsafe code tells the compiler that we check the borrowing rules manually, instead of relying on the compiler.

    // `RefCell<T>` enforces borrowing rules at runtime instead of compile time.
    // If the borrowing rules are broken at runtime, the program panics.
    // `RefCell<T>` is intended for single threaded code (`Mutex<T>` for multithreaded code).
    // It counts the references provided by `borrow()` and `borrow_mut()`.
    // The borrowing rules are enforces like this at runtime.
    // That means, calling `borrow_mut()` twice will cause it to panic.

    // Using `Rc<RefCell<T>>` allows you to have multiple mutable ownership over a value.

    // It's possible to create memory leaks when some data points to other data and that data points to the former.
    // An `Rc<T>` also tracks the count of weak references (`Weak<T>`).
    // Weak references are not being considered when cleaning up the data.

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

enum List {
    RcCons(i32, Rc<List>),
    Cons(i32, Box<List>),
    Nil
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // This definition is necessary, because T can not be used directly.
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// For mutable references, you can also implement the DerefMut trait.
// It's possible to implement DerefMut to return an immutable reference.
// It's not possible to implement Deref to return a mutable reference!

#[derive(Debug)]
struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger
                .send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    // This is a good example for using `Weak<T>`.
    // A node should not own their parent, creating a reference cycle.
    // Using `Weak<T>` allows this node to be cleaned up, if it isn't used anymore.
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // With `borrow_mut()`, we get a mutable reference to the vector.
            // We can add the String here.
            // It returns a `RefMut<T>` that implements `Deref`.
            self.sent_messages
                .borrow_mut()
                .push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // With `borrow()`, we get an immutable reference of the vector.
        // We can use it to check the state and not modify the vector.
        // It returns `Ref<T>` which implements `Deref`.
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}