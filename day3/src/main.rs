use std::fs;
use std::io;
use std::iter;
use std::str;

type Grid = Vec<Vec<char>>;
type Coord = (usize, usize);

struct Schematic {
    grid: Grid,
    spans: Vec<Vec<Coord>>,
    nums: Vec<Vec<u32>>,
}

fn parse_schematic(inp: &str) -> Schematic {
    let g: Grid = inp.lines().map(|l| l.chars().collect()).collect();
    let n = g.len();
    let m = g[0].len();

    let mut nums: Vec<Vec<u32>> = iter::repeat_with(|| Vec::<u32>::new()).take(n).collect();
    let mut spans: Vec<Vec<Coord>> = iter::repeat_with(|| Vec::<Coord>::new()).take(n).collect();

    let (mut i, mut j) = (0, 0);

    while i < n && j < m {
        match next_num(&g, (i, j)) {
            None => break,
            Some(c) => (i, j) = c,
        };

        let num_j = parse_num(&g[i], j);
        let len = num_j.1;
        nums[i].push(num_j.0);
        spans[i].push((j, j + len));
        j += len;
        if j >= m {
            i += 1;
            j = 0;
        }
    }
    Schematic {
        grid: g,
        spans: spans,
        nums: nums,
    }
}

fn next_num(g: &Grid, c: Coord) -> Option<Coord> {
    let n = g.len();
    let m = g[0].len();
    let (mut i, mut j) = c;

    while !g[i][j].is_digit(10) {
        j += 1;
        if j >= m {
            i += 1;
            j = 0;
        }
        if i >= n {
            return None;
        }
    }
    Some((i, j))
}

fn parse_num(v: &Vec<char>, j: usize) -> (u32, usize) {
    let mut n = 0;
    while j + n < v.len() && v[j + n].is_digit(10) {
        n += 1;
    }
    let num = v[j..(j + n)]
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    (num, n)
}

fn is_symbol(chr: char) -> bool {
    return chr != '.' && !chr.is_digit(10);
}

fn is_adj_to_sym(g: &Grid, i: usize, span: &Coord) -> bool {
    let (j0, j1) = *span;
    let start = if j0 > 0 { j0 - 1 } else { j0 };
    let end = if j1 < g[i].len() { j1 } else { j1 - 1 };

    for n in start..=end {
        if i > 0 && is_symbol(g[i - 1][n]) {
            return true;
        }
        if is_symbol(g[i][n]) {
            return true;
        }
        if i + 1 < g.len() && is_symbol(g[i + 1][n]) {
            return true;
        }
    }
    false
}

fn find_nums_idx(s: &Schematic, c: &Coord) -> Option<Coord> {
    let (i, j) = *c;
    if i >= s.grid.len() || j >= s.grid[0].len() {
        return None;
    }
    for (n, span) in s.spans[i].iter().enumerate() {
        let (j0, j1) = *span;
        if j >= j0 && j < j1 {
            return Some((i, n));
        }
    }
    None
}

fn solve1(s: &Schematic) -> u32 {
    let mut ans = 0;
    let n = s.grid.len();
    for i in 0..n {
        for (j, span) in s.spans[i].iter().enumerate() {
            if is_adj_to_sym(&s.grid, i, span) {
                ans += s.nums[i][j];
            }
        }
    }
    ans
}

fn solve2(s: &Schematic) -> u32 {
    let mut sum: u32 = 0;
    let grd = &s.grid;
    let n = grd.len();
    let m = grd[0].len();
    for i in 0..n {
        for j in 0..m {
            if grd[i][j] != '*' {
                continue;
            }

            let adj_idxs = vec![
                (i, j + 1),
                (i, j - 1),
                (i + 1, j),
                (i - 1, j),
                (i + 1, j + 1),
                (i + 1, j - 1),
                (i - 1, j - 1),
                (i - 1, j + 1),
            ];
            let mut idxs = Vec::new();
            for adj_idx in adj_idxs.iter() {
                match find_nums_idx(&s, adj_idx) {
                    Some(num_idx) => {
                        idxs.push(num_idx);
                    }
                    None => {}
                }
            }

            idxs.sort_unstable();
            idxs.dedup();
            if idxs.len() == 2 {
                sum += idxs.into_iter().map(|(i, j)| s.nums[i][j]).product::<u32>();
            }
        }
    }
    sum
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let s = parse_schematic(&input);

    //println!("{}", solve1(&s));
    println!("{}", solve2(&s));

    Ok(())
}
