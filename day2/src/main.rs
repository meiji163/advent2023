use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;

type Game = Vec<[u32; 3]>;

fn parse_game(s: &str) -> Option<Game> {
    let seq = s.split(": ").last()?;
    let mut rgbs : Vec<[u32; 3]> = Vec::new();

    // parse rgb's
    for draw in seq.split("; ") {
        let mut rgb: [u32; 3] = [0; 3];
        for balls in draw.split(", ") {
            let num_color : Vec<&str> = balls.split(" ").collect();
            if num_color.len() != 2 {
                return None;
            }

            let num : u32 = num_color[0].parse().unwrap();
            let color = num_color[1];
            match color {
                "red" => rgb[0] = num,
                "green" => rgb[1] = num,
                "blue" => rgb[2] = num,
                _ => return None,
            }
        }
        rgbs.push(rgb);
    }
    Some(rgbs)
}

fn is_valid(g: &Game) -> bool {
    for b in g.iter() {
        if b[0] > 12 || b[1] > 13 || b[2] > 14 {
            return false
        }
    }
    true
}

fn max_rgb(g: &Game) -> [u32; 3] {
    let mut min: [u32; 3] =  [0; 3];
    for rgb in g.iter() {
        if min[0] < rgb[0] {min[0] = rgb[0];}
        if min[1] < rgb[1] {min[1] = rgb[1];}
        if min[2] < rgb[2] {min[2] = rgb[2];}
    }
    min
}

fn solve1(games: &Vec<Game>) -> u32 {
    let mut sum = 0;
    for (i, game) in games.iter().enumerate() {
        if is_valid(game) {
            sum += (i+1) as u32;
        }
    }
    sum
}

fn solve2(games: &Vec<Game>) -> u32 {
    return games
        .iter()
        .map(|g| max_rgb(&g))
        .map(|rgb| rgb[0]*rgb[1]*rgb[2])
        .sum();
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let games : Vec<Game> = reader
        .lines()
        .filter_map(|l| parse_game(&l.unwrap()))
        .collect();

    println!("{}", solve1(&games));
    println!("{}", solve2(&games));

    Ok(())
}
