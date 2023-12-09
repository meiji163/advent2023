use std::io::{self, prelude::*, BufReader};
use std::fs::File;

fn parse_line(s: &str) -> Vec<i32> {
    s.split(' ').filter_map(|x| x.parse().ok()).collect()
}

fn diff(seq: &Vec<i32>) -> Vec<i32> {
    let mut out = Vec::new();
    let len = seq.len();
    for i in 1..len {
        out.push(seq[i] - seq[i-1])
    }
    out
}

fn diff_until_const(seq: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut diffs : Vec<Vec<i32>> = vec![seq.clone()];
    for i in 0..seq.len() {
        let diff_seq = diff(&diffs[i]);
        if diff_seq.iter().all(|&x| x == 0) {
            break;
        }
        diffs.push(diff_seq);
    }
    diffs
}

fn next_value(seq: &Vec<i32>) -> i32 {
    let diffs = diff_until_const(seq);
    diffs.into_iter().rev().fold(0, |acc, v| acc + v.last().unwrap())
}

fn prev_value(seq: &Vec<i32>) -> i32 {
    let diffs = diff_until_const(seq);
    diffs.into_iter().rev().fold(0, |acc, v| v[0] - acc)
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let seqs : Vec<Vec<i32>> = reader.lines().map(|l| parse_line(&l.unwrap())).collect();
    let ans1 : i32 = seqs.iter().map(|seq| next_value(seq)).sum();
    println!("{}", ans1);

    let ans2 : i32 = seqs.iter().map(|seq| prev_value(seq)).sum();
    println!("{}", ans2);

    Ok(())
}

#[test]
fn test() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(file);
    let seqs : Vec<Vec<i32>> = reader.lines().map(|l| parse_line(&l.unwrap())).collect();
    let mut next_vals = seqs.iter().map(|seq| next_value(&seq));
    assert_eq!(next_vals.next(), Some(18));
    assert_eq!(next_vals.next(), Some(28));
    assert_eq!(next_vals.next(), Some(68));

    let mut prev_vals = seqs.iter().map(|seq| prev_value(&seq));
    assert_eq!(prev_vals.next(), Some(-3));
    assert_eq!(prev_vals.next(), Some(0));
    assert_eq!(prev_vals.next(), Some(5));
}
