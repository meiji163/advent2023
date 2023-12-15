use std::fs;
use std::io;
use std::iter;
use std::str;

#[derive(Debug, Eq, PartialEq)]
enum OpType {
    Rm,
    Add(usize),
}
use OpType::*;
type Op = (String, OpType);

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| {
        let n = c as usize;
        (17 * (acc + n)) % 256
    })
}

fn parse_op(s: &str) -> Option<Op> {
    if s.ends_with('-') {
        Some((s[..s.len() - 1].to_string(), Rm))
    } else {
        let (label, len_str) = s.split_once('=')?;
        let len: usize = len_str.parse().unwrap();
        Some((label.to_string(), Add(len)))
    }
}

fn solve(ops: &Vec<Op>) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = iter::repeat(vec![]).take(256).collect();
    for op in ops.iter() {
        let hash = hash(&op.0);
        match op {
            (label, Rm) => {
                boxes[hash].retain(|(l, _)| l != label);
            }
            (label, Add(n)) => {
                if let Some(i) = boxes[hash].iter().position(|(l, _)| l == label) {
                    boxes[hash][i] = (label.clone(), *n);
                } else {
                    boxes[hash].push((label.clone(), *n));
                }
            }
        }
    }
    boxes
        .into_iter()
        .enumerate()
        .map(|(i, box_vec)| {
            box_vec
                .iter()
                .enumerate()
                .fold(0, |acc, (j, op)| acc + (j + 1) * (i + 1) * op.1)
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let seq: Vec<&str> = input.trim_end().split(",").collect();

    let ans1: usize = seq.iter().map(|s| hash(s)).sum();
    println!("{}", ans1);

    let ops: Vec<Op> = seq.iter().filter_map(|s| parse_op(s)).collect();
    println!("{}", solve(&ops));

    Ok(())
}

#[test]
fn test() {
    let s = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let seq: Vec<&str> = s.trim_end().split(",").collect();
    let hashes: Vec<usize> = seq.iter().map(|s| hash(s)).collect();
    let ops: Vec<Op> = seq.iter().filter_map(|s| parse_op(s)).collect();

    assert_eq!(hashes.len(), 11);
    assert_eq!(hashes, vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231]);
    assert_eq!(solve(&ops), 145);
}
