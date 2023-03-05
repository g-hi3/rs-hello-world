// The Rust book says that a variable can only be one of the values inside an Enum.
// This seems to suggest that there is no way to or-combine the enums like V4 | V6.
enum IpAddrKind {
    // Enums can have values inside of them.
    // Usually, this is a more concise way of describing grouped data.
    V4(u8, u8, u8, u8),
    // Each enum value can have different values.
    // That's why V6 can have string as a value.
    V6(String)
}

fn main() {
    // This is how an instance of an enum is created.
    // Rust allows us to create V4 without specifying the string value.
    let _four = IpAddrKind::V4;
    // Because V6 contains a string value, it can be initialized like this.
    let _loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello world"));
    m.call();

    // The Option type is comparable to Optional<T> in Java or Nullable<T> in C#.
    // The enum value None represents that the value is not set.
    let _absent_number: Option<i32> = None;
    // The enum for a value being present is called Some.
    // Rust can also infer the generic type from the value used.
    let _some_number = Some('e');
}

enum Message {
    Write(String),
    // The Rust compiler shows each enum value that isn't being used in the code.
    ChangeColor(i32, i32, i32),
    // This is the enum equivalent to a unit struct.
    Quit,
    // Enums can have named values.
    Move { x: i32, y: i32 }
}

// Enums can also have an impl block, like structs.
impl Message {
    fn call(&self) {
        // Method body would be defined here.
    }
}