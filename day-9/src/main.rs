type Diffs = Vec<Vec<i32>>;

fn main() {
    let input = include_str!("input.txt");

    let lines = input.lines().map(|line| {
        line.split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });

    let diffs = lines.map(|line| diffs(line)).collect::<Vec<_>>();

    let next_values = diffs.iter().map(|diff| next_value(diff));

    let sum = next_values.sum::<i32>();

    println!("Part 1: {}", sum);

    let prev_values = diffs.iter().map(|diff| prev_value(diff));

    let sum = prev_values.sum::<i32>();

    println!("Part 2: {}", sum);
}

// Gets line diffs until line is all 0
fn diffs(nums: Vec<i32>) -> Diffs {
    let mut diffs = Vec::new();
    let mut line = nums;
    while line.iter().any(|&n| n != 0) {
        diffs.push(line.clone());
        line = line_diff(&line);
    }
    diffs.push(line);
    diffs
}

fn line_diff(nums: &Vec<i32>) -> Vec<i32> {
    let mut diffs = Vec::new();
    for i in 0..nums.len() - 1 {
        diffs.push(nums[i + 1] - nums[i]);
    }
    diffs
}

fn next_value(diffs: &Diffs) -> i32 {
    let mut add = 0;
    for diff in diffs.iter().rev().skip(1) {
        let last = diff.last().unwrap();
        add = last + add;
    }
    add
}

fn prev_value(diffs: &Diffs) -> i32 {
    let mut sub = 0;
    for diff in diffs.iter().rev().skip(1) {
        let first = diff.first().unwrap();
        sub = first - sub;
    }
    sub
}
