// A trait is comparable to interfaces in languages such as Java or C#.
// It defines the head of functions to be implemented by otehr types.
// It does not contain any data.
pub trait Summary {
    // Note that the line ends with a semicolon.
    fn summarize_author(&self) -> String;

    // A function defined in a trait may also provide a default implementation.
    fn summarize(&self) -> String {
        // Default implementations may call other functions defined on the trait like this.
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// By itself, the struct does not implement a trait.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

// The trait is implemented by using the for keyword in the impl block.
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // Other functions that aren't defined in the trait may not be implemented here.
}

// The example uses both types in another crate.
// Because the use statement is interesting, I decided to include it as a comment here:
// use aggregator::{Summary, Tweet};
// It allows us to "import" two types from the same crate.
// Interestingly, to use summarize(), we needed to import Summary as well, even though it isn't explicitly used.
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// Traits can only be implemented if either the trait or the struct is local to the current crate.
// For example, we can implement Display on a custom type or have Vec<T> implement our Summary trait.
// It is not possible to implement an external trait for an external type (e.g. Display for Vec<T>).
// This is called coherence, or more specifically the orphan rule.

// Traits may be used as parameters like this.
// This tells the compiler that the function may be used with any type that implements Summary.
// This is syntactic sugar for definining a trait boundary.
// pub fn notify<T: Summary>(item: &T)
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// The impl syntax is useful if a function takes two Summary references, but doesn't care if they are of the same type.
// pub fn notify(item1: &impl Summary, item2: &impl Summary)
// is the same as
// pub fn notify<T1: Summary, T2: Summary>(item1: &T1, item2: &T2)

// It is also possible to combine multiple traits using the + syntax.
// pub fn notify(item: &(impl Summary + Display))
// This syntax is also valid for generic types.
// pub fn notify<T: Summary + Display>(item: &T)

use std::fmt::Display;

// An alternate syntax for trait boundaries is the where syntax.
// This is very similar to the where syntax in C#.
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone
{
    0
}

// Traits can be returned by using this impl syntax.
// However, it is not possible to return values of different types.
// So a factory function would not be possible here.
// The rust book says that there is a possibility to do so explained in a later chapter.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
                "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false
    }
}

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// This syntax allows us to implement cmp_display for Pair with a type parameter that implements Display and PartialOrd.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Blanket implementations are impl blocks that implement a trait for a type that implements another trait.
// impl<T: Display> ToString for T