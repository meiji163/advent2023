use std::io;
use std::fs;
use std::u64;

#[derive(Debug,Eq,PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use Dir::*;

#[derive(Debug,Eq,PartialEq)]
struct Op {
    dir: Dir,
    len: u64,
}

type Coord = (i64,i64);

fn parse(s: &str) -> Option<Op> {
    let s_vec = s.splitn(3, ' ').collect::<Vec<_>>();
    if s_vec.len() != 3 {
        return None;
    }
    let len : u64 = s_vec[1].parse().unwrap();
    let dir = match s_vec[0] {
        "U" => Up,
        "R" => Right,
        "L" => Left,
        "D" => Down,
        _ => {return None;}
    };
    Some(Op {dir, len})
}

fn parse_2(s: &str) -> Option<Op> {
    let op_str = s.splitn(3, ' ').last().unwrap();
    let color_str = op_str.replace(&['(', ')', '#'], "");
    let len = u64::from_str_radix(&color_str[0..5], 16).unwrap();
    let dir = match color_str.chars().last().unwrap() {
        '0' => Right,
        '1' => Down,
        '2' => Left,
        '3' => Up,
        _ => {return None;}
    };
    Some(Op{ dir, len })
}

fn area(vs: &Vec<Coord>) -> u64 {
    let mut a = 0;
    for i in 0..vs.len()-1 {
        let (x1, y1) = vs[i];
        let (x2, y2) = vs[i+1];
        a += x1*y2 - x2*y1;
    }
    i64::abs(a / 2) as u64
}

fn boundary(ops: &Vec<Op>) -> u64 {
    ops.iter().map(|op| op.len).sum()
}

fn solve(ops: &Vec<Op>) -> u64 {
    let start = (0,0);
    let mut coords : Vec<Coord> = vec![start];
    for op in ops.iter() {
        let (i,j) = coords[coords.len()-1];
        let n = op.len as i64;
        let next = match op.dir {
            Up => (i-n, j),
            Down => (i+n, j),
            Left => (i,j-n),
            Right => (i,j+n),
        };
        coords.push(next);
        if next == start {
            break;
        }
    }
    // Pick's theorem
    let b = boundary(&ops);
    let a = area(&coords);
    let i = a + 1 - b/2;
    b + i
}

fn main() -> io::Result<()>{
    let input = fs::read_to_string("input.txt")?;
    let ops1 : Vec<Op> = input.lines().filter_map(|l| parse(l)).collect();
    let ops2 : Vec<Op> = input.lines().filter_map(|l| parse_2(l)).collect();

    println!("{}",solve(&ops1));
    println!("{}",solve(&ops2));

    Ok(())
}

#[test]
fn test() {
    let input = fs::read_to_string("test.txt").unwrap();
    let ops1 : Vec<Op> = input.lines().filter_map(|l| parse(l)).collect();
    let ops2 : Vec<Op> = input.lines().filter_map(|l| parse_2(l)).collect();

    assert_eq!(62, solve(&ops1));
    assert_eq!(952408144115, solve(&ops2));
}
