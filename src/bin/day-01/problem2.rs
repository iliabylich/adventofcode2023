fn main() {
    println!("Day 1, Problem 2");
    println!("{}", solve(include_bytes!("input.txt")));
}

const NUMS: &[&[u8]] = &[
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn leftmost_rightmost(input: &[u8]) -> (u8, u8) {
    let mut leftmost = None;
    let mut rightmost = None;
    let mut offset = 0;

    while offset < input.len() {
        let slice = &input[offset..];
        if slice[0].is_ascii_digit() {
            let digit = slice[0] - b'0';
            if leftmost.is_none() {
                leftmost = Some(digit);
            }
            rightmost = Some(digit);
        } else {
            for (n, num) in NUMS.iter().enumerate() {
                if slice.starts_with(num) {
                    let digit = n as u8 + 1;
                    if leftmost.is_none() {
                        leftmost = Some(digit);
                    }
                    rightmost = Some(digit);
                    break;
                }
            }
        }

        offset += 1;
    }

    if leftmost.is_none() || rightmost.is_none() {
        panic!("failed to find a number in {:?}", input);
    }

    (leftmost.unwrap(), rightmost.unwrap())
}

fn solve(input: &[u8]) -> u64 {
    input
        .split(|b| *b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (left, right) = leftmost_rightmost(line);
            (left * 10 + right) as u64
        })
        .sum()
}

#[test]
fn test() {
    assert_eq!(solve(include_bytes!("problem2-input-small.txt")), 281);
}
