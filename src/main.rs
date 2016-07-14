extern crate fact_rust;

use fact_rust::fact;

fn main() {
    let s = fact(150000);

    println!("{}", s.to_string().len());
}
