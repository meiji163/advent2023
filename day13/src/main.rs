use std::cmp::min;
use std::fs;
use std::io;
use std::iter;
use std::str;

type Grid = Vec<Vec<char>>;

#[derive(Debug, Eq, PartialEq)]
enum Mirror {
    Vert(usize),
    Horiz(usize),
}
use Mirror::*;

fn parse_grid(s: &str) -> Grid {
    s.lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

fn transpose(g: &Grid) -> Grid {
    let n = g.len();
    let m = g[0].len();
    let mut gt: Grid = iter::repeat_with(|| vec!['\0'; n]).take(m).collect();
    for i in 0..n {
        for j in 0..m {
            gt[j][i] = g[i][j];
        }
    }
    gt
}

fn horizontal_refl(g: &Grid) -> Option<Vec<usize>> {
    let mut refls: Vec<usize> = Vec::new();
    let n = g.len();
    for i in 0..n - 1 {
        let mut refl = true;
        for k in 0..min(i + 1, n - i - 1) {
            if g[i + k + 1] != g[i - k] {
                refl = false;
                break;
            }
        }
        if refl {
            refls.push(i);
        }
    }
    if refls.len() > 0 {
        Some(refls)
    } else {
        None
    }
}

fn solve(g: &Grid) -> Mirror {
    if let Some(rs) = horizontal_refl(g) {
        return Horiz(rs[0]);
    } else {
        let g_t = transpose(g);
        let rs = horizontal_refl(&g_t).unwrap();
        return Vert(rs[0]);
    }
}

fn solve2(g: &mut Grid) -> Mirror {
    let n = g.len();
    let m = g[0].len();

    let old_mirror = solve(g);
    for i in 0..n {
        for j in 0..m {
            let c = g[i][j];
            g[i][j] = if c == '#' { '.' } else { '#' };
            match horizontal_refl(g) {
                Some(rs) => {
                    for &r in rs.iter() {
                        if Horiz(r) != old_mirror {
                            return Horiz(r);
                        }
                    }
                }
                None => {}
            }
            g[i][j] = c;
        }
    }

    let mut g_t = transpose(g);
    for j in 0..m {
        for i in 0..n {
            let c = g_t[j][i];
            g_t[j][i] = if c == '#' { '.' } else { '#' };
            match horizontal_refl(&g_t) {
                Some(rs) => {
                    for &r in rs.iter() {
                        if Vert(r) != old_mirror {
                            return Vert(r);
                        }
                    }
                }
                None => {}
            }
            g_t[j][i] = c;
        }
    }
    panic!("no mirror found");
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut grids: Vec<Grid> = input.split("\n\n").map(|s| parse_grid(s)).collect();

    let ans1: usize = grids
        .iter_mut()
        .map(|g| match solve(g) {
            Horiz(i) => 100 * (i + 1),
            Vert(i) => i + 1,
        })
        .sum();
    println!("{}", ans1);

    let ans2: usize = grids
        .iter_mut()
        .map(|g| match solve2(g) {
            Horiz(i) => 100 * (i + 1),
            Vert(i) => i + 1,
        })
        .sum();
    println!("{}", ans2);

    Ok(())
}

#[test]
fn test() {
    let first = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
    let second = "\
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    let mut g1 = parse_grid(&first);
    let mut g2 = parse_grid(&second);

    assert_eq!(solve(&g1), Vert(4));
    assert_eq!(solve(&g2), Horiz(3));
    assert_eq!(solve2(&mut g1), Horiz(2));
    assert_eq!(solve2(&mut g2), Horiz(0));
}
