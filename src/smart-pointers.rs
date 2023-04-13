// References are considered a kind of pointer.
// Smart pointers carry additional data.
// Smart pointers in Rust also own the data in many cases, instead of just borrowing it.
// `String` and `Vec<T>` are some kinds of smart pointers.
// Smart pointers implement the `Deref` and `Drop` traits.
// With the `Deref` trait, we can customize what happens when the dereference operator is used.
// With the `Drop` trait, we can customize what happens when an instance goes out of scope.

use std::ops::Deref;

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
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

enum List {
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