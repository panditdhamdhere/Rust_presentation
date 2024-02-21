enum CarTypes {
    Hatchback,
    Sedan,
    Convertible,
    Truck,
}

fn print_cars(car: CarTypes) {
    match car {
        CarTypes::Hatchback => {
            println!("Small car in a segment");
        }

        CarTypes::Sedan => {
            println!("Luxery car in a segment");
        }

        CarTypes::Convertible => {
            println!("Big car in a segment");
        }

        CarTypes::Truck => {
            println!("Truck in a segment");
        }
    }
}
fn main() {
    println!("Enums!");

    print_cars(CarTypes::Hatchback);
    print_cars(CarTypes::Sedan);
    print_cars(CarTypes::Convertible);
    print_cars(CarTypes::Truck);
    
}
