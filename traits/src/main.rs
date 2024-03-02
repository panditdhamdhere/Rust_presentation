// &self -> for only to read data
// &mut self -> for read and write data
// traits -> similar to interfaces
// traits are like behaviours

trait Printable {
    fn print(&self);  // &self - to use read data -> &mut self - to use read and write data
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
