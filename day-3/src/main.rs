use std::collections::HashSet;

type Point = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Part {
    pub number: i32,
    pub neighbors: Vec<Point>,
    pub coordinates: Vec<Point>,
}

struct Gear {
    pub coordinates: Point,
    pub neighbors: Vec<Point>,
}

impl Gear {
    pub fn get_ratio(&self, parts: &Vec<Part>) -> i32 {
        // Get all parts that are neighbors
        let neighbor_parts = self
            .neighbors
            .iter()
            .filter_map(|(x, y)| {
                parts
                    .iter()
                    .find(|part| part.coordinates.contains(&(*x, *y)))
            })
            .collect::<HashSet<&Part>>();

        // Debug (119, 1)
        if self.coordinates == (119, 1) {
            println!("Found gear at {:?}", self.coordinates);
            println!("Neighbors: {:?}", neighbor_parts);
        }

        if neighbor_parts.len() != 2 {
            return 0;
        }

        println!(
            "Gear at {:?} has {} neighbors",
            self.coordinates,
            neighbor_parts.len()
        );

        // Multiply all part numbers together
        neighbor_parts
            .iter()
            .map(|part| part.number)
            .fold(1, |acc, x| acc * x)
    }
}

trait CharExt {
    fn is_symbol(&self) -> bool;
}

impl CharExt for char {
    fn is_symbol(&self) -> bool {
        *self != '.' && !self.is_digit(10)
    }
}

static GEAR: char = '*';

fn main() {
    let input = include_str!("input.txt");

    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Building parts
    let mut parts: Vec<Part> = Vec::new();

    for (y, line) in grid.iter().enumerate() {
        let mut part = None;
        for (x, c) in line.iter().enumerate() {
            // Start of part. Need to add left, above, below, top-left and bottom-left
            if c.is_digit(10) && part.is_none() {
                let mut neighbors = Vec::new();

                if x > 0 {
                    neighbors.push((x - 1, y)); // Left
                }

                // Top-left
                if y > 0 && x > 0 {
                    neighbors.push((x - 1, y - 1));
                }

                // Bottom-left
                if y < grid.len() - 1 && x > 0 {
                    neighbors.push((x - 1, y + 1));
                }

                part = Some(Part {
                    number: c.to_digit(10).unwrap() as i32,
                    neighbors,
                    coordinates: vec![(x, y)],
                });
            }
            // Add number to part
            else if c.is_digit(10) && part.is_some() {
                part.as_mut().unwrap().number *= 10;
                part.as_mut().unwrap().number += c.to_digit(10).unwrap() as i32;
                part.as_mut().unwrap().coordinates.push((x, y));
            }

            // Add above and below
            if part.is_some() {
                if y > 0 {
                    part.as_mut().unwrap().neighbors.push((x, y - 1)); // Above
                }
                if y < grid.len() - 1 {
                    part.as_mut().unwrap().neighbors.push((x, y + 1)); // Below
                }
            }

            // Look ahead to see if we're at the end of the part
            if !c.is_digit(10) && part.is_some() {
                part.as_mut().unwrap().neighbors.push((x, y)); // Here
                parts.push(part.unwrap());
                part = None;
            }
        }
        // Adding part at end of line
        if part.is_some() {
            parts.push(part.unwrap());
        }
    }

    // Part 1
    let valid_parts = parts
        .iter()
        .filter(|part| {
            part.neighbors
                .iter()
                .map(|(x, y)| grid[*y][*x])
                .any(|c| c.is_symbol())
        })
        .collect::<Vec<&Part>>();

    let part_numbers = valid_parts
        .iter()
        .map(|part| part.number)
        .collect::<Vec<i32>>();

    let sum = part_numbers.iter().sum::<i32>();

    println!("Sum: {}", sum);

    // Part 2
    let mut gears = grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c == GEAR)
                .map(move |(x, _)| Gear {
                    coordinates: (x, y),
                    neighbors: Vec::new(),
                })
        })
        .collect::<Vec<Gear>>();

    // Add neighbors to gears
    for gear in gears.iter_mut() {
        let (x, y) = gear.coordinates;
        // Left
        if x > 0 {
            gear.neighbors.push((x - 1, y));
        }
        // Right
        if x < grid[0].len() - 1 {
            gear.neighbors.push((x + 1, y));
        }
        // Above
        if y > 0 {
            gear.neighbors.push((x, y - 1));
        }
        // Below
        if y < grid.len() - 1 {
            gear.neighbors.push((x, y + 1));
        }

        // Top-left
        if y > 0 && x > 0 {
            gear.neighbors.push((x - 1, y - 1));
        }

        // Bottom-left
        if y < grid.len() - 1 && x > 0 {
            gear.neighbors.push((x - 1, y + 1));
        }

        // Top-right
        if y > 0 && x < grid[0].len() - 1 {
            gear.neighbors.push((x + 1, y - 1));
        }

        // Bottom-right
        if y < grid.len() - 1 && x < grid[0].len() - 1 {
            gear.neighbors.push((x + 1, y + 1));
        }
    }

    // Modified grid to highlight gears with at least 2 neighboring parts
    let mut grid = grid.clone();

    for gear in gears.iter() {
        let (x, y) = gear.coordinates;
        if gear
            .neighbors
            .iter()
            .filter_map(|(x, y)| {
                parts
                    .iter()
                    .find(|part| part.coordinates.contains(&(*x, *y)))
            })
            .collect::<HashSet<&Part>>()
            .len()
            >= 2
        {
            grid[y][x] = 'X';
        }
    }

    // find all gears with at least 2 neighboring parts
    let gear_ratio_sum = gears.iter().map(|gear| gear.get_ratio(&parts)).sum::<i32>();

    println!("Gear ratio sum: {}", gear_ratio_sum);
}
