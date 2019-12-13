use std::fs;

mod d02;

fn main() {
    let input = fs::read_to_string("input/02").expect("Error");
    let lines = input.split("\n").collect();
    d02::process(lines);
}
