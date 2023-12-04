#[derive(Debug)]
struct Game {
    pub id: u8,
    pub sets: Vec<GameSet>,
}

#[derive(Debug)]
struct GameSet {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Game {
    /**
     * Parses expected format: Game 1: 3 blue, 1 red; 2 green, 1 red; ...
     */
    pub fn parse(input: &'static str) -> Game {
        let mut sets = Vec::new();
        let mut id = 0;
        let mut split = input.split(':');
        if let Some(game_id) = split.next() {
            id = game_id[5..].trim().parse().unwrap();
        }
        if let Some(game_sets) = split.next() {
            for set in game_sets.split(';') {
                let cubes = set.split(',');
                let mut set = GameSet {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
                for cube in cubes {
                    let mut parts = cube.split(' ').filter(|s| s.len() > 0).map(|s| s.trim());
                    let count = parts.next().unwrap().parse::<u8>().unwrap();
                    let color = parts.next().unwrap();
                    match color {
                        "red" => set.red = count,
                        "green" => set.green = count,
                        "blue" => set.blue = count,
                        _ => (),
                    }
                }
                sets.push(set);
            }
        }
        Game { id, sets }
    }

    pub fn min_cubes(&self) -> GameSet {
        GameSet {
            red: self.sets.iter().map(|set| set.red).max().unwrap(),
            green: self.sets.iter().map(|set| set.green).max().unwrap(),
            blue: self.sets.iter().map(|set| set.blue).max().unwrap(),
        }
    }
}

impl GameSet {
    pub fn power(&self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }
}

static MAX_RED: u8 = 12;
static MAX_GREEN: u8 = 13;
static MAX_BLUE: u8 = 14;

fn main() {
    let input = include_str!("input.txt");

    let games = input.lines().map(Game::parse);

    // Part 1
    let valid_games = games.clone().filter(|game| {
        game.sets
            .iter()
            .all(|set| set.red <= MAX_RED && set.green <= MAX_GREEN && set.blue <= MAX_BLUE)
    });

    let valid_id_sums = valid_games.map(|game| game.id as u32).sum::<u32>();

    println!("Valid ID sum: {}", valid_id_sums);

    // Part 2
    let sum_of_power = games.map(|game| game.min_cubes().power()).sum::<u32>();

    println!("Sum of power: {}", sum_of_power);
}
