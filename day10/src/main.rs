use std::char;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::iter::FromIterator;
use std::str;

#[derive(Debug, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
type Coord = (usize, usize);
type Grid = Vec<Vec<char>>;
type Pipes = HashMap<Coord, [Dir; 2]>;

fn adj_coords(c: &Coord, x_max: usize, y_max: usize) -> Vec<Coord> {
    let &(i, j) = c;
    let mut adj: Vec<Coord> = Vec::with_capacity(4);
    if i > 1 {
        adj.push((i - 1, j));
    }
    if i + 1 < x_max {
        adj.push((i + 1, j));
    }
    if j > 1 {
        adj.push((i, j - 1));
    }
    if j + 1 < y_max {
        adj.push((i, j + 1));
    }
    adj
}

fn char_dirs(c: &char) -> Option<[Dir; 2]> {
    match c {
        '|' => Some([Dir::Up, Dir::Down]),
        'J' => Some([Dir::Up, Dir::Left]),
        'L' => Some([Dir::Up, Dir::Right]),
        '7' => Some([Dir::Down, Dir::Left]),
        'F' => Some([Dir::Down, Dir::Right]),
        '-' => Some([Dir::Left, Dir::Right]),
        _ => None,
    }
}

// if the pipe character connects in a given direction
fn connects(dir: Dir, c: &char) -> bool {
    match char_dirs(c) {
        Some(dirs) => dirs.contains(&dir),
        None => false,
    }
}

fn parse_input(s: &str) -> Option<(Grid, Pipes, Coord)> {
    let mut adj: Pipes = HashMap::new();
    let grid: Grid = s.lines().map(|l| l.chars().collect()).collect();
    let x_max = grid.len();
    let y_max = grid[0].len();
    let mut start_coord = None;

    for (i, row) in grid.iter().enumerate() {
        for (j, chr) in row.iter().enumerate() {
            match char_dirs(chr) {
                Some(dirs) => adj.insert((i, j), dirs),
                None => None,
            };
            if *chr == 'S' {
                start_coord = Some((i, j));
            }
        }
    }

    let (i, j) = start_coord?;
    let start_char = if i > 0
        && connects(Dir::Down, &grid[i - 1][j])
        && i + 1 < x_max
        && connects(Dir::Up, &grid[i + 1][j])
    {
        '|' // UD
    } else if i > 0
        && connects(Dir::Down, &grid[i - 1][j])
        && j > 0
        && connects(Dir::Right, &grid[i][j - 1])
    {
        'J' // UL
    } else if i > 0
        && connects(Dir::Down, &grid[i - 1][j])
        && j + 1 < y_max
        && connects(Dir::Left, &grid[i][j + 1])
    {
        'L' // UR
    } else if i + 1 < x_max
        && connects(Dir::Up, &grid[i + 1][j])
        && j > 0
        && connects(Dir::Right, &grid[i][j - 1])
    {
        '7' // DL
    } else if i + 1 < x_max
        && connects(Dir::Up, &grid[i + 1][j])
        && j + 1 < y_max
        && connects(Dir::Left, &grid[i][j + 1])
    {
        'F' // DR
    } else {
        '-' // LR
    };

    adj.insert((i, j), char_dirs(&start_char).unwrap());
    Some((grid, adj, (i, j)))
}

fn run_loop(pipes: &Pipes, start: &Coord) -> Vec<Coord> {
    let mut path: Vec<Coord> = vec![*start];
    let max = pipes.keys().len();
    for i in 0..=max {
        let last_state = if i > 0 {
            path[i - 1]
        } else {
            *start //dummy val
        };
        let cur_state = path[i];
        let (i, j) = cur_state;

        let next_state = pipes
            .get(&cur_state)
            .unwrap()
            .into_iter()
            .map(|dir| match dir {
                Dir::Up => (i - 1, j),
                Dir::Down => (i + 1, j),
                Dir::Left => (i, j - 1),
                Dir::Right => (i, j + 1),
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

// find if coordinate is inside the loop using number of ray crossings
fn is_inside_loop(c: &Coord, loop_set: &HashSet<Coord>, grid: &Grid) -> bool {
    let mut crosses = 0;
    let &(i, j) = c;
    let m = if i > j { j } else { i };
    for k in 0..=m {
        if loop_set.contains(&(i - k, j - k))
            && grid[i - k][j - k] != '7'
            && grid[i - k][j - k] != 'L'
        {
            crosses += 1;
        }
    }
    crosses % 2 == 1
}

fn flood_fill(c: &Coord, loop_set: &HashSet<Coord>, grid: &Grid) -> HashSet<Coord> {
    let x_max = grid.len();
    let y_max = grid[0].len();
    let mut edge: Vec<Coord> = vec![*c];
    let mut filled: HashSet<Coord> = HashSet::new();
    while !edge.is_empty() {
        let expand = edge.pop().unwrap();
        filled.insert(expand);

        let adj_coords = adj_coords(&expand, x_max, y_max);
        for coord in adj_coords.into_iter() {
            if !loop_set.contains(&coord) && !filled.contains(&coord) {
                edge.push(coord);
            }
        }
    }
    filled
}

fn solve2(loop_set: &HashSet<Coord>, grid: &Grid) -> usize {
    let mut interior_points = 0;
    let mut filled: HashSet<Coord> = HashSet::new();

    let x_max = grid.len();
    let y_max = grid[0].len();
    for i in 0..x_max {
        for j in 0..y_max {
            let coord = (i, j);
            if filled.contains(&coord) || loop_set.contains(&coord) {
                continue;
            } else {
                let next_fill = flood_fill(&coord, loop_set, grid);
                if is_inside_loop(&coord, loop_set, grid) {
                    interior_points += next_fill.len();
                }
                filled.extend(next_fill);
            }
        }
    }
    interior_points
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let (grid, pipes, start) = parse_input(&input).unwrap();
    let loop_vec = run_loop(&pipes, &start);
    let ans1 = (loop_vec.len() + 1) / 2;
    println!("{}", ans1);

    let loop_set: HashSet<Coord> = HashSet::from_iter(loop_vec);
    let ans2 = solve2(&loop_set, &grid);
    println!("{}", ans2);

    Ok(())
}

#[test]
fn test_grid_2() {
    let input = fs::read_to_string("test_2.txt").unwrap();
    let (grid, pipes, start) = parse_input(&input).unwrap();
    let loop_vec = run_loop(&pipes, &start);
    let loop_set: HashSet<Coord> = HashSet::from_iter(loop_vec);

    let coord = (4, 10);
    let filled = flood_fill(&coord, &loop_set, &grid);
    assert_eq!(filled.len(), 9);

    assert!(is_inside_loop(&coord, &loop_set, &grid));

    let interior_count = solve2(&loop_set, &grid);
    assert_eq!(interior_count, 10);
}
