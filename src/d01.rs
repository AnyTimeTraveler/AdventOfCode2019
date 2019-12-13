pub fn process(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut counter: i64 = 0;
    for line in lines {
        if line.is_empty() { continue; }
        let cargo: i64 = line.parse().expect("A");
        counter += calc_fuel(cargo);
    }
    println!("{}\n", counter);
}

fn calc_fuel(cargo: i64) -> i64 {
    let fuel = (cargo / 3) - 2;
    if fuel > 0 {
        fuel + calc_fuel(fuel)
    } else {
        0
    }
}
