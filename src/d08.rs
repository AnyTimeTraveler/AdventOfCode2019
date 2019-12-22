use std::process::exit;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn process(input: String) {
    let mut layers = Vec::new();
    let chars: Vec<char> = input.chars().collect();

    let mut i = 0;
    while i + 26 < chars.len() {
        let mut layer = [['0'; WIDTH]; HEIGHT];
        for x in 0..HEIGHT {
            layer[x].copy_from_slice(&chars[i..i + WIDTH]);
            i += WIDTH;
        }
        layers.push(layer);
    }
    let mut fewest_zeros = (count_chars(&layers[0], '0'), &layers[0]);
    for layer in &layers {
        let zeros = count_chars(layer, '0');
        if zeros < fewest_zeros.0 {
            fewest_zeros = (zeros, layer);
        }
    }
    println!("Fewest: {}", fewest_zeros.0);
    println!("Result: {}", count_chars(&fewest_zeros.1, '1') * count_chars(&fewest_zeros.1, '2'));

    let mut final_layer = [['2'; WIDTH]; HEIGHT];

    for layer in &layers {
        for row in 0..HEIGHT {
            for pixel in 0..WIDTH {
                if final_layer[row][pixel] == '2' {
                    final_layer[row][pixel] = layer[row][pixel];
                }
            }
        }
    }
    for row in &final_layer {
        for pixel in row {
            if *pixel == '1' {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

fn count_chars(layer: &[[char; WIDTH]; HEIGHT], number: char) -> usize {
    layer.iter()
        .flat_map(|row| { row.iter() })
        .filter(|c| { **c == number })
        .count()
}