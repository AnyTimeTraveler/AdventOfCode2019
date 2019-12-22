use std::fs;

mod intcode_computer;
mod d08;

fn main() {
    let input = fs::read_to_string("input/08a").expect("Error");
//    let lines = input.split("\n").collect();
//    d07::process(lines);
    d08::process(input);
}
