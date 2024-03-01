struct User {
    name: String,
    company: String,
    designation: String,
    age: u32,
}

// struct Home {
//     address: String,
//     city: String,
//     state: String,
//     house_number: u32,
//     you_lived: bool,
// }

fn main() {
    println!("Struct!");

    let u1 = User {
        // To modify struct values use mut
        name: String::from("Pandit"),
        company: String::from("Op Labs"),
        designation: String::from("Blockchain Engineer"),
        age: 55,
    };

    // u1.age = 56; -> changed the value of age
    println!("u1 name is {}", u1.name);
    println!("u1 company is {}", u1.company);
    println!("u1 designation is {}", u1.designation);
    println!("u1 age is {}", u1.age);

    ////////Struct /////////

    // let uh1 = Home {
    //     address: String::from("123 Main St"),
    //     city: String::from("Pune"),
    //     state: String::from("Maharashtra"),
    //     house_number: 123,
    //     you_lived: true,
    // };

    // println!("uh1 address is {}", uh1.address);
    // println!("uh1 city is {}", uh1.city);
    // println!("uh1 state is {}", uh1.state);
    // println!("uh1 house_number is {}", uh1.house_number);
    // println!("uh1 you_lived is {}", uh1.you_lived);
}

// CLASS & OBJECTS -> just like other programming languages

// oops -> object oriented programming stuff
