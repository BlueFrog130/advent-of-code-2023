use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use rayon::prelude::*;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    left: usize,
    right: usize,
}

fn main() {
    let input = include_str!("input.txt");

    let mut lines = input.lines();

    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(Direction::from_char)
        .collect::<Vec<_>>();

    let nodes = lines
        .skip(1)
        .map(|line| {
            let mut parts = line.split(" = ");
            let name = parts.next().unwrap();
            let mut lr = parts.next().unwrap()[1..9].split(", ");
            (name, (lr.next().unwrap(), lr.next().unwrap()))
        })
        .collect::<Vec<_>>();

    let nodes = nodes
        .iter()
        .map(|node| {
            let left = nodes.iter().position(|n| n.0 == node.1 .0).unwrap();
            let right = nodes.iter().position(|n| n.0 == node.1 .1).unwrap();
            Node {
                name: node.0.to_string(),
                left,
                right,
            }
        })
        .collect::<Vec<_>>();

    let mut current = nodes.iter().position(|n| n.name == "AAA").unwrap();
    let mut direction = 0;
    let mut steps: u64 = 0;
    loop {
        let node = &nodes[current];
        if node.name == "ZZZ" {
            break;
        }
        // println!("{:?}", node);
        // println!("{:?}", directions[direction]);
        match directions[direction] {
            Direction::Left => current = node.left,
            Direction::Right => current = node.right,
        }
        steps += 1;
        direction = (direction + 1) % directions.len();
    }

    println!("Part 1: {}", steps);

    let current = nodes
        .iter()
        .enumerate()
        .filter_map(|(i, n)| match n.name.ends_with("A") {
            true => Some(i),
            false => None,
        })
        .collect::<Vec<_>>();

    let occurances = Arc::new(Mutex::new(HashMap::<String, Vec<i32>>::new()));

    // loop {
    //     if current.iter().all(|n| nodes[*n].name.ends_with("Z")) {
    //         break;
    //     }
    //     let mut next_current = Vec::new();
    //     for &n in &current {
    //         let node = &nodes[n];
    //         match directions[direction] {
    //             Direction::Left => next_current.push(node.left),
    //             Direction::Right => next_current.push(node.right),
    //         }
    //     }
    //     current = next_current;
    //     steps += 1;
    //     direction = (direction + 1) % directions.len();
    // }

    let iterations: u64 = 10_000_000_000;

    current.par_iter().for_each(|&n| {
        let start_node = &nodes[n];
        let mut node = &nodes[n];
        let mut direction = 0;
        occurances
            .lock()
            .unwrap()
            .insert(start_node.name.clone(), Vec::new());
        for i in 0..iterations {
            // Print update every 100_000 iterations
            if i % 100_000 == 0 {
                println!(
                    "{}: {}, occuraces = {}",
                    start_node.name,
                    i,
                    occurances
                        .lock()
                        .unwrap()
                        .get(&start_node.name)
                        .unwrap()
                        .len()
                );
            }
            if node.name.ends_with("Z") {
                let mut occurances = occurances.lock().unwrap();
                occurances.get_mut(&start_node.name).unwrap().push(i as i32);
            }

            match directions[direction] {
                Direction::Left => node = &nodes[node.left],
                Direction::Right => node = &nodes[node.right],
            }

            direction = (direction + 1) % directions.len();
        }
    });

    let occurances = occurances.lock().unwrap();
    // Find values that are in all occurances
    let mut values = occurances.values();
    let mut common = values.next().unwrap().iter().collect::<HashSet<_>>();

    for value in values {
        common = common
            .intersection(&value.iter().collect::<HashSet<_>>())
            .map(|&v| v)
            .collect::<HashSet<_>>();
    }

    println!("{:#?}", common);

    println!("Part 2: {}", common.iter().next().unwrap());
}
