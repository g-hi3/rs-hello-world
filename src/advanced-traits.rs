use std::fmt;
use std::ops::Add;

pub trait Iterator {
    // An associated type is a placeholder.
    // When implementing a trait, the associated type will be substituted with an "concrete" type.
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter;

impl Iterator for Counter {
    // In this case, Item will be substituted with u32 in this impl block.
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// Associated types are different to generics where a generic trait can be implemented multiple types for a struct.
// An associated type can only be implemented once with a specific type.
// The associated type may be resolved to a generic type.
//     impl<T> Iterator for Implemented<T> {
//         type Item = T;
//
//         fn next(&mut self) -> Option<Self::Item> {
//             todo!();
//         }
//     }

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

// A default generic is the type that should be used if no other type is defined.
//     trait Add<Rhs=Self> {
//         type Output;
//
//         fn add(self, rhs: Rhs) -> Self::Output;
//     }

// This is an implementation that uses the default generic type as-is.
impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// This is an implementation that overrides the default generic type.
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// `Display` here is the supertrait of `OutlinePrint`.
// This means that a type that implements `OutlinePrint` must also implement `Display`.
// It also means that this function can call `self.to_string()`, even though the trait does not declare it.
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// The `Wrapper` struct acts as a known type.
struct Wrapper(Vec<String>);

// Since this crate defines `Wrapper`, we may implement `Display`.
// `Wrapper` uses the contained `Vec<String>` to implement `fmt`.
// At compile time, `Wrapper` will be removed, meaning this has no impact on performance.
// This kind of implementation is called the newtype pattern.
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    // This uses the `fly` implementation on `Human` that doesn't implement a trait.
    // This call would cause a compiler error, if `Human` didn't implement `fly` directly.
    person.fly();
    // To be clear which implementation should be used, the fully qualified syntax must be used.
    // This is roughly comparable to extension methods in C# that can be called from it's type or an instance of `this`.
    Pilot::fly(&person);
    Wizard::fly(&person);
    // This call is also valid, but it's longer than necessary.
    Human::fly(&person);

    // This returns Spot.
    println!("A baby dog is called a {}", Dog::baby_name());
    // Because `Animal::baby_name()` doesn't specify which implementation to be used, it is an invalid call.
    // Instead we can use the `<Type as Trait>` syntax to specify which implementation to use.
    // Even though we specify an implementation, the default implementation will be called.
    // This returns puppy.
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let w = Wrapper(vec![String::from("hello"), String::from("World")]);
    println!("w = {w}");
}