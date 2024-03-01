enum Food {
    Apple,
    Orange,
    Grapes,
    Pineapple,
}

fn print_foods(food: Food) {
    match food {
        Food::Apple => {
            println!("Good Food");
        }

        Food::Orange => {
            println!("Nice Food");
        }

        Food::Grapes => {
            println!("Best Food");
        }

        Food::Pineapple => {
            println!("Tasty Food");
        }
    }
}
fn main() {
    println!("Enums!");

    print_foods(Food::Apple);
    print_foods(Food::Orange);
    print_foods(Food::Grapes);
    print_foods(Food::Pineapple);
}

//// BASIC ENUMS /////////

// enums are custom data types
