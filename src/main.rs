mod shape;
mod type_trait;
mod complex_type;

use std::string::String;
use shape::square::Square;
use shape::triangle::Triangle;
use shape::run::Run;
use type_trait::demo_type::DemoType;
use type_trait::comparable::Comparable;
use complex_type::complex::Complex;
// use enigo::*;

fn main() {
    let squre = Square::new(3, 5);
    let triangle = Triangle::new(4, 5);

    let name1 = "Hello".to_string();
    let name2 = "World".to_string();
    let demotype = DemoType::new(name1, 10);
    let demotype_sec = DemoType::new(name2, 20);

    demotype.print_name();
    println!("Compare Result {}", demotype.compare(&demotype_sec));
    // println!("Compare Result {}", demotype.compare(&demotype_sec));

    println!("Hello, square! {}", squre.calculate());
    println!("Hello, triangle! {}", triangle.calculate());

    // let path = "/tmp/dat";
    // println!("{}", read_file(path));

    let com1: Complex<i32> = Complex::default();
    let com2 = Complex::new(1, 3);

    let com3 = com1 + com2;
    println!("Com3 is re {} im {}", com3.re, com3.im)
}

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}
