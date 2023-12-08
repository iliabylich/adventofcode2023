use std::collections::HashMap;

fn main() {
    println!("Day 8, Problem 1");
    println!("{}", solve(include_str!("input.txt")));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[repr(u8)]
enum Direction {
    Left = b'L',
    Right = b'R',
}

impl From<u8> for Direction {
    fn from(b: u8) -> Self {
        match b {
            b'L' => Self::Left,
            b'R' => Self::Right,
            _ => panic!("Invalid direction: {}", b),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Node([u8; 3]);

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        assert!(s.len() == 3);
        Self(s.as_bytes().try_into().unwrap())
    }
}

#[derive(Debug, Default)]
struct Map {
    go_left: HashMap<Node, Node>,
    go_right: HashMap<Node, Node>,
}

fn parse(input: &str) -> (Vec<Direction>, Map) {
    let (directions, edges) = input.split_once('\n').unwrap();

    let directions = directions.bytes().map(Direction::from).collect::<Vec<_>>();

    let mut map = Map::default();

    for line in edges.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (from, rest) = line.split_once(" = ").unwrap();
        let from = Node::from(from);

        let rest = rest.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
        let (left, right) = rest.split_once(", ").unwrap();
        let left = Node::from(left);
        let right = Node::from(right);

        map.go_left.insert(from, left);
        map.go_right.insert(from, right);
    }

    (directions, map)
}

fn solve(input: &str) -> u64 {
    let (directions, map) = parse(input);

    let mut node = Node::from("AAA");
    const STOP_AFTER_ITERATION: usize = 100_000;

    for (iteration, direction) in directions.into_iter().cycle().enumerate() {
        // eprintln!("{:?} goes to {:?}", node, direction);
        node = match direction {
            Direction::Left => *map.go_left.get(&node).unwrap(),
            Direction::Right => *map.go_right.get(&node).unwrap(),
        };

        if node == Node::from("ZZZ") {
            return (iteration + 1) as u64;
        }

        if iteration == STOP_AFTER_ITERATION {
            panic!("Too many iterations");
        }
    }

    0
}

#[test]
fn test1() {
    assert_eq!(solve(include_str!("problem1-input-small1.txt")), 2);
}

#[test]
fn test2() {
    assert_eq!(solve(include_str!("problem1-input-small2.txt")), 6);
}
