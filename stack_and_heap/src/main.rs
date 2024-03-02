fn main() {
    println!("Stack & Heap!");

    stack_example();
    heap_example();
}


///////////////////Stack /////////////////////

fn stack_example() {
    let x = 5;
    let y = x;
    println!("x: {}", x);
    println!("y: {}", y);
}


///////////////////Heap ///////////////////// -> dynamically allocate
fn heap_example() {
   let mut v = Vec::new(); // -> heap

   v.push(5);
   v.push(600);
   v.push(7000000);

   println!("v: {:?}", v);
}


// stack memory is fixed on compile time -> single data types
// heap memory is fixed on runtime -> multiple data types... arrays, vectors etc -> dynamic allocate -> multiple data types