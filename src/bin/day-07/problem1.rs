fn main() {
    println!("Day 7, Problem 1");
    println!("{}", solve(include_str!("input.txt")));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    A = 13,
    K = 12,
    Q = 11,
    J = 10,
    T = 9,
    Nine = 8,
    Eight = 7,
    Seven = 6,
    Six = 5,
    Five = 4,
    Four = 3,
    Three = 2,
    Two = 1,
}

impl Card {
    fn parse(c: char) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Invalid card: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn parse(s: &str) -> Self {
        let mut iter = s.chars().map(Card::parse);
        let cards = [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ];
        Self { cards }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum HandScore {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandScore {
    fn of_hand(Hand { cards }: Hand) -> Self {
        let mut map = std::collections::HashMap::new();

        for card in cards.iter() {
            *map.entry(card).or_insert(0) += 1;
        }

        if map.iter().any(|(_, count)| *count == 5) {
            return Self::FiveOfAKind;
        }

        if map.iter().any(|(_, count)| *count == 4) {
            return Self::FourOfAKind;
        }

        if map.iter().any(|(_, count)| *count == 3) {
            if map.iter().any(|(_, count)| *count == 2) {
                return Self::FullHouse;
            }
            return Self::ThreeOfAKind;
        }

        if let Some((high_pair, _)) = map
            .iter()
            .filter(|(_, count)| **count == 2)
            .max_by(|&(card1, _), &(card2, _)| card1.cmp(card2))
        {
            if map
                .iter()
                .any(|(card, count)| *count == 2 && card != high_pair)
            {
                return Self::TwoPair;
            }
            return Self::OnePair;
        }

        Self::HighCard
    }

    fn score(self) -> u8 {
        self as u8
    }
}

impl PartialOrd for HandScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score().cmp(&other.score()))
    }
}

impl Ord for HandScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HandWithBid {
    hand: Hand,
    bid: u64,
}

impl HandWithBid {
    fn parse(line: &str) -> Self {
        let (hand, bid) = line.split_once(' ').unwrap();
        let hand = Hand::parse(hand);
        let bid = bid.parse().unwrap();
        Self { hand, bid }
    }

    fn hand_score(&self) -> (HandScore, [Card; 5]) {
        (HandScore::of_hand(self.hand), self.hand.cards)
    }
}

impl PartialOrd for HandWithBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandWithBid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_score().cmp(&other.hand_score())
    }
}

fn solve(input: &str) -> u64 {
    let mut hands_with_bids = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(HandWithBid::parse)
        .collect::<Vec<_>>();

    hands_with_bids.sort();

    hands_with_bids
        .into_iter()
        .enumerate()
        .map(|(idx, h_with_bid)| h_with_bid.bid * (idx as u64 + 1))
        .sum()
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("problem1-input-small.txt")), 6440);
}
