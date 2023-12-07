fn main() {
    println!("Day 6, Problem 2");
    println!("{}", solve(include_str!("input.txt")));
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse_line(line: &str) -> u64 {
    let numbers = line.split_once(':').unwrap().1;
    numbers
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .fold(String::new(), |acc, s| acc + s)
        .parse()
        .unwrap()
}

fn options_count_for_race(race: Race) -> u64 {
    let Race { time, distance } = race;
    // delay of D produces:
    // constant speed of D
    // total running time of <time> - D
    // distance = (<time> - D) * D
    //
    // We need to find all D that
    // (<time> - D) * D > <distance>
    // and D can only be in the range [0, <time>]

    (0..=time).filter(|&d| (time - d) * d > distance).count() as u64
}

fn parse(input: &str) -> Race {
    let (time_l, distance_l) = input.split_once('\n').unwrap();
    let time = parse_line(time_l);
    let distance = parse_line(distance_l);
    Race { time, distance }
}

fn solve(input: &str) -> u64 {
    let race = parse(input);
    options_count_for_race(race)
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("problem2-input-small.txt")), 71503);
}
