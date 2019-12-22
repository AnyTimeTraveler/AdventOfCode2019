use std::fs;

#[allow(dead_code)]
mod intcode_computer;

mod d09;

fn main() {
    let input = fs::read_to_string("input/09").expect("Error");
//    let lines = input.split("\n").collect();
//    d07::process(lines);
    d09::process(input);
}
