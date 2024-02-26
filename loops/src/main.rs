fn main() {
    println!("Loops!");

    first();

    while_loop();

    for_loop();
}

// types of loops in rust - 3
// loop
// while
// for

fn first() {
    loop {
        println!("Hello, simple loop!");
        break;
    }
}

// while loop
fn while_loop() {
    let mut x = 1;
    while x < 10 {
        println!("x = {}", x);
        x += 1;
    }
}

// for loop
fn for_loop() {
    for pandit in 1..10 {
        println!("pandit = {}", pandit);
    }
}