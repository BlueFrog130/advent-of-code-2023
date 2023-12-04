use std::collections::HashMap;

struct Card {
    pub id: u8,
    pub winning: Vec<u8>,
    pub current: Vec<u8>,
}

impl Card {
    pub fn parse(line: &str) -> Card {
        let mut split = line.split(':');
        let title = split.next().unwrap().split_whitespace();
        let id = title.last().unwrap().parse::<u8>().unwrap();

        let mut rest = split.next().unwrap().split('|');
        let winning = rest
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let current = rest
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        Card {
            id,
            winning,
            current,
        }
    }

    pub fn points(&self) -> u16 {
        let mut points = 0;
        for x in self.current.iter() {
            if self.winning.contains(x) {
                if points == 0 {
                    points += 1;
                } else {
                    points *= 2;
                }
            }
        }
        points
    }

    pub fn matching(&self) -> u8 {
        let mut matching = 0;
        for x in self.current.iter() {
            if self.winning.contains(x) {
                matching += 1;
            }
        }
        matching
    }
}

fn main() {
    let input = include_str!("input.txt");

    let cards = input.lines().map(Card::parse).collect::<Vec<Card>>();

    // Part 1
    let points = cards.iter().map(|x| x.points()).sum::<u16>();

    println!("Part 1: {}", points);

    // Part 2
    let mut instances = cards.iter().fold(HashMap::new(), |mut map, card| {
        map.insert(card.id, 1);
        map
    });

    for card in cards.iter() {
        let matching = card.matching();
        let copies = *instances.get(&card.id).unwrap();
        for i in 1..(matching + 1) {
            let next_card = card.id + i;
            let instance_value = instances.get(&next_card).unwrap();
            instances.insert(next_card, instance_value + copies);
        }
    }

    println!("Part 2: {}", instances.values().sum::<u32>());
}
