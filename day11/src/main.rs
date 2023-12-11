use std::fs;
use std::io;

type Grid = Vec<Vec<char>>;
type Coord = (usize, usize);

fn parse_grid(s: &str) -> (Grid, Vec<Coord>) {
    let grid: Grid = s
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let n = grid.len();
    let m = grid[0].len();

    let mut planets: Vec<Coord> = Vec::new();
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '#' {
                planets.push((i, j));
            }
        }
    }
    (grid, planets)
}

// coords of planets when empty rows and cols expand by factor
fn expand(
    planets: &Vec<Coord>,
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
    factor: usize,
) -> Vec<Coord> {
    let mut expanded = planets.clone();
    for (i, r) in empty_rows.iter().enumerate() {
        for p in expanded.iter_mut() {
            if p.0 > *r + i * (factor - 1) {
                p.0 += factor - 1;
            }
        }
    }
    for (i, c) in empty_cols.iter().enumerate() {
        for p in expanded.iter_mut() {
            if p.1 > *c + i * (factor - 1) {
                p.1 += factor - 1;
            }
        }
    }
    expanded
}

fn distances(planets: &Vec<Coord>) -> Vec<usize> {
    let n = planets.len();
    let mut dists: Vec<usize> = Vec::new();
    for i in 0..n {
        for j in i..n {
            let (x1, y1) = planets[i];
            let (x2, y2) = planets[j];
            let xdiff = if x1 < x2 { x2 - x1 } else { x1 - x2 };
            let ydiff = if y1 < y2 { y2 - y1 } else { y1 - y2 };
            dists.push(xdiff + ydiff);
        }
    }
    dists
}

fn solve(grid: &Grid, planets: &Vec<Coord>, factor: usize) -> usize {
    let empty_rows: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter_map(|(i, r)| {
            if r.iter().all(|&c| c == '.') {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let mut empty_cols: Vec<usize> = Vec::new();
    let n = grid.len();
    let m = grid[0].len();
    for j in 0..m {
        let mut empty = true;
        for i in 0..n {
            if grid[i][j] != '.' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_cols.push(j);
        }
    }

    let expanded = expand(&planets, &empty_rows, &empty_cols, factor);
    let dist = distances(&expanded);
    dist.into_iter().sum()
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let (grid, planets) = parse_grid(&input);

    println!("{}", solve(&grid, &planets, 2));
    println!("{}", solve(&grid, &planets, 1000000));

    Ok(())
}
