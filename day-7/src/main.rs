use core::fmt;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Option<Card> {
        match c {
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'J' => Some(Card::Jack),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Card::Two => '2',
            Card::Three => '3',
            Card::Four => '4',
            Card::Five => '5',
            Card::Six => '6',
            Card::Seven => '7',
            Card::Eight => '8',
            Card::Nine => '9',
            Card::Ten => 'T',
            Card::Jack => 'J',
            Card::Queen => 'Q',
            Card::King => 'K',
            Card::Ace => 'A',
        };
        write!(f, "{}", c)
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum Type {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    cards: [Card; 5],
    bid: u16,
    rank: Type,
}

impl Hand {
    fn from_str(str: &str) -> Self {
        let mut split = str.split_whitespace();
        let cards_iter = split.next().unwrap().chars().map(Card::from_char);
        let bid = split.next().unwrap().parse().unwrap();

        let mut cards = [Card::Two; 5];

        for (i, card) in cards_iter.enumerate() {
            cards[i] = card.unwrap();
        }

        Hand {
            cards,
            bid,
            rank: Self::get_rank(&cards),
        }
    }

    fn get_rank(cards: &[Card; 5]) -> Type {
        let mut counts = [0; 13];

        for card in cards {
            counts[*card as usize] += 1;
        }

        let single = counts.iter().filter(|&&x| x == 1).count();
        let double = counts.iter().filter(|&&x| x == 2).count();
        let triple = counts.iter().filter(|&&x| x == 3).count();
        let quad = counts.iter().filter(|&&x| x == 4).count();
        let quint = counts.iter().filter(|&&x| x == 5).count();

        match (single, double, triple, quad, quint) {
            (5, _, _, _, _) => Type::HighCard,
            (3, 1, _, _, _) => Type::OnePair,
            (_, 2, _, _, _) => Type::TwoPairs,
            (_, 0, 1, _, _) => Type::ThreeOfAKind,
            (_, 1, 1, _, _) => Type::FullHouse,
            (_, _, _, 1, _) => Type::FourOfAKind,
            (_, _, _, _, 1) => Type::FiveOfAKind,
            _ => Type::HighCard,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.rank == other.rank {
            // Return whichever hand has a higher card first
            for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                if a > b {
                    return Some(std::cmp::Ordering::Greater);
                } else if a < b {
                    return Some(std::cmp::Ordering::Less);
                }
            }
            return None;
        } else if self.rank > other.rank {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in self.cards.iter() {
            write!(f, "{} ", card)?;
        }
        write!(f, "{}", self.bid)
    }
}

fn main() {
    debug_assert!(Card::Ace > Card::King);
    debug_assert!(Type::HighCard < Type::OnePair);

    let input = include_str!("input.txt");

    let mut hands: Vec<Hand> = input.lines().map(Hand::from_str).collect();

    hands.sort();

    let result: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid as u32 * (i as u32 + 1))
        .sum();

    println!("Part 1: {}", result);
}
