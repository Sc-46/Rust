mod tools;

use crate::tools::*;

fn main() {
    let x=14;
    println!("x={}",x);
    println!("x^2={}",square(x));
    println!("x^3={}",cube(x));
}
