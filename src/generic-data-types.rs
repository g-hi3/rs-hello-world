// Rust uses the same syntax to define generic type parameters as C#.
// To restrict a generic type, it can use the type declaration syntax.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T
}

// Multiple type parameters can be used.
// Enums can also use generic type parameters.
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

// Impl blocks may also use generic type parameters.
// This is more akin to the way java uses type parameter definitions.
// This is because we need to define T in the impl block, so we're defining a function for Point<T>.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Rust uses monomorphization to generate concrete implementations of generic functions.
// This means, runtime speed will not be affected by generics.
// Compile-time speed however will be affected.

// Alternatively, we'd be able to implement methods for specific implementations.
impl Point<i32> {
    fn xy(&self) -> i32 {
        self.x * self.y
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    let some_point = Point { x: 16, y: -2 };
}