use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;

#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    hand: Vec<u32>,
}

fn parse_card(l: &str) -> Option<Card> {
    let seq = l.split(": ").last()?;
    let (win_str, hand_str) = seq.split_once("| ")?;
    let winning: Vec<u32> = win_str
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let hand: Vec<u32> = hand_str
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    Some(Card { winning, hand })
}

fn count_matches(c: &Card) -> u32 {
    let count = c.hand.iter().filter(|x| c.winning.contains(x)).count();
    count as u32
}

fn solve1(cs: &Vec<Card>) -> u32 {
    return cs
        .iter()
        .map(|c| {
            let matches = count_matches(&c);
            if matches == 0 {
                0
            } else {
                let exp = (matches as u32) - 1;
                (2_u32).pow(exp)
            }
        })
        .sum();
}

fn solve2(cs: &Vec<Card>) -> u32 {
    let len = cs.len();
    let scores: Vec<u32> = cs.iter().map(|c| count_matches(c)).collect();
    let mut num_cards = vec![1; len];

    for (i, score) in scores.iter().enumerate() {
        let s = *score as usize;
        for j in (i + 1)..=(i + s) {
            if j >= len {
                break;
            }
            num_cards[j] += num_cards[i];
        }
    }
    num_cards.into_iter().sum()
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let cards: Vec<Card> = reader
        .lines()
        .filter_map(|l| parse_card(&l.unwrap()))
        .collect();

    println!("{}", solve1(&cards));
    println!("{}", solve2(&cards));

    Ok(())
}
