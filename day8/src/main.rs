use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

type Graph = HashMap<String, (String, String)>;

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r"^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)$").unwrap();
}

fn parse_line(l: &str) -> Option<(String, (String, String))> {
    let captures = LINE_REGEX.captures_iter(l).map(|caps| caps.extract());
    for (_, [n1, n2, n3]) in captures {
        return Some((n1.to_string(), (n2.to_string(), n3.to_string())));
    }
    None
}

fn parse(s: &str) -> Option<(Vec<Dir>, Graph)> {
    let (dir_str, graph_str) = s.split_once("\n\n")?;
    let dirs: Vec<Dir> = dir_str
        .chars()
        .filter_map(|c| match c {
            'L' => Some(Dir::Left),
            'R' => Some(Dir::Right),
            _ => None,
        })
        .collect();
    let graph: Graph = graph_str.lines().filter_map(|l| parse_line(l)).collect();

    Some((dirs, graph))
}

fn gcd_two(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
            std::mem::swap(&mut n, &mut m);
        }
        m = m % n;
    }
    n
}

fn lcm_two(n: u64, m: u64) -> u64 {
    n * (m / gcd_two(n, m))
}

// Run search from start until stop_fn(node) is true
fn run(g: &Graph, dirs: &Vec<Dir>, start: &String, stop_fn: fn(&String) -> bool) -> u64 {
    let mut steps = 0;
    let mut state = start;
    for dir in dirs.iter().cycle() {
        if stop_fn(state) {
            break;
        }
        let (left, right) = g.get(state).unwrap();
        state = match dir {
            Dir::Left => left,
            Dir::Right => right,
        };
        steps += 1;
    }
    steps
}

fn solve1(g: &Graph, dirs: &Vec<Dir>) -> u64 {
    let start = String::from("AAA");
    return run(g, dirs, &start, |s| s == "ZZZ");
}

fn solve2(g: &Graph, dirs: &Vec<Dir>) -> u64 {
    // this works if the paths from each starting node are disjoint cycles
    // with exactly one 'Z' node on each cycle.
    let starts = g.keys().filter(|k| k.ends_with("A"));
    let steps: Vec<u64> = starts
        .map(|start| run(g, dirs, start, |s| s.ends_with("Z")))
        .collect();
    let init = steps[0];
    let lcm = steps.into_iter().fold(init, |acc, x| lcm_two(acc, x));
    lcm
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let (dirs, graph) = parse(&input).unwrap();

    println!("{:?}", solve1(&graph, &dirs));
    println!("{:?}", solve2(&graph, &dirs));

    Ok(())
}
