use std::io;
use std::char;
use std::fs;
use std::str;
use std::collections::HashMap;

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}
type Coord = (usize, usize);
type Pipes = HashMap<Coord,[Dir; 2]>;

fn parse_char(c: &char) -> Option<[Dir; 2]> {
    match c {
        '|' => Some([Dir::Up, Dir::Down]),
        '-' => Some([Dir::Left, Dir::Right]),
        'F' => Some([Dir::Down, Dir::Right]),
        'L' => Some([Dir::Up, Dir::Right]),
        '7' => Some([Dir::Down, Dir::Left]),
        'J' => Some([Dir::Up, Dir::Left]),
        _ => None,
    }
}

fn parse_pipes(s: &str) -> Option<(Pipes,Coord)> {
    let mut adj: Pipes = HashMap::new();
    let grid : Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
    let mut start_coord = None;

    for (i, row) in grid.iter().enumerate() {
        for (j, chr) in row.iter().enumerate() {
            match parse_char(chr) {
                Some(dirs) => adj.insert((i,j), dirs),
                None => None,
            };
            if *chr == 'S' {
                start_coord = Some((i,j));
            }
        }
    }

    let start_char = match start_coord {
        None => None,
        Some((i,j)) => {
            let (is, js) = (i as i32, j as i32);
            let udlr = [(is-1,js),(is+1,js),(is,js-1),(is,js+1)];
            let [up,down,left,right] = udlr;
            let cs : Vec<(i32,i32)> = udlr
                .into_iter()
                .filter(|&(n,m)| {
                    n >= 0 && m >= 0
                        && (n as usize) < grid.len()
                        && (m as usize) < grid[0].len()
                        && grid[n as usize][m as usize] != '.'
                })
                .collect();

            // UD, UL, UR, DL, DR, LR
            if cs[0] == up && cs[1] == down {
                Some('|')
            } else if cs[0] == up && cs[1] == left {
                Some('J')
            } else if cs[0] == down && cs[1] == right {
                Some('F')
            } else if cs[0] == down && cs[1] == left {
                Some('7')
            } else if cs[0] == up && cs[1] == right {
                Some('L')
            } else {
                None
            }
        }
    };

    match start_char {
        Some(c) => {
            let coord = start_coord.unwrap();
            adj.insert(start_coord.unwrap(), parse_char(&c).unwrap());
            Some((adj, coord))
        }
        None => None,
    }
}

fn run_loop(pipes: &Pipes, start: &Coord) -> Vec<Coord> {
    let mut path : Vec<Coord> = vec![*start];
    let max = pipes.keys().len();
    for i in 0..=max {
        let last_state = if i>0 {
            path[i-1]
        } else {
            *start //dummy val
        };
        let cur_state = path[i];
        let (i,j) = cur_state;

        let next_state = pipes
            .get(&cur_state)
            .unwrap()
            .into_iter()
            .map(|dir| {
                match dir {
                    Dir::Up => (i-1,j),
                    Dir::Down => (i+1,j),
                    Dir::Left => (i,j-1),
                    Dir::Right => (i,j+1),
                }
            })
            .filter(|&s| s != last_state)
            .next()
            .unwrap();
        if next_state == *start {
            break;
        } else {
            path.push(next_state);
        }
    }
    path
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let (pipes, start) = parse_pipes(&input).unwrap();
    let path = run_loop(&pipes, &start);
    let ans1 = (path.len()+1)/2;
    println!("{}", ans1);

    Ok(())
}
