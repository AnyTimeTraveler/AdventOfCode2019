use std::fs;

#[allow(dead_code)]
mod intcode_computer;

mod d11;

fn main() {
    let input = fs::read_to_string("input/11").expect("Error");
//    let lines = input.split("\n").collect();
    d11::process(input);
}
