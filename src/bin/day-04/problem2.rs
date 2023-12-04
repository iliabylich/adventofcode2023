fn main() {
    println!("Day 4, Problem 2");
    println!("{}", solve(include_str!("input.txt")));
}

#[derive(Debug)]
struct Card {
    cardno: u32,
    winning_cards: Vec<u32>,
}

fn process(line: &str) -> Card {
    let (header, line) = line.split_once(": ").unwrap();
    let cardno = header
        .split_once(' ')
        .unwrap()
        .1
        .trim()
        .parse::<u32>()
        .unwrap();
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

    let winning_cards = if cardno == 0 {
        vec![]
    } else {
        (cardno + 1..=cardno + cnt as u32).collect()
    };

    Card {
        cardno,
        winning_cards,
    }
}

fn solve(input: &str) -> u32 {
    let cards = input.lines().map(process).collect::<Vec<_>>();

    let mut available_cards = std::collections::HashMap::new();
    let mut won_cards = std::collections::HashMap::new();
    for card in &cards {
        available_cards.insert(card.cardno, 1);
        won_cards.insert(card.cardno, 0);
    }

    let mut total = 0;
    for card in &cards {
        let count = *available_cards.get(&card.cardno).unwrap();
        assert!(count > 0);
        total += count;

        for winning_cardno in &card.winning_cards {
            available_cards
                .entry(*winning_cardno)
                .and_modify(|n| *n += count);
        }
    }

    total
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("problem2-input-small.txt")), 30);
}
