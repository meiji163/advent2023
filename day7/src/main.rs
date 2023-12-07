use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::iter::zip;

type Hand = [i32; 5];

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse_line(s: &str, use_jokers: bool) -> Option<(Hand, usize)> {
    let (hand_str, bid_str) = s.split_once(' ')?;
    let hand = parse_hand(hand_str, use_jokers);
    let bid: usize = bid_str.parse().unwrap_or(0);
    Some((hand, bid))
}

fn parse_hand(s: &str, use_jokers: bool) -> Hand {
    s.chars()
        .filter_map(|c| match c {
            'J' => Some(if use_jokers { 1 } else { 11 }),
            '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Some(c as i32 - 0x30),
            'T' => Some(10),
            'Q' => Some(12),
            'K' => Some(13),
            'A' => Some(14),
            _ => None,
        })
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap()
}

fn improve_hand(kind: Kind, num_jokers: usize) -> Kind {
    if num_jokers == 1 {
        match kind {
            Kind::HighCard => Kind::OnePair,
            Kind::OnePair => Kind::ThreeOfAKind,
            Kind::TwoPair => Kind::FullHouse,
            Kind::ThreeOfAKind | Kind::FullHouse => Kind::FourOfAKind,
            Kind::FourOfAKind => Kind::FiveOfAKind,
            _ => kind,
        }
    } else if num_jokers == 2 {
        match kind {
            Kind::HighCard => Kind::ThreeOfAKind,
            Kind::OnePair => Kind::FourOfAKind,
            Kind::ThreeOfAKind => Kind::FiveOfAKind,
            _ => kind,
        }
    } else if num_jokers == 3 {
        match kind {
            Kind::HighCard => Kind::FourOfAKind,
            Kind::OnePair => Kind::FiveOfAKind,
            _ => kind,
        }
    } else if num_jokers == 4 {
        Kind::FiveOfAKind
    } else {
        kind
    }
}

fn hand_kind(hand: &Hand, use_jokers: bool) -> Kind {
    let counts = hand.iter().sorted().dedup_with_count();
    let mut jokers = 0;
    let mut pairs = 0;
    let mut threes = 0;
    let mut fours = 0;
    for (count, &card) in counts {
        if card == 1 {
            jokers += count;
            continue;
        }
        match count {
            5 => {
                return Kind::FiveOfAKind;
            }
            4 => {
                fours += 1;
            }
            3 => {
                threes += 1;
            }
            2 => {
                pairs += 1;
            }
            _ => {}
        }
    }
    let kind = if fours == 1 {
        Kind::FourOfAKind
    } else if pairs == 2 {
        Kind::TwoPair
    } else if pairs == 1 && threes == 1 {
        Kind::FullHouse
    } else if threes == 1 {
        Kind::ThreeOfAKind
    } else if pairs == 1 {
        Kind::OnePair
    } else {
        Kind::HighCard
    };

    if use_jokers {
        improve_hand(kind, jokers)
    } else {
        kind
    }
}

fn cmp_hands(h1: &Hand, h2: &Hand, use_jokers: bool) -> Ordering {
    let t1 = hand_kind(h1, use_jokers);
    let t2 = hand_kind(h2, use_jokers);
    if t1 != t2 {
        t1.cmp(&t2)
    } else {
        for (n1, n2) in zip(h1.iter(), h2.iter()) {
            let ord = n1.cmp(n2);
            if ord != Ordering::Equal {
                return ord;
            }
        }
        Ordering::Equal
    }
}

fn solve(hands_scores: &Vec<(Hand, usize)>, use_jokers: bool) -> usize {
    let sorted_scores = hands_scores
        .iter()
        .sorted_by(|t1, t2| cmp_hands(&t1.0, &t2.0, use_jokers))
        .map(|t| t.1);
    sorted_scores
        .enumerate()
        .fold(0, |acc, (i, score)| acc + (i + 1) * score)
}

fn main() -> io::Result<()> {
    //let file = File::open("test.txt")?;
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let use_jokers = true;
    let hands_scores: Vec<(Hand, usize)> = reader
        .lines()
        .filter_map(|l| parse_line(&l.unwrap(), use_jokers))
        .collect();
    println!("{:?}", solve(&hands_scores, use_jokers));

    Ok(())
}
