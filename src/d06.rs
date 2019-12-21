extern crate vec_tree;

use std::collections::HashMap;

use self::vec_tree::{Index, VecTree};

pub fn process(input: Vec<&str>) {
    let mut map = HashMap::new();
    let mut planets = VecTree::new();
    let root_node = planets.insert_root("ROOT");

    for line in &input {
        let names = line.split(')');
        for name in names {
            if !map.contains_key(name) {
                map.insert(String::from(name), planets.insert(name, root_node));
            }
        }
    }
    for line in &input {
        let names: Vec<&str> = line.split(')').collect();
        planets.append_child(*map.get(names[0]).unwrap(), *map.get(names[1]).unwrap())
    }

    let sum: usize = map.values().map(|a| {
        let mut current = *a;
        let mut counter = 0;
        loop {
            current = planets.parent(current).unwrap();
            if current == root_node {
                return counter;
            }
            counter += 1;
        }
    }).sum();

    print_children(&planets, root_node);

    println!("{}", sum);

    let src = planets.parent(*map.get("YOU").unwrap()).unwrap();
    let dst = planets.parent(*map.get("SAN").unwrap()).unwrap();

    let mut path = Vec::new();
    decend(&planets, &mut path, src);
    let inbetween = *path.last().unwrap();
    acend(&planets, &mut path, inbetween);
    println!("{}", path.len() - 2);
}

fn decend(planets: &VecTree<&str>, path: &mut Vec<Index>, src: Index) {
    path.push(src);
    let descendants = planets
        .descendants(src)
        .map(|node| planets[node])
        .collect::<Vec<&str>>();
    if descendants.contains(&"SAN") {
        return;
    } else {
        decend(planets, path, planets.parent(src).unwrap());
    }
}

fn acend(planets: &VecTree<&str>, path: &mut Vec<Index>, src: Index) {
    for child in planets.children(src) {
        let descendants = planets
            .descendants(child)
            .map(|node| planets[node])
            .collect::<Vec<&str>>();
        if descendants.contains(&"SAN") {
            path.push(child);
            acend(planets, path, child);
        }
    }
}

fn print_children(tree: &VecTree<&str>, parent: Index) {
    println!("{}", tree.get(parent).unwrap());
    for child in tree.children(parent) {
        println!("\t{}", tree.get(child).unwrap());
    }
    for child in tree.children(parent) {
        print_children(tree, child);
    }
}
