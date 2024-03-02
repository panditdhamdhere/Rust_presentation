fn main() {
    println!("Reference!");

    let x = 10;

    let y = &x; // -> reference

    let z = &x;
    
    println!("x is {}", x);
    println!("y is {}", y); // refered
    println!("z is {}", z); // refered

    // At any given time, you can have either one mutable reference or any number of immutable references. 

    // References must always be valid.
    // If you break a reference, the program will crash.
    
}


// Slice 


// let a = [1, 2, 3, 4, 5];

// let slice = &a[1..3];

// assert_eq!(slice, &[2, 3]);

