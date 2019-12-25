use std::fs;

#[allow(dead_code)]
mod intcode_computer;

mod d13;

fn main() {
    let input = fs::read_to_string("input/13").expect("Error");
//    let lines = input.split("\n").collect();
    d13::process(input);
}
