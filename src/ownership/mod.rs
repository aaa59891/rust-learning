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

pub fn copy_primitive_type() {
    let x = 5;
    let y = x;
    // although x and y's value is the same 5
    println!("x: {}, y: {}", x, y);

    // in stack, there are two values
    // because this is the primitive type, the address stores the content(5) directly
    // means, y's pointer doesn't point to x's address, it points to the value 5 directly
    println!("&x: {:p}, &y: {:p}", &x, &y);
}

pub fn copy_non_primitive_type() {
    // two different addresses but point to the same heap memory
    let x = String::from("test");
    println!("x's address: {:p}, x: {}", &x, x);
    let y = x;
    println!("y's address: {:p}, y: {}", &y, y);

    /*
    compile error because of rule: There can only be one owner at a time.
    let x = String::from("test");: x -> "test"
    let y = x;: x move to y, rust treat x as invalidate, y -> "test"
    similar as shallow copy, only copy a new address but points to the same heap memory
    but because rust invalidates the original variable, it's usually called `move`, x was moved into y
    */
    // println!("x: {}, y: {}", x, y);
}
