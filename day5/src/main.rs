use itertools::Itertools;
use std::fs;
use std::io;
use std::str;

type Interval = (u64, u64);
type RangeMap = Vec<[u64; 3]>;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    ranges: Vec<RangeMap>,
}

fn parse_range(l: &str) -> Option<[u64; 3]> {
    let v: Vec<u64> = l.split(' ').filter_map(|d| d.parse().ok()).collect();
    if v.len() != 3 {
        None
    } else {
        Some([v[0], v[1], v[2]])
    }
}

fn parse_almanac(inp: &str) -> Option<Almanac> {
    let (seed_str, rest) = inp.split_once("\n\n")?;
    let seeds: Vec<u64> = seed_str
        .split(": ")
        .last()?
        .split(" ")
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut ranges: Vec<RangeMap> = Vec::new();
    for r_str in rest.split("\n\n") {
        let mut range: RangeMap = r_str
            .lines()
            .skip(1)
            .filter_map(|l| parse_range(l))
            .collect();
        range.sort_by(|r1, r2| r1[1].cmp(&r2[1]));

        ranges.push(range);
    }
    Some(Almanac { seeds, ranges })
}

fn query_map(rm: &RangeMap, key: u64) -> u64 {
    for range in rm.iter() {
        let &[dest, src, len] = range;
        if key >= src && key < src + len {
            return dest + (key - src);
        }
    }
    key
}

// Map an interval given the range map.
fn query_map_range(rm: &RangeMap, invl: Interval) -> Vec<Interval> {
    let mut invls: Vec<Interval> = Vec::new();
    let (mut lo, hi) = invl;
    for range in rm.iter() {
        let &[dest, src, len] = range;
        // case equal
        if lo == src && hi == src + len {
            invls.push((dest, dest + len));
            return invls;
        }
        // case [ (  ) ]
        else if (lo >= src && lo < src + len) && (hi >= src && hi < src + len) {
            invls.push((dest + (lo - src), dest + (hi - src)));
            return invls;
        }
        // case ( [   ] )
        else if lo <= src && hi >= src + len {
            invls.push((lo, src - 1));
            invls.push((dest, dest + len));
            lo = src + len;
        }
        // case ( [---) ]
        else if lo < src && hi >= src && hi < src + len {
            invls.push((lo, src - 1));
            invls.push((dest, dest + (hi - src)));
            return invls;
        }
        // case [ (---] )
        else if lo >= src && lo < src + len && hi > src + len {
            invls.push((dest + (lo - src), dest + len));
            lo = src + len;
        }
    }
    if invls.is_empty() {
        vec![invl]
    } else {
        invls
    }
}

fn seed_location(ranges: &Vec<RangeMap>, seed: u64) -> u64 {
    ranges.iter().fold(seed, |k, rng| query_map(rng, k))
}

fn interval_location(ranges: &Vec<RangeMap>, seed: Interval) -> Vec<Interval> {
    let invls: Vec<Interval> = vec![seed];
    return ranges.iter().fold(invls, |is, rng| {
        is.iter().map(|i| query_map_range(rng, *i)).concat()
    });
}

fn solve1(alm: &Almanac) -> u64 {
    return alm
        .seeds
        .iter()
        .map(|s| seed_location(&alm.ranges, *s))
        .min()
        .unwrap_or(0);
}

fn solve2(alm: &Almanac) -> u64 {
    let invls: Vec<Interval> = alm
        .seeds
        .iter()
        .tuples::<(_, _)>()
        .map(|(&start, &len)| (start, start + len - 1))
        .collect();
    let locs: Vec<Interval> = invls
        .into_iter()
        .map(|invl| interval_location(&alm.ranges, invl))
        .concat();
    return locs.into_iter().map(|(lo, _)| lo).min().unwrap();
}

fn main() -> io::Result<()> {
    let inp_str = fs::read_to_string("input.txt")?;
    let alm = parse_almanac(&inp_str).unwrap();
    println!("{}", solve1(&alm));
    println!("{}", solve2(&alm));

    Ok(())
}
