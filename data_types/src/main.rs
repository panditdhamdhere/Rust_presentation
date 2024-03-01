fn main() {
    println!("Hello data types!");

    // data types in rust

    // scalar data types -> (Types where we store single values)
    // integer, float, boolean, char

    // integer length -> signed or unsigned
    // signed bit -> i8, i16, i32, i64, i128 -> predifined use arch
    // unsigned bit -> u8, u16, u32, u64, u128

    // int
    let no = 2;
    println!("no is {}", no);

    // bool
    let is_rust = true;
    println!("is_rust is {}", is_rust);

    // character
    let chart = 'a';
    println!("letter is {}", chart);

    // float
    let dec = 79.89;
    println!("dec is {}", dec);

    // compound data types -> (Types where we store multiple data at a time);
    // tuple, arrays, disctionary, enums, structs

    // tuples
    let mut tup: (i32, u8, i32, i32, f32) = (1, 2, 3, 4, 5.6);
    println!("tup is {:?}", tup);
    println!("{}", tup.1);
    // you cant add value in tuples

    // change value
    tup.0 = 89;
    println!("tup is {:?}", tup);
    // value changed; tuple is immutable by default

    // arrays
    let mut arr = [1, 2, 3, 4, 5];
    println!("arr is {:?}", arr);
    println!("arr length is {}", arr.len());
    println!("arr first value is {}", arr[0]);

    // change value
    arr[0] = 76;
    println!("arr is {:?}", arr);
    // value changed; array is mutable by default

    
}

// custom data types in Rust 
// enums 
// structs
// traits
// generics
