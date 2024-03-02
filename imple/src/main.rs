struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
 fn print_desc(&self) {
    println!("Rectangle: {} x {}", self.width, self.height);
}

fn is_square(&self) -> bool {
    self.width == self.height
}


}
fn main() {
    println!("Hello, imple!");

    let my_rectangle = Rectangle{width: 10, height: 20};
    my_rectangle.print_desc();
    println!("Is my rectangle a square? {}", my_rectangle.is_square());

}
//
