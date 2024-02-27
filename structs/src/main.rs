struct User {
    name: String,
    company: String,
    designation: String,
    age: u32,
}

fn main() {
    println!("Struct!");

    let u1 = User {
        name: String::from("Pandit"),
        company: String::from("Op Labs"),
        designation: String::from("Blockchain Engineer"),
        age: 55,
    };

    println!("u1 name is {}", u1.name);
    println!("u1 company is {}", u1.company);
    println!("u1 designation is {}", u1.designation);
    println!("u1 age is {}", u1.age);
}
