// ownership is Rust's most unique feature, it has deep implications for the language
// It enables to make memory sefety guarantees without needing garbage collector
// it is very important to understand how it works

// MEMORY HANDLING

// memory is managed through a set of rules that the compiler checks -> if any rule is violated, the program will crash

fn main() {
    println!("Rust ownership !");
    let mut s = String::from("hello");
    println!("{}", s);
    s.push_str(", world!");
    println!("{}", s);
}

// Ownership Rules

// Each value in Rust has an owner
// There can only be one owner at a time
// When the owner goes out of scope, the value will be dropped
