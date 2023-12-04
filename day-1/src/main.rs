use std::collections::HashMap;

static NUMBER_WORDS: [&str; 18] = [
    "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
    "eight", "8", "nine", "9",
];

fn main() {
    let input = include_str!("input.txt");

    // Part 1
    let values = input.lines().map(|line| {
        let mut iter = line.chars().filter(|c| c.is_digit(10));
        let first = iter.next().unwrap();
        let last = iter.last().unwrap_or(first);
        [first, last]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    });

    println!("Sum 1: {}", values.sum::<i32>());

    // Part 2
    let mut values = Vec::new();

    for line in input.lines() {
        let mut min_map = HashMap::new();
        let mut max_map = HashMap::new();

        for &word in NUMBER_WORDS.iter() {
            let index = line.find(word).map(|i| i as i32).unwrap_or(-1);
            if min_map.contains_key(&word) {
                continue;
            }
            min_map.insert(word, index as i32);
        }

        for &word in NUMBER_WORDS.iter() {
            let index = line.rfind(word).map(|i| i as i32).unwrap_or(-1);
            if max_map.contains_key(&word) {
                continue;
            }
            max_map.insert(word, index as i32);
        }

        let first_word = *min_map
            .iter()
            .filter(|(_, &v)| v != -1)
            .min_by_key(|(_, &v)| v)
            .unwrap()
            .0;

        let second_word = *max_map.iter().max_by_key(|(_, &v)| v).unwrap().0;

        let value = [first_word, second_word]
            .iter()
            .map(|&word| map_to_value(word))
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        values.push(value);
    }

    println!("Sum 2: {}", values.iter().sum::<i32>());
}

fn map_to_value(word: &str) -> char {
    match word {
        "one" | "1" => '1',
        "two" | "2" => '2',
        "three" | "3" => '3',
        "four" | "4" => '4',
        "five" | "5" => '5',
        "six" | "6" => '6',
        "seven" | "7" => '7',
        "eight" | "8" => '8',
        "nine" | "9" => '9',
        _ => '0',
    }
}
