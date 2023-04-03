// Lifetimes are a kind of generic.
// Instead of specifying what a types does, it tells us how long it will be valid.
// Every variable in Rust has a lifetime, which is the scope in which it is valid.
// Usually, the lifetime is inferred.

// The reason for lifetimes is to prevent dangling references.

fn main() {
    // Rust allows declaring a variable witout type annotation and value.
    // Since it can infer its type from later assignment, it only needs to check that the value is not used before anything is assigned.
    // The book calls this lifetime 'a (until the end of main).
    let r;

    {
        // Because x is declared in a scope that expires before using r, this causes a compiler error.
        // The book calls this lifetime 'b (until the end of this scope).
        let x = 5;
        r = &x;
        println!("r: {r}");
    }

    let a = String::from("asdf");
    {
        let b = String::from("xyz");
        // Because b is only valid until the end of this scope, result is also valid until then.
        // This is because the lifetime expressed by 'a is the smaller of both arguments (in this case b).
        // This means that result would not be valid if used outside of this scope (e.g. when result was declared before the scope).
        let result = longest(a.as_str(), b.as_str());
        println!("The longest is {result}");
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // static is a special lifetime.
    // It indicates that the value can live for the lifetime of the program.
    let s: &'static str = "I have a static lifetime.";
}

// Because the return value of this function is a borrowed value, the compiler requires it to define a lifetime.
// This lifetime tells the compiler that the return value should be valid for as long as left or right.
// The lifetime does not change how long a value is valid.
// Instead it tells the compiler how the different lifetimes relate to each other.
// A lifetime on parameters is called an input lifetime.
// A lifetime on the return value is called an output lifetime.
fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() > right.len() {
        left
    }
    else {
        right
    }
}

// An instance of ImportantExerpt may not outlife the value of part.
struct ImportantExcerpt<'a> {
    part: &'a str
}

// Lifetime ellision is something the Rust compiler does to find deterministic lifetime uses.
// A programmer may omit the lifetime, when Rust can infer the lifetime in such cases.
// If the lifetime use is still ambiguous, the compiler tells you to specify the lifetime explicitly.

// Declaring the lifetime in the impl block is required, but we don't need to use the lifetime because of the first elision rule.
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // Because one of the parameters is self, the output lifetime is the lifetime of this instance.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
