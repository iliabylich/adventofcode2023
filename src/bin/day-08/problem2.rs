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

fn solve1(mut node: Node, directions: &[Direction], map: &Map) -> usize {
    const STOP_AFTER_ITERATION: usize = 100_000;

    for (iteration, direction) in directions.iter().cycle().enumerate() {
        // eprintln!("{:?} goes to {:?}", node, direction);
        node = match direction {
            Direction::Left => *map.go_left.get(&node).unwrap(),
            Direction::Right => *map.go_right.get(&node).unwrap(),
        };

        if node.0[2] == b'Z' {
            return iteration + 1;
        }

        if iteration == STOP_AFTER_ITERATION {
            panic!("Too many iterations");
        }
    }

    0
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn solve(input: &str) -> usize {
    let (directions, map) = parse(input);

    map.go_left
        .keys()
        .filter(|node| node.0[2] == b'A')
        .map(|node| solve1(*node, &directions, &map))
        .fold(1, lcm)
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("problem2-input-small.txt")), 6);
}
