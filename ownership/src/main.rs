// ownership is Rust's most unique feature, it has deep implications for the language
// It enables to make memory sefety guarantees without needing garbage collector
// it is very important to understand how it works

// MEMORY HANDLING

// memory is managed through a set of rules that the compiler checks -> if any rule is violated, the program will crash

fn main() {
    println!("Rust ownership !");

    first();
}

// Ownership Rules

// Each value in Rust has an owner
// There can only be one owner at a time
// When the owner goes out of scope, the value will be dropped

fn first() {
    let a = 90; // -> example 8 bit memory

    /////// MEMORY SHIFT //////// same memory consumptions

    let b = a; // -> example 8 bit memory

    println!("The value of b is {}", b);

    // ------------STRING ------------//

    let pd = String::from("Pandit Dhamdhere");

    let pk = pd;

    println!("The value of pk is {}", pk);
}

// if there is big value -> with string and array you can use clone() to perform task
