// main function is entry point of your program!
fn main() {
    println!("Hello, Functions!");

    first_fn();

    second_fn(12);

    third_fn(10, 'r');

    exp();

    let p = return_value();

    println!("The value of p is: {}", p);
}

/////// MAIN END ///////

///////////////////////////////////////////////////
//////////////////////Functions////////////////////
//////////////////////////////////////////////////

fn first_fn() {
    println!("Hello, I am first fn!");
}

// pass single parameter
fn second_fn(x: i32) {
    println!("The value of x is: {}", x);
}

// pass multiple parameters -> example of statement
fn third_fn(x: i32, y: char) {
    println!("The valu of x is: {}", x);
    println!("The valu of y is: {}", y);
}

// expressions -> it doesnt return anything
fn exp() {
    let y = {
        let x = 9;
        x + 1
    };

    println!("The value of y is: {}", y);
}

// return value from function;
fn return_value() -> i32 {
    55 + 44
}

// expressions -> it doesnt return anything

