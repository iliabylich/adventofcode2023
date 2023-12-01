fn main() {
    println!("Day 1, Problem 1");
    println!("{}", solve(include_bytes!("input.txt")));
}

fn solve(input: &[u8]) -> u64 {
    input
        .split(|b| *b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let first_number = line.iter().find(|b| b.is_ascii_digit()).unwrap();
            let last_number = line.iter().rev().find(|b| b.is_ascii_digit()).unwrap();
            ((first_number - b'0') * 10 + (last_number - b'0')) as u64
        })
        .sum()
}

#[test]
fn test() {
    assert_eq!(solve(include_bytes!("problem1-input-small.txt")), 142);
}
