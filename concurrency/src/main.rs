// importing package

use std::thread;
use std::time::Duration;

fn main() {
    println!("Concurrency!");


//create a new thread to run parallay with main fn
    //spawn

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

     // code run from main fn 
     /////////They share memory ////////

     for i in 1..5 {
         println!("hi number {} from the main thread!", i);
         thread::sleep(Duration::from_millis(1));
     }
}
// threads -> processs
// concurrency -> processs -> multiple threads
// aapas mai memory share karega 😂