// &self -> for only to read data

trait Printable {
    fn print(&self);
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Printable for Rectangle {
    fn print(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
    }
}

fn main() {
    println!("Rust Traits!");

    let rect = Rectangle { width: 10, height: 20 };
    rect.print();

}
