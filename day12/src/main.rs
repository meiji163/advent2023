use std::fs;
use std::io;
use std::iter;

type Row = (Vec<char>, Vec<usize>);

fn parse_row(s: &str) -> Option<Row> {
    let (springs_str, groups_str) = s.split_once(' ')?;
    let groups: Vec<usize> = groups_str
        .split(',')
        .filter_map(|d| d.parse().ok())
        .collect();
    let springs: Vec<char> = springs_str.chars().collect();
    Some((springs, groups))
}

fn is_placeable(spring: &Vec<char>, i: usize, group: usize) -> bool {
    (i + group - 1 < spring.len())
        && (i + group == spring.len() || spring[i + group] != '#')
        && spring[i..i + group].iter().all(|c| *c != '.')
}

fn solve(row: &Row) -> usize {
    let (springs, group) = row;
    let n_spr = springs.len();
    let n_grp = group.len();

    // counts[n][i] = # configurations with n groups, where nth group starts at position i
    let mut counts: Vec<Vec<usize>> = iter::repeat_with(|| vec![0; n_spr]).take(n_grp).collect();
    for i in 0..n_spr {
        if is_placeable(&springs, i, group[0]) {
            counts[0][i] = 1;
        }
        if springs[i] == '#' {
            break;
        }
    }

    for n in 1..n_grp {
        // find count[n][j] from count[n-1][i]...
        for i in 0..n_spr {
            let count = counts[n - 1][i];
            if count != 0 {
                for j in (i + group[n - 1] + 1)..n_spr {
                    if is_placeable(&springs, j, group[n]) {
                        counts[n][j] += count;
                    }
                    if springs[j] == '#' {
                        break; // stop because we reached the nth group
                    }
                }
            }
        }
    }

    // discount ones with more than n groups
    let n = n_grp - 1;
    for i in 0..n_spr {
        let grp = group[n];
        if i + grp <= n_spr && springs[(i + grp)..].contains(&'#') {
            counts[n][i] = 0;
        }
    }
    counts[n].iter().sum()
}

fn quintuple(row: &Row) -> Row {
    let mut spring: Vec<char> = row.0.clone();
    spring.push('?');
    let slen = spring.len();
    let spring_5 = spring.into_iter().cycle().take(5 * slen - 1).collect();

    let groups = &row.1;
    let groups_5: Vec<usize> = groups
        .clone()
        .into_iter()
        .cycle()
        .take(5 * groups.len())
        .collect();
    (spring_5, groups_5)
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let rows: Vec<Row> = input.lines().filter_map(|l| parse_row(l)).collect();

    let ans: usize = rows.iter().map(|r| solve(r)).sum();
    println!("{}", ans);

    let ans2: usize = rows.iter().map(|r| solve(&quintuple(r))).sum();
    println!("{}", ans2);

    Ok(())
}

#[test]
fn test() {
    let s = "?###???????? 3,2,1";
    let row = quintuple(&parse_row(s).unwrap());
    assert_eq!(solve(&row), 506250);
}
