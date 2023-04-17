struct Point {
    x: i32,
    y: i32
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color)
}

enum FriendlyMessage {
    Hello { id: i32 }
}

fn main() {
    let x = Some(5);
    // The match arms take a pattern and evaluate to an expression.
    // The match control structure must be exhaustive.
    // This means, that all cases must be covered.
    // The discard `_` may be used as a sort of default case that matches for all remaining cases.
    let y = match x {
        None => None,
        Some(i) => Some(i + 1)
    };

    // The `if let` control structure can be thought of as a very simple `match` structure.
    // It is used in place where only a single pattern is relevant.
    // Optionally, an `else` block can be used when the remaining cases are relevant in another way.
    // `if let` branches are not being checked for exhaustiveness.
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    // The `if let` can also be mixed and match with `if`, `else if` and `else`.
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        // `age` at this point is a new variable created by the `if let` control structure that shadows the original `age`.
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // Similar to the `if let` control structure is `while let`.
    // It works as if a `while` loop would evaluate an expression each iteration until the pattern doesn't match anymore.
    // The code block will be executed for each matching iteration.
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    // `for` loops also use expressions.
    // In the case of `x in y`, `x` is a pattern.
    let v = vec!['a', 'b', 'c'];
    // Using tuples like this is considered a pattern.
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // The `let` keyword also uses patterns.
    // The compiler makes sure that the expression on the right can be assigned to the variables on the left.
    let (x, y, z) = (1, 2, 3);

    // This is how to call a function with pattern parameters.
    let point = (3, 5);
    print_coordinates(&point);
    print_coordinates(&(18, 22));

    // Patterns can be refutable or irrefutable.
    // Irrefutable patterns are patterns that can not fail to match.
    // An example of this is `let x = 5;`.
    // An example of a refutable pattern is `if let Some(x) = a_value`.
    // Function parameters, `let` statements and `for` loops can only accept irrefutable patterns.
    // There may be compiler error messages that mention refutable or irrefutable patterns.
    // It is important to understand how these kinds of patterns are applied to work with these messages.

    // Literals are considered a kind of pattern syntax.
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Named variables can be used for matching.
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // Here, `y` is a named variable that is assigned by matching any `Some(...)` value.
        // This new variable shadows `y` in the outer scope.
        // 5 will be printed here.
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }
    // 10 will be printed here (for y).
    println!("at the end: x = {x:?}, y = {y}");

    // Multiple patterns can be joined using the `|`.
    let x = 1;
    match x {
        // This arm matches when `x` is either 1 or 2.
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Ranges can be matched with `..=`.
    let x = 5;
    match x {
        // This arm will be executed when `x` is 1, 2, 3, 4 or 5.
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // A struct can be destructured.
    let p = Point { x: 0, y: 7 };
    // The variables created do not have to match the fields of the struct.
    let Point { x: a, y: b } = p;
    // A shorthand version of this creates a new variable with the names of the struct's fields.
    let Point { x, y } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // The same kind of destructuring can be applied to match arms.
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})")
    }

    // Enums can be destructured in a similar way to structs.
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => println!("Move in the x direction {x} and in the y direction {y}"),
        Message::Write(text) => println!("Text message: {text}"),
        // Enum and struct destructuring can also be nested (un-nested in this case?).
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
    }

    // This is another, more complex example of destructuring.
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // In this example, 3 will be discarded.
    foo(3, 4);

    // A discard `_` is used to discard a single value.
    // The `..` can be used to discard the remaining values.
    let x = 1..=4;
    let origin = Point3D { x: 0, y: 0, z: 0 };
    match origin {
        // If any values follow the `..` discard, they will be the last couple of values of the struct, tuple, enum.
        // This is somewhat comparable to list patterns in C#.
        Point3D { x, .. } => println!("x is {x}")
    }

    // Match guards can be used to question some more about the value being matched.
    let num = Some(4);
    let y = 10;
    match num {
        // The `if x % 2 == 0` expresses a condition that wouldn't have been possible to be expressed as a pattern.
        // The downside of this is that the compiler will not check the exhaustiveness of the involved arms.
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        // This is also how we can evaluate values that were shadowed in a previous example.
        Some(n) if n == y => println!("Matched, n = {n}"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    // Destructured values can be assigned in a special way by using the `@` operator.
    let msg = FriendlyMessage::Hello { id: 5 };
    match msg {
        FriendlyMessage::Hello {
            // The newly created variable `id_variable` will contain the matched value for this arm.
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        FriendlyMessage::Hello { id: 10..=12 } => {
            // Since we didn't capture the matched value here, we won't (or don't need to) know what it was.
            println!("Found an id in another range")
        }
        FriendlyMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}

// Function parameters can also be patterns.
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: {x} {y}");
}

// Using `_` in a pattern discards it.
// This is also true for parameters.
// Notice that we still have to define the parameter type.
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}