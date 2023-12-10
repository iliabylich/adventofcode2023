fn main() {
    println!("Day 9, Problem 2");
    println!("{}", solve(include_str!("input.txt")));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Sequence {
    numbers: Vec<i64>,
}

impl Sequence {
    fn next(&self) -> Self {
        let next_numbers = self
            .numbers
            .iter()
            .zip(self.numbers.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();
        Self {
            numbers: next_numbers,
        }
    }

    fn zeroes(&self) -> bool {
        self.numbers.iter().all(|n| *n == 0)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Matrix {
    rows: Vec<Sequence>,
}

impl From<Sequence> for Matrix {
    fn from(seq: Sequence) -> Self {
        let mut rows = vec![];
        let mut seq = seq;
        loop {
            let next = seq.next();
            rows.push(seq);
            seq = next;

            if seq.zeroes() {
                rows.push(seq);
                break;
            }
        }
        Self { rows }
    }
}

impl Matrix {
    fn predict(&mut self) {
        self.rows.last_mut().unwrap().numbers.push(0);

        for row_idx in (0..self.rows.len() - 1).rev() {
            let a = self.rows[row_idx].numbers[0];
            let b = self.rows[row_idx + 1].numbers[0];
            self.rows[row_idx].numbers.insert(0, a - b);
        }
    }

    fn prediction(&self) -> i64 {
        self.rows[0].numbers[0]
    }
}

fn parse_line(line: &str) -> Sequence {
    let numbers = line.split(' ').map(|s| s.parse().unwrap()).collect();
    Sequence { numbers }
}

fn solve(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line)
        .map(|seq| {
            let mut m = Matrix::from(seq);
            m.predict();
            m
        })
        .map(|matrix| matrix.prediction())
        .sum()
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("problem1-input-small.txt")), 2);
}
