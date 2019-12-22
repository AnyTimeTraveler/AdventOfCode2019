use std::fs;

mod intcode_computer;
mod d07;

fn main() {
    let input = fs::read_to_string("input/07").expect("Error");
    let lines = input.split("\n").collect();
    d07::process(lines);
}
