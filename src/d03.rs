
pub fn process(input: Vec<&str>) {
//    Testing:
//    let path = draw_path("U8");
//    let first = path.first().unwrap();
//    let last = path.last().unwrap();
//
//    println!("({},{}) ({},{})", first.0, first.1, last.0, last.1);


    let path_one = draw_path(input[0]);
    let path_two = draw_path(input[1]);

    println!("Lengths: {} {}", path_one.len(), path_two.len());
    let intersections: Vec<(i32, i32)> = path_one.intersect(&path_two);
    println!("Length: {}", intersections.len());
    let distances: Vec<i32> = intersections.iter().map(|e| { e.0.abs() + e.1.abs() }).collect();
    let smallest = distances.into_iter().fold(None, |min, x| match min {
        None => Some(x),
        Some(y) => Some(if x < y { x } else { y }),
    });
    println!("Closest intersection: {}", smallest.unwrap());

    let intersection = path_one.iter().fold((None, 0u32), |old, new| {
        match old {
            (None, count) => (if intersections.contains(new) { Some(new) } else { None }, count + 1),
            (Some(x), count) => old
        }
    });
    let length = intersection.1;
    let intersection = intersection.0.unwrap();

    let otherdistance = path_two.iter().fold((false, 0u32), |count, val| {
        match count {
            (false, c) => (intersection == val, c + 1),
            (true, c) => count
        }
    });
    println!("{}+{}={}", length, otherdistance.1, length + otherdistance.1);
}

trait Intersectable {
    fn intersect(&self, other: &Vec<(i32, i32)>) -> Vec<(i32, i32)>;
}

impl Intersectable for Vec<(i32, i32)> {
    fn intersect(&self, other: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        let mut result: Vec<(i32, i32)> = Vec::new();

        for item in other.iter() {
            if self.contains(item) {
                result.push(*item);
            }
        }
        result
    }
}

fn draw_path(input: &str) -> Vec<(i32, i32)> {
    let mut path: Vec<(i32, i32)> = Vec::new();
    let directions: Vec<&str> = input.split(',').collect();
    let mut pos = (0i32, 0i32);

    for elem in directions {
        let amount: u32 = elem[1..].parse().unwrap();
        for _ in 1..amount + 1 {
            match elem.chars().nth(0).unwrap() {
                'R' => pos.1 += 1,
                'L' => pos.1 -= 1,
                'U' => pos.0 += 1,
                'D' => pos.0 -= 1,
                _ => println!("Unknown: {}", elem)
            }
            path.push(pos);
        }
    }
    path
}