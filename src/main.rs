use std::fs;

mod intcode_computer;
mod d05;

fn main() {
    let input = fs::read_to_string("input/05").expect("Error");
    let lines = input.split("\n").collect();
    d05::process(lines);
}
