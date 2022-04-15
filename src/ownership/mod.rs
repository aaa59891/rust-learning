/*
Each value in Rust has a variable that’s called its owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.
*/

pub fn different_str() {
    // we already know the content at compile time, it's more efficient than String type
    let mut string_literal = "test";

    println!("string_literal before reassign: {}", string_literal);
    // the string isn't really mutatable, we assign a new string literal to the variable directly
    string_literal = "test new";

    println!("string_literal after reassign: {}", string_literal);

    // the system allocates the memory at runtime, the size isn't fixed
    let mut str = String::from("test");
    str.push_str("new");
    println!("str: {}", str);
}
