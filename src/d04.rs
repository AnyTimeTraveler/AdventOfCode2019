pub fn process(input: Vec<&str>) {
    let mut counter = 0;

    for i in input[0].parse().unwrap()..input[1].parse().unwrap() {
        if has_adjacent_digits(i) && no_decreasing_digits(i) && has_double_repetition(i) {
            counter += 1;
        }
    }
    println!("{}", counter);
}

fn has_adjacent_digits(i: u32) -> bool {
    let string = i.to_string();
    string.chars().fold((false, 'x'), |prev, curr| {
        match prev {
            (true, _) => prev,
            (false, p) => (p == curr, curr)
        }
    }).0
}

fn no_decreasing_digits(i: u32) -> bool {
    let string = i.to_string();
    string.chars().map(|e| { e.to_digit(10).unwrap() }).fold((true, 0), |prev, curr| {
        match prev {
            (false, _) => prev,
            (true, p) => (p <= curr, curr)
        }
    }).0
}

fn has_double_repetition(i: u32) -> bool {
    let string = i.to_string();
    let mut reps = 0;
    let mut prev = 'x';
    for c in string.chars() {
        if c == prev {
            reps += 1;
        } else if reps == 2 {
            return true;
        } else {
            prev = c;
            reps = 1;
        }
    }
    reps == 2
}