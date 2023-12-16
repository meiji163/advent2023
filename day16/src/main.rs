use std::io;
use std::fs;
use std::cmp;
use std::collections::HashSet;

type Grid = Vec<Vec<char>>;
type Coord = (usize, usize);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use Dir::*;

fn parse_grid(s: &str) -> Grid {
    s.lines().map(|l| l.chars().collect::<Vec<char>>()).collect()
}

fn traverse(g: &Grid, c: &Coord, dir: &Dir) -> Vec<Coord> {
    let mut vis : Vec<Coord> = vec![*c];
    let (mut i, mut j) = *c;
    if g[i][j] != '.' {
        return vis;
    }
    match dir {
        Up => {
            while i>0 {
                i -= 1;
                vis.push((i,j));
                if g[i][j] != '.' {break;}
            }
        },
        Down => {
            let n = g.len();
            while i<n-1 {
                i += 1;
                vis.push((i,j));
                if g[i][j] != '.' {break;}
            }
        }
        Left => {
            while j>0 {
                j -= 1;
                vis.push((i,j));
                if g[i][j] != '.' {break;}
            }
        },
        Right => {
            let m = g[0].len();
            while j<m-1 {
                j += 1;
                vis.push((i,j));
                if g[i][j] != '.' {break;}
            }
        }
    }
    vis
}

fn solve(g: &Grid, c: Coord, d: Dir) -> usize {
    let n = g.len();
    let m = g[0].len();
    let mut visited : HashSet<(Coord, Dir)> = HashSet::new();
    let mut energized : HashSet<Coord> = HashSet::new();
    let mut stack : Vec<(Coord, Dir)> = Vec::new();
    energized.insert(c);
    stack.push((c, d));

    while !stack.is_empty() {
        let v = stack.pop().unwrap();
        if !visited.contains(&v) {
            visited.insert(v.clone());

            let (c, dir) = v;
            let crds = traverse(g, &c, &dir);
            if crds.is_empty() {
                continue;
            }
            let (i,j) = crds[crds.len() - 1];
            energized.extend(crds.into_iter());

            let next_dir = match g[i][j] {
                '\\' => {
                    match dir {
                        Right => Down,
                        Up => Left,
                        Down => Right,
                        Left => Up,
                    }
                },
                '/' => {
                    match dir {
                        Right => Up,
                        Down => Left,
                        Left => Down,
                        Up => Right,
                    }

                },
                '|' => {
                    match dir {
                        Right | Left => {
                            stack.push(((i,j), Up));
                            Down
                        },
                        Up => Up,
                        Down => Down,
                    }
                },
                '-' => {
                    match dir {
                        Up | Down => {
                            stack.push(((i,j), Left));
                            Right
                        },
                        Right => Right,
                        Left => Left,
                    }
                },
                _ => {continue;}
            };

            let next_coord = match next_dir {
                Up => if i>0 {(i-1, j)} else {(i,j)},
                Down => if i<n-1 {(i+1, j)} else {(i,j)},
                Left => if j>0 {(i,j-1)} else {(i,j)},
                Right => if j<m-1 {(i,j+1)} else {(i,j)},
            };

            stack.push((next_coord, next_dir));
        }
    }
    energized.len()
}

fn solve2(g: &Grid) -> usize {
    let n = g.len();
    let m = g[0].len();
    let mut max = 0;
    for i in 0..n {
        let r = solve(g, (i,0), Right);
        let l = solve(g, (i,m-1), Left);
        max = cmp::max(r, max);
        max = cmp::max(l, max);
    }
    for j in 0..m {
        let d = solve(g, (0,j), Down);
        let u = solve(g, (n-1,j), Up);
        max = cmp::max(d, max);
        max = cmp::max(u, max);
    }
    max
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let g = parse_grid(&input);

    let ans1 = solve(&g, (0,0), Right);
    println!("{}", ans1);

    let ans2 = solve2(&g);
    println!("{}", ans2);

    Ok(())
}

#[test]
fn test() {
    let input = fs::read_to_string("test.txt").unwrap();
    let g = parse_grid(&input);
    assert_eq!(solve(&g, (0,0), Right), 46);
    assert_eq!(solve2(&g), 51);
}
