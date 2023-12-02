use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;

const NUM_WORDS : &[&str] = &[
    "one", "two", "three", "four", "five",
    "six", "seven", "eight", "nine",
];

const NUM_WORDS_REV : &[&str] = &[
    "eno", "owt", "eerht", "ruof", "evif",
    "xis", "neves", "thgie", "enin",
];

const NUM_DIGITS : &[&str] = &[
    "1", "2", "3", "4", "5", "6", "7", "8", "9",
];

fn solve1(s: &str) -> u32 {
    let digs: Vec<u32> = s
        .chars()
        .filter_map(|c| {
            c.to_digit(10)
        })
        .collect();
    let n = digs.len();
    if n < 1 { 0 } else { 10*digs[0] + digs[n-1] }
}

fn rev_str(s: &str) -> String {
    return s.chars().rev().collect::<String>();
}

fn first_digit(s: &str) -> u32 {
    let mut sc = s.to_string();
    while !sc.is_empty() {
        for (i, num) in NUM_DIGITS.iter().enumerate() {
            if sc.starts_with(num) {
                return (i+1) as u32;
            }
        }
        for (i, num) in NUM_WORDS.iter().enumerate() {
            if sc.starts_with(num) {
                return (i+1) as u32;
            }
        }
        sc.remove(0);
    }
    0
}

fn last_digit(s: &str) -> u32 {
    let mut sc = rev_str(s);

    while !sc.is_empty() {
        for (i, num) in NUM_DIGITS.iter().enumerate() {
            if sc.starts_with(num) {
                return (i+1) as u32;
            }
        }
        for (i, num) in NUM_WORDS_REV.iter().enumerate() {
            if sc.starts_with(num) {
                return (i+1) as u32;
            }
        }
        sc.remove(0);
    }
    0
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // let ans1: u32 = reader
    //     .lines()
    //     .map(|l| { solve1(&l.unwrap())})
    //     .sum();
    // println!("{}", ans1);

    let ans2: u32 = reader
        .lines()
        .map(|l| {
            let lu = l.unwrap();
            10*first_digit(&lu) + last_digit(&lu)
        })
        .sum();
    println!("{}", ans2);

    Ok(())
}
