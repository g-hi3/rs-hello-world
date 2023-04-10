use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {user_pref1:?} gets {giveaway1:?}");

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {user_pref2:?} gets {giveaway2:?}");

    // Closure parameters can also be annotated explicitly.
    // At this point, it's unclear how lifetimes come to play with closures.
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(std::time::Duration::from_secs(2));
        num
    };

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    // The compiler decides how the variables are being captured based on how they're used in the closure.
    // In this case, the value is immutably borrowed, because it's just being printed.
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    // Closures can be called like functions, when they're stored in a variable.
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    // In this case, it captures the variable mutably, because it mutates `list`.
    // It's not allowed to use `list` immutably (to print) between the declaration of `borrows_mutably` and the call to it.
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    // The keyword move tells the compiler that the closure should take ownership of the captured variable.
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    
    // Closures implement three kinds of traits:
    //  - FnOnce: implemented by all closures
    //  - FnMut: implemented by closures that use mutable captured variables, can be called multiple times
    //  - Fn: implemented by closures that don't mutate captured variables, they can also be called multiple times

    // Functions can be used as closures by only using the function name, just like in C#.
}