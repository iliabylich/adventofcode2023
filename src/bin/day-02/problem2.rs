fn main() {
    println!("Day 2, Problem 2");
    println!("{}", solve(include_str!("input.txt")));
}

#[derive(Debug)]
struct GameResult {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameResult {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    #[allow(dead_code)]
    n: u32,
    results: Vec<GameResult>,
}

impl Game {
    fn bare_minimum(&self) -> GameResult {
        let min_r = self.results.iter().map(|r| r.red).max().unwrap_or(0);
        let min_g = self.results.iter().map(|r| r.green).max().unwrap_or(0);
        let min_b = self.results.iter().map(|r| r.blue).max().unwrap_or(0);
        GameResult {
            red: min_r,
            green: min_g,
            blue: min_b,
        }
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
        .map(parse)
        .map(|game| game.bare_minimum())
        .map(|result| result.power())
        .sum()
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("problem2-input-small.txt")), 2286);
}
