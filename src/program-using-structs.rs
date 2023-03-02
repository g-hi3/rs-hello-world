fn main() {
    let width = 30;
    let height = 30;

    // Calling the area function from within the formatting string doesn't work.
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width, height));

    let dimensions = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(dimensions));

    let scale = 2;
    let rectangle = Rectangle {
        // The dbg macro prints the value to stderr and returns the result.
        width: dbg!(50 * scale),
        height: 50
    };

    // We use a reference to rectangle here, so area3 borrows it and we may use it again later.
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rectangle));
    // This syntax can be used to print the variable using the derive(Debug) attribute.
    // The expression {:?} can be used when the variable is passed as a second argument.
    // The expression {:#?} additionally pretty-prints the values.
    println!("rectangle is {rectangle:?}");
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

// Overloads are not allowed in Rust!
fn area2(dimensions: (u32, u32)) -> u32 {
    // To access individual values of a tuple, use the index as a field.
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// This is called an outer attribute.
// This derive(Debug) allows us to print the Rectangle.
// Apparently, Debug is a trait.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}