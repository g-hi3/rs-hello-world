fn main() {
    let coin = Coin::Dime;
    let value = value_in_cents(coin);
    println!("A dime is worth {value} cents.");

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    do_roll();
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    // The Rust compiler checks that the match expression is exhaustive.
    // This means that it will cause an error if there's an enum value not covered by the match.
    match coin {
        // The match arms use the same syntax as C# (=>) as opposed to Java (->).
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Contained values of an enum can be used like this.
        // Multiple statements can be included when using the curly braces.
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // By using Option, a possibly unset value can be handled.
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
    // Clippy suggested this instead:
    // x.map(|i| i + 1)
    // This syntax seems to describe a delegate.
}

fn do_roll() {
    // Since move_player takes a u8 parameter, dice_roll is inferred to be a u8 value.
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // This creates a new variable called other that is only valid for this arm.
        other => move_player(other)
        // The _ discard pattern (similar to C#) can be used as well.
        // The discard underscore must be the last arm.
        // If an arm should do nothing, this line could be used:
        // _ => ()
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}