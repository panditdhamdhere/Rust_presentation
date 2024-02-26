fn main() {
    println!("Reference!");

    let x = 10;
    let y = &x; // -> reference
    let z = &x;
    println!("x is {}", x);
    println!("y is {}", y); // refered
    println!("z is {}", z); // refered

    // thus we can however you want!
    // & is notation mark of references in rust
}
