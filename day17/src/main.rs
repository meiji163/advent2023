use std::io;
use std::fs;
use std::iter;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Grid = Vec<Vec<usize>>;
type Coord = (usize,usize);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Dir {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}
use Dir::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Coord,
    dir: Dir,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_grid(s: &str) -> Grid {
    s.lines().map(|l| {
        l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
    }).collect()
}

fn next_states(g: &Grid, s: &State) -> Vec<State> {
    let mut next = Vec::new();
    let State {cost, position, dir} = s;
    let &(i,j) = position;

    match dir {
        Up(n) => {
            if *n < 3 && i>0 {
                next.push(State {cost: cost + g[i-1][j],
                                 position: (i-1,j),
                                 dir: Up(n+1)});
            }
        },
        Down(n) => {
            if *n < 3 && i+1 < g.len() {
                next.push(State {cost: cost + g[i+1][j],
                                 position: (i+1,j),
                                 dir: Down(n+1)});
            }
        },
        Right(n) => {
            if *n < 3 && j+1 < g[0].len() {
               next.push(State {cost: cost + g[i][j+1],
                                position: (i,j+1),
                                dir: Right(n+1)});
            }
        },
        Left(n) => {
            if *n < 3 && j>0 {
                next.push(State {cost: cost + g[i][j-1],
                                 position: (i,j-1),
                                 dir: Left(n+1)});
            }
        },
    }

    match dir {
        Up(_) | Down(_) => {
            if j > 0 {
                next.push(State {cost: cost + (g[i][j-1] as usize),
                                 position: (i,j-1),
                                 dir: Left(1)});
            }
            if j + 1 < g[0].len() {
                next.push(State {cost: cost + (g[i][j+1] as usize),
                                 position: (i,j+1),
                                 dir: Right(1)});
            }
        },
        Right(_) | Left(_) => {
            if i > 0 {
                next.push(State {cost: cost + (g[i-1][j] as usize),
                                 position: (i-1,j),
                                 dir: Up(1)});
            }
            if i + 1 < g.len() {
                next.push(State {cost: cost + (g[i+1][j] as usize),
                                 position: (i+1,j),
                                 dir: Down(1)});
            }
        },
    }
    next
}

fn print_grid(g: &Grid, last: &Vec<Vec<Dir>>) {
    let n = g.len();
    let m = g[0].len();
    let mut grid : Vec<_> = iter::repeat_with(|| vec!['.'; m]).take(n).collect();

    let mut c = (n-1,m-1);
    while c != (0,0) {
        let (i,j) = c;
        match last[i][j] {
            Up(_) => {
                grid[i][j] = '^';
                c = (i+1,j);
            },
            Down(_) => {
                grid[i][j] = 'v';
                c = (i-1,j);
            },
            Left(_) => {
                grid[i][j] = '<';
                c = (i,j+1);
            },
            Right(_) => {
                grid[i][j] = '>';
                c = (i,j-1);
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            print!("{}", grid[i][j]);
        }
        print!("\n");
    }

    println!("{:?}", grid[0]);
}

fn search(g: &Grid) -> usize {
    let n = g.len();
    let m = g[0].len();
    let end = (n-1,m-1);

    let mut heap = BinaryHeap::new();
    let mut dist : Vec<_> = iter::repeat_with(|| vec![usize::MAX; m]).take(n).collect();
    let mut last : Vec<_> = iter::repeat_with(|| vec![Up(0); m]).take(n).collect();
    dist[0][0] = 0;
    heap.push(State { cost:0, position: (0,0), dir: Up(0)});
    //heap.push(State { cost:0, position: (0,0), dir: Down(1)});

    while let Some(state) = heap.pop() {
        let (i,j) = state.position;
        if state.position == end {
            print_grid(g, &last);
            return state.cost;
        }
        if state.cost > dist[i][j] { continue; }

        for next in &next_states(g, &state) {
            let (ni, nj) = next.position;
            if next.cost < dist[ni][nj] {
                heap.push(*next);
                dist[ni][nj] = next.cost;
                last[ni][nj] = next.dir;
            }
        }
    }
            print_grid(g, &last);
    return 0;
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let g = parse_grid(&input);
    println!("{:?}", search(&g));

    Ok(())
}

#[test]
fn test() {
    let s = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    let g = parse_grid(&s);

    for r in g.iter() {
        println!("{:?}", r);
    }

    assert_eq!(102, search(&g));

}
