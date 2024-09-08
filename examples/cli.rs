use std::env::args;

fn main() {
    let num = args().nth(1).unwrap();
    let formula = homo::roar(&num).unwrap();

    println!("{formula}");
}
