fn main() {
    println!("Day 3, Problem 2");
    println!("{}", solve(include_str!("input.txt")));
}

#[derive(Debug, Clone, Copy)]
struct Number {
    value: u32,
    lineno: usize,
    col: usize,
    width: usize,
}

impl Number {
    fn is_adjacent_to_sym(&self, sym: &Symbol) -> bool {
        let l = self.col.checked_sub(1).unwrap_or(0);
        let r = self.col + self.width;
        let t = self.lineno.checked_sub(1).unwrap_or(0);
        let b = self.lineno + 1;

        sym.lineno >= t && sym.lineno <= b && sym.col >= l && sym.col <= r
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    lineno: usize,
    col: usize,
}

fn solve(input: &str) -> u32 {
    let mut n_start_col: Option<usize> = None;
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    let mut flush_if_have_number =
        |line: &str, lineno: usize, start_col: &mut Option<usize>, end_col: usize| {
            if let Some(start_col) = start_col.take() {
                let n = Number {
                    value: line[start_col..end_col].parse::<u32>().unwrap(),
                    lineno,
                    col: start_col,
                    width: end_col - start_col,
                };
                numbers.push(n);
            }
        };

    for (lineno, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => {
                    if n_start_col.is_none() {
                        n_start_col = Some(col);
                    }
                }
                '.' => {
                    flush_if_have_number(line, lineno, &mut n_start_col, col);
                }
                _ => {
                    flush_if_have_number(line, lineno, &mut n_start_col, col);
                    symbols.push(Symbol {
                        value: c,
                        lineno,
                        col,
                    });
                }
            }
        }
        flush_if_have_number(line, lineno, &mut n_start_col, line.len());
    }

    symbols
        .iter()
        .filter(|sym| sym.value == '*')
        .filter_map(|sym| {
            let mut iter = numbers.iter().filter(|n| n.is_adjacent_to_sym(sym));
            match (iter.next(), iter.next(), iter.next()) {
                (Some(n1), Some(n2), None) => Some(n1.value * n2.value),
                _ => None,
            }
        })
        .sum()
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("problem2-input-small.txt")), 467835);
}
