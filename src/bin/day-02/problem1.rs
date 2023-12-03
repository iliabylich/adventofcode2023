fn main() {
    println!("Day 2, Problem 1");
    println!("{}", solve(include_str!("input.txt")));
}

#[derive(Debug)]
struct GameResult {
    red: u32,
    green: u32,
    blue: u32,
}

const MAX_GAME_RESULT: GameResult = GameResult {
    red: 12,
    green: 13,
    blue: 14,
};

impl GameResult {
    fn is_possible(&self) -> bool {
        self.red <= MAX_GAME_RESULT.red
            && self.green <= MAX_GAME_RESULT.green
            && self.blue <= MAX_GAME_RESULT.blue
    }
}

struct Game {
    n: u32,
    results: Vec<GameResult>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.results.iter().all(|r| r.is_possible())
    }
}

fn parse(line: &str) -> Game {
    let (header, games) = line.split_once(':').unwrap();

    let (_, game_no) = header.split_once(' ').unwrap();
    let game_no = game_no.parse::<u32>().unwrap();

    let results = games
        .split(';')
        .map(|game_s| {
            let mut result = GameResult {
                red: 0,
                green: 0,
                blue: 0,
            };

            for part in game_s.split(',') {
                let (value, color) = part.trim().split_once(' ').unwrap();
                let value = value.parse::<u32>().unwrap();

                match color {
                    "red" => result.red = value,
                    "green" => result.green = value,
                    "blue" => result.blue = value,
                    _ => panic!("Unknown color: {}", color),
                }
            }

            result
        })
        .collect::<Vec<_>>();

    Game {
        n: game_no,
        results,
    }
}

fn solve(input: &str) -> u32 {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| parse(line))
        .filter(|game| game.is_possible())
        .map(|game| game.n)
        .sum()
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("problem1-input-small.txt")), 8);
}
