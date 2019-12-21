use std::fs;

//mod d01;
//mod d02;
//mod d03;
//mod d04;
//mod d05;
mod d06;

fn main() {
    let input = fs::read_to_string("input/06").expect("Error");
    let lines = input.split("\n").collect();
    d06::process(lines);
}
