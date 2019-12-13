pub fn process(input: Vec<&str>) {
    let numbers: Vec<&str> = input.first().expect("").split(',').collect();
    let numbers: Vec<u32> = numbers.iter().map(|x| -> u32 { x.parse().expect("") }).collect();
    for noun in 0..100 {
        for verb in 0..100 {
            if run(&numbers, noun, verb) == 19690720 {
                println!("100 * {} + {} = {}", noun, verb, 100 * noun + verb);
                return;
            }
        }
    }
}

fn run(numbers: &Vec<u32>, noun: u32, verb: u32) -> u32 {
    let mut values = [0u32; 173];
    for (i, item) in numbers.iter().enumerate() {
        values[i] = *item;
    }
    values[1] = noun;
    values[2] = verb;
    let mut numbers = values;

    let mut index: usize = 0;
    loop {
        match numbers[index] {
            99 => {
                return numbers[0];
            }
            1 => {
                let temp = get(&numbers, index + 1 as usize) + get(&numbers, index + 2 as usize);
                numbers[numbers[index + 3 as usize] as usize] = temp;
            }
            2 => {
                let temp = get(&numbers, index + 1 as usize) * get(&numbers, index + 2 as usize);
                numbers[numbers[index + 3 as usize] as usize] = temp;
            }
            _ => panic!("WTF?!")
        }
        index += 4;
    }
}

fn get(numbers: &[u32], pos: usize) -> u32 {
    *numbers.get(*numbers.get(pos).unwrap() as usize).unwrap()
}