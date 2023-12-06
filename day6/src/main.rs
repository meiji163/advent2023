use std::fs;
use std::io;
use std::iter::zip;
use std::str;

fn parse_races(inp: &str) -> Vec<(i64, i64)> {
    let mut lines = inp.lines();
    let t_str = lines.next().unwrap();
    let d_str = lines.next().unwrap();
    let times: Vec<i64> = t_str
        .split_ascii_whitespace()
        .filter_map(|d| d.parse().ok())
        .collect();
    let dists: Vec<i64> = d_str
        .split_ascii_whitespace()
        .filter_map(|d| d.parse().ok())
        .collect();
    zip(times, dists).collect()
}

fn parse_big_race(inp: &str) -> (i64, i64) {
    let mut lines = inp.lines();
    let t_str: String = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect();
    let d_str: String = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect();
    let t: i64 = t_str.parse().unwrap();
    let d: i64 = d_str.parse().unwrap();
    (t, d)
}

fn solve(race: &(i64, i64)) -> i64 {
    let &(t, d) = race;
    let disc: f64 = (t * t - 4 * d) as f64;
    if disc < 0.0 {
        return 0;
    }
    let hi = ((t as f64) + f64::sqrt(disc)) / 2.0;
    let lo = ((t as f64) - f64::sqrt(disc)) / 2.0;
    let mut range = (hi.floor() - lo.ceil()) as i64 + 1;
    // exclude endpoints if zeros are ints
    if hi.fract() == 0.0 {
        range -= 1;
    }
    if lo.fract() == 0.0 {
        range -= 1;
    }
    range
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let races = parse_races(&input);
    let ans1: i64 = races.iter().map(|r| solve(r)).product();
    println!("{}", ans1);

    let big_race = parse_big_race(&input);
    println!("{}", solve(&big_race));

    Ok(())
}
