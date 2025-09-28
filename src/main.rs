mod tools;

use crate::tools::*;

fn main() {
    println!("大风起兮云飞扬");
    let x=14;
    println!("x={}",x);
    println!("x^2={}",square(x));
    println!("x^3={}",cube(x));
    println!("{}",multiply(327,950));
}
