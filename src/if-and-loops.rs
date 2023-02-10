fn main() {
    let number = 3;

    // if conditions don't need braces in Rust.
    // Rust actually discourages the use of braces in ifs.
    // Rust will not try to convert other values to a bool.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // Ternary statements exist in Rust. This is how you write them.
    // Using values of different types will create an error.
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    let mut loop_value = 4;
    // This creates an infinite loop.
    // Infinite loops must be exited by using a break or return.
    // Rust will not catch infinite loops without break or return at compile time!
    loop {
        println!("loop value {loop_value}");
        loop_value *= 2;
        if loop_value > 10 {
            break;
        }
    }

    let mut counter = 0;
    // Loops can also be used to calculate a value.
    let result = loop {
        counter += 1;

        if counter == 10 {
            // This break will return the result of the loop.
            break counter * 2;
        }
    // It's important to place the semicolon, if you assign a value to a variable.
    };

    println!("The result is {result}");


    let mut count = 0;
    // Rust has labels for loops.
    // The leading ' character is important to mark it as a label.
    // Labels only work on loops.
    // Labelled loops can not assign their value to a variable.
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // Like if, while don't need braces.
    // The result of a while loop can also be assigned to a variable.
    // However, the type of a while result is `()`.
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];
    // for loops work like this.
    // They're comparable to foreach loops in C#.
    for element in a {
        println!("the value is: {element}");
    }

    // They can also be used with ranges.
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}