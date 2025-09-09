extern crate factrs3;

use factrs3::fact;

fn main() {
    let s = fact(150000);

    println!("Hello, world!");
    //println!(fact(5))
    println!("{}", s.to_string().len());
}
