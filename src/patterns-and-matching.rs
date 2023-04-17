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
}

// Function parameters can also be patterns.
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: {x} {y}");
}