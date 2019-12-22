use std::process::exit;

pub fn process(input: Vec<&str>) {
    let map: Vec<Vec<bool>> = input.iter().map(|line| { line.chars().map(|c| { c == '#' }).collect() }).collect();
    let mut locations = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, pos) in row.iter().enumerate() {
            if *pos {
                locations.push((x, y));
            }
        }
    }
    let locations = locations;
    let max_size: i32 = if map.len() > map[0].len() { map.len() } else { map[0].len() } as i32;

    let mut best_location = ((0, 0), 0);

    let mut result_map: Vec<Vec<u32>> = Vec::new();
    for row in &map {
        result_map.push(row.iter().map(|_| { 0 }).collect());
    }

    for (base_x, base_y) in locations {
        println!("Scanning from: {} {}", base_x, base_y);
        let mut local_map = Vec::new();

        for row in &map {
            local_map.push(row.as_slice().to_vec());
        }

        // hide own asteroid
        local_map[base_y][base_x] = false;

        let mut asteroids = 0;
        for max_offset in 0..=max_size {
            for of_y in (-max_offset)..max_offset {
                for of_x in (-max_offset)..max_offset {
                    let pos_y = (base_y as i32 + of_y) as usize;
                    if let Some(row) = local_map.get(pos_y) {
                        let pos_x = (base_x as i32 + of_x) as usize;
                        if let Some(true) = row.get(pos_x) {
                            asteroids += 1;
                            // found one!
//                            println!("Found: {} {} at offset {} {}", pos_x, pos_y, of_x, of_y);

                            noop();
                            // eliminate all asteroids it blocks
                            for multiplier in 1..100 {
                                let pos_y = (base_y as i32 + of_y * multiplier) as usize;
                                if let Some(row) = local_map.get(pos_y) {
                                    let pos_x = (base_x as i32 + of_x * multiplier) as usize;
                                    if let Some(ast) = row.get(pos_x) {
                                        if *ast {
                                            local_map[pos_y][pos_x] = false;
//                                            println!("Removed one!");
                                        }
                                    } else {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        result_map[base_y][base_x] = asteroids;
        if asteroids > best_location.1 {
            best_location = ((base_x, base_y), asteroids);
        }
    }
    for row in result_map {
        for point in row {
            if point > 0 {
                print!("{:4} ", point);
            } else {
                print!("{:4} ", "");
            }
        }
        println!();
    }
    println!("Best location: ({} {}) can see {} asteroids", (best_location.0).0, (best_location.0).1, best_location.1);
}

fn noop(){

}
