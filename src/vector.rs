fn main() {
    let v: Vec<i32> = Vec::new();
    // This is called the turbo fish syntax.
    // It allows you to explicitly declare the generic type to be used in a function.
    // This is only necessary, when the variable's type is not explicitly declared and the variable doesn't use functions using the generic type.
    let _v1 = Vec::<i32>::new();
    // The vec macro allows us to create a vector from some values.
    // This lets Rust infer the generic type.
    let v2 = vec![1, 2, 3];

    // Iterating a vector looks like this:
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // Using the dereferencing operator, we are able to mutate the value of i, even though it is a reference.
        *i += 50;
    }

    // To add values to the vector, it needs to be mutable.
    let mut mutable_vector = Vec::new();
    // Vector uses push (and pop?).
    mutable_vector.push(5);
    mutable_vector.push(6);
    // let first = &mutable_vector[0];
    // If we were to declare this variable first and then push, before passing the reference to println, the compiler would show an error because of borrowing rules.
    mutable_vector.push(7);
    // println!("The first element is {first}");

    // Values can be referenced using indexing.
    // This will return the reference directly, but can cause the program to panic.
    // Very importantly, indexing starts at 0.
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // The get function returns an Option containing Some(value) or None.
    // This means the program will never panic using get.
    let second: Option<&i32> = v.get(2);
    match second {
        Some(second) => println!("The second element is {second}"),
        None => println!("There is no second element.")
    }

    // Using enums, values of different types can be added to the vector.
    // Of course, the vector only uses SpreadsheetCell, but its contents can be of different types.
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("ok"))];
// When a vector goes out of scope, it will be dropped, along with all the references to its contents.
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}