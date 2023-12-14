use std::collections::HashSet;
use std::fs;
use std::io;
use std::str;

type Grid = Vec<Vec<char>>;

const NUM_ITERS: usize = 1000000000;

fn parse_grid(s: &str) -> Grid {
    s.lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

fn north_load(g: &Grid) -> usize {
    let mut sum = 0;
    let n = g.len();
    let m = g[0].len();
    for j in 0..m {
        for i in 0..n {
            if g[i][j] == 'O' {
                sum += n - i;
            }
        }
    }
    sum
}

fn solve1(g: &mut Grid) -> usize {
    tilt_north(g);
    north_load(g)
}

fn tilt_north(g: &mut Grid) {
    let n = g.len();
    let m = g[0].len();
    for j in 0..m {
        let mut last = 0;
        let mut count = 0;
        for i in 0..n {
            match g[i][j] {
                'O' => {
                    if i != last + count {
                        g[last + count][j] = 'O';
                        g[i][j] = '.';
                    }
                    count += 1;
                }
                '#' => {
                    last = i + 1;
                    count = 0;
                }
                _ => {}
            }
        }
    }
}

fn tilt_south(g: &mut Grid) {
    let n = g.len();
    let m = g[0].len();
    for j in 0..m {
        let mut last = n - 1;
        let mut count = 0;
        for i in (0..n).rev() {
            match g[i][j] {
                'O' => {
                    if i != last - count {
                        g[last - count][j] = 'O';
                        g[i][j] = '.';
                    }
                    count += 1;
                }
                '#' => {
                    last = if i == 0 { 0 } else { i - 1 };
                    count = 0;
                }
                _ => {}
            }
        }
    }
}

fn tilt_west(g: &mut Grid) {
    let n = g.len();
    let m = g[0].len();
    for i in 0..n {
        let mut last = 0;
        let mut count = 0;
        for j in 0..m {
            match g[i][j] {
                'O' => {
                    if j != last + count {
                        g[i][last + count] = 'O';
                        g[i][j] = '.';
                    }
                    count += 1;
                }
                '#' => {
                    last = j + 1;
                    count = 0;
                }
                _ => {}
            }
        }
    }
}

fn tilt_east(g: &mut Grid) {
    let n = g.len();
    let m = g[0].len();
    for i in 0..n {
        let mut last = m - 1;
        let mut count = 0;
        for j in (0..m).rev() {
            match g[i][j] {
                'O' => {
                    if j != last - count {
                        g[i][last - count] = 'O';
                        g[i][j] = '.';
                    }
                    count += 1;
                }
                '#' => {
                    last = if j == 0 { 0 } else { j - 1 };
                    count = 0;
                }
                _ => {}
            }
        }
    }
}

fn cycle(g: &mut Grid) {
    tilt_north(g);
    tilt_west(g);
    tilt_south(g);
    tilt_east(g);
}

fn solve2(g: &mut Grid) -> usize {
    // detect start of the cycle
    let mut grids = HashSet::new();
    let mut i = 0;
    while !grids.contains(g) {
        grids.insert(g.clone());
        cycle(g);
        i += 1;
    }

    // now find the cycle length
    let mut cycle_len = 0;
    grids.clear();
    while !grids.contains(g) {
        grids.insert(g.clone());
        cycle(g);
        cycle_len += 1;
    }

    let rem = (NUM_ITERS - i) % cycle_len;
    for _ in 0..rem {
        cycle(g);
    }
    north_load(g)
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut grid = parse_grid(&input);

    println!("{}", solve1(&mut grid));
    println!("{}", solve2(&mut grid));

    Ok(())
}

#[test]
fn test_1() {
    let s = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let mut grid = parse_grid(&s);

    assert_eq!(solve1(&mut grid), 136);
    assert_eq!(solve2(&mut grid), 64);
}
