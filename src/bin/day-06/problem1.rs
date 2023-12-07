fn main() {
    println!("Day 6, Problem 1");
    println!("{}", solve(include_str!("input.txt")));
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse_line(line: &str) -> Vec<u64> {
    let line = line.split_once(':').unwrap().1;
    line.trim()
        .split(' ')
        .filter(|p| !p.is_empty())
        .map(|p| p.parse::<u64>().unwrap())
        .collect()
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

fn parse(input: &str) -> Vec<Race> {
    let (time_l, distance_l) = input.split_once('\n').unwrap();
    let times = parse_line(time_l);
    let distances = parse_line(distance_l);

    assert_eq!(times.len(), distances.len());

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn solve(input: &str) -> u64 {
    let races = parse(input);
    races.into_iter().map(options_count_for_race).product()
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("problem1-input-small.txt")), 288);
}
