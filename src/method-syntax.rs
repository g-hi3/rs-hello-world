#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// This impl block can contain method definitions.
// It is associated with the type that follows.
// It's possible to define multiple impl blocks for the same type.
// This doesn't yield any benefits (so far).
impl Rectangle {
    // Methods are function in the context of structs, enums or traits.
    // Their first parameter is a reference to the struct.
    // &self is shorthand for self: &Self.
    // Self is an alias for the type of the impl block.
    // Methods must define a parameter called self of the impl type.
    // The self parameter can be immutable or mutable.
    // Methods can have the same name as a field on the same type.
    // Functions defined in the impl block are also called associated functions.
    // An associated function is a method, when the self parameter is present.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // This is called the method syntax (calling the method using the dot operator).
        // A method can also be called by using Rectangle::area(&rect1).
        // Rust uses automatic dereferencing.
        // This means that there is no -> operator as in C++, because Rust automatically dereferences the value.
        rect1.area());
    
    // This is how an associated function is called.
    let square = Rectangle::square(15);
    println!(
        "The area of a square with size 15 is {} pixels.",
        square.area());
}
