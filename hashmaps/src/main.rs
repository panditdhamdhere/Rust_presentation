use std::collections::HashMap; // -> way to import standerd library !

fn main() {
    println!("Hashmap!");

    // declaring new hashmap as a marks variable...
    let mut marks: HashMap<&str, i32> = HashMap::new();

    // adding value to the hashmap...
    marks.insert("Rust", 75);
    marks.insert("Solidity", 95);
    marks.insert("JavaScript", 70);
    marks.insert("Clarity", 60);
    marks.insert("Sway", 50);
    marks.insert("Cadence", 30);

    println!("{:?}", marks);

    // find the length of programming languages/
    println!(
        "How many programming languages you studied - {}",
        marks.len()
    );

    // match value -> using match keaword
    match marks.get("Solidity") {
        Some(mark) => println!("you got {} marks in Solidity", mark),
        None => println!("You did not studied this programming language"),
    }

    // remove the value
    marks.remove("Sway");
    println!(
        "Languages remains - {}, Remaining languages are - {:?}",
        marks.len(),
        marks
    );

    // loop through hashmap

    for (language, mark) in &marks {
        println!("For {}, you got {} marks", language, mark);
    }

    // check the value
    println!("Did you studied Solidity? {}" , marks.contains_key("Solidity"));


}
