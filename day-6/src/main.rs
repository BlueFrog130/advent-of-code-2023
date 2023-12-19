use std::ops::Range;

fn main() {
    let input = include_str!("input.txt");

    let mut lines = input.lines();

    let l1 = lines.next().unwrap();
    let l2 = lines.next().unwrap();

    // Pairs of time (ms) and distance (mm)
    let data = l1
        .split_whitespace()
        .skip(1)
        .zip(l2.split_whitespace().skip(1))
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()));

    let winning_ranges = data.map(|(t, d)| winning_range(t, d)).collect::<Vec<_>>();

    winning_ranges
        .iter()
        .for_each(|r| println!("{:?}, {}", r, r.end - r.start + 1));

    let result = winning_ranges
        .iter()
        .map(|r| r.end - r.start + 1)
        .product::<i64>();

    println!("Part 1: {}", result);

    let time = l1
        .split_whitespace()
        .skip(1)
        // Join rest of elements
        .fold(String::new(), |mut acc, x| {
            acc.push_str(x);
            acc
        })
        .parse::<i64>()
        .unwrap();

    let distance = l2
        .split_whitespace()
        .skip(1)
        // Join rest of elements
        .fold(String::new(), |mut acc, x| {
            acc.push_str(x);
            acc
        })
        .parse::<i64>()
        .unwrap();

    let result = winning_range(time, distance);

    println!("Part 2: {:#?}", result.end - result.start + 1);
}

fn winning_range(t: i64, d: i64) -> Range<i64> {
    let mut min = i64::MAX;
    let mut max = i64::MIN;
    for i in 0..(t - 1) {
        // Button gets held for i
        // Distance is i * (t - i)
        let dist = i * (t - i);

        // println!("i: {}, t: {}, dist: {}", i, t, dist);

        // If not a winner, continue
        if dist <= d {
            continue;
        }

        // If a winner, check if better than current
        if i < min {
            min = i;
        }

        if i > max {
            max = i;
        }
    }

    min..max
}
