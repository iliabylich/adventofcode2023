fn main() {
    println!("Day 4, Problem 1");
    println!("{}", solve(include_str!("input.txt")));
}

fn process(line: &str) -> u32 {
    let (_, line) = line.split_once(": ").unwrap();
    let line = line.trim();
    let (numbers, winning_numbers) = line.split_once(" | ").unwrap();
    let numbers: Vec<u32> = numbers
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.trim().parse::<u32>().unwrap())
        .collect();
    let winning_numbers: Vec<u32> = winning_numbers
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.trim().parse::<u32>().unwrap())
        .collect();

    let cnt = numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count();

    if cnt == 0 {
        0
    } else {
        1 << (cnt - 1)
    }
}

fn solve(input: &str) -> u32 {
    input.lines().map(process).sum()
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("problem1-input-small.txt")), 13);
}
