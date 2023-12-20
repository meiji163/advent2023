use std::io;
use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug,Eq,PartialEq,Hash,Clone)]
enum Cmp {
    Greater,
    Less,
}
use Cmp::*;

#[derive(Debug,Eq,PartialEq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn get(&self, f: &String) -> usize {
        match f.as_str() {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => 0,
        }
    }
    fn get_mut(&mut self, f: &String) -> &mut usize {
        match f.as_str() {
            "x" => &mut self.x,
            "m" => &mut self.m,
            "a" => &mut self.a,
            _ => &mut self.s,
        }
    }
}

type Check = (String, Cmp, usize);

#[derive(Debug,Eq,PartialEq,Hash,Clone)]
struct Op {
    check: Option<Check>,
    next: String,
}

type Workflow = (String, Vec<Op>);

lazy_static! {
    static ref WORKFL_RE : Regex = Regex::new(r"(x|m|a|s)(>|<)(\d+):(\w+)").unwrap();
    static ref PART_RE : Regex = Regex::new(r"(x|m|a|s)=(\d+)").unwrap();
}

fn parse_workflow(s: &str) -> (String, Vec<Op>) {
    let mut ops : Vec<_> = Vec::new();
    let l_idx = s.chars().position(|c| c == '{').unwrap();
    let name = String::from(&s[..l_idx]);
    let op_strs : &Vec<&str> = &s[l_idx+1..s.len()-1].split(',').collect();

    for &op_str in op_strs.iter() {
        let op = match WORKFL_RE.captures_iter(op_str).last() {
            Some(cap) => {
                let arg1 = cap.get(1).unwrap().as_str();
                let cmp = match cap.get(2).unwrap().as_str() {
                    ">" => Greater,
                    _ => Less,
                };
                let arg2 : usize = cap.get(3).unwrap().as_str().parse().unwrap();
                let next_label = cap.get(4).unwrap().as_str();

                Op { check: Some((String::from(arg1), cmp, arg2)),
                     next: String::from(next_label) }
            },
            None => Op { check: None, next: String::from(op_str) }
        };
        ops.push(op);
    }
    (name, ops)
}

fn parse_part(s: &str) -> Part {
    let mut part = Part{x:0, m:0, a:0, s:0};
    for pstr in s.split(",") {
        if let Some(cap) = PART_RE.captures_iter(pstr).last() {
            let field = cap.get(1).unwrap().as_str();
            let val : usize = cap.get(2).unwrap().as_str().parse().unwrap();
            match field {
                "x" => part.x = val,
                "m" => part.m = val,
                "a" => part.a = val,
                "s" => part.s = val,
                _ => {}
            }
        }
    }
    part
}

fn apply_op(p: &Part, op: &Op) -> bool {
    match &op.check {
        Some((arg,cmp,val)) => {
            match cmp {
                Greater => p.get(arg) > *val,
                Less => p.get(arg) < *val,
            }
        },
        None => true,
    }
}

fn sort_part(p: &Part, wfs: &Vec<Workflow>) -> bool {
    let mut state = "in";
    while state != "A" && state != "R" {
        let (_, ops) = wfs.iter().find(|(name,_)| *name == state).unwrap();
        for op in ops.iter() {
            if apply_op(p, op) {
                state = op.next.as_str();
                break;
            } else {
                continue;
            }
        }
    }
    state == "A"
}

fn paths(wfs: &Vec<Workflow>) -> Vec<Vec<(String,usize)>>{
    let start = String::from("in");
    let mut found = vec![];
    let mut paths = vec![
        vec![(start.clone(),0)],
        vec![(start.clone(),1)],
    ];
    while !paths.is_empty() {
        let path = paths.pop().unwrap();
        let (name, num) = &path[path.len()-1];

        let (_, ops) = wfs.iter().find(|(n,_)| n == name).unwrap();
        let next = &ops[*num].next;
        if next == "A" {
            found.push(path);
            continue;
        } else if next == "R" {
            continue;
        }
        let (_, next_ops) = wfs.iter().find(|(n,_)| n == next).unwrap();
        for i in 0..next_ops.len() {
            let mut next_path = path.clone();
            next_path.push((next.clone(),i));
            paths.push(next_path);
        }
    }
    found
}

fn solve1(parts: &Vec<Part>, wfs: &Vec<Workflow>) -> usize {
    parts
        .iter()
        .filter(|&p| sort_part(p, &wfs))
        .map(|p| p.x + p.m + p.a + p.s)
        .sum()
}

fn check_path(wfs: &Vec<Workflow>, path: &Vec<(String,usize)>) -> (Part, Part){
    let mut max = Part {x: 4000, m: 4000, a: 4000, s: 4000};
    let mut min = Part {x: 1, m: 1, a: 1, s: 1};

    for (name, num) in path.iter() {
        let (_, ops) = wfs.iter().find(|(n,_)|  n == name).unwrap();
        // the first num-1 checks should fail
        for i in 0..*num {
            if let Some((arg,cmp,val)) = &ops[i].check {
                match cmp {
                    Greater => {
                        // part.arg <= val
                        let part_val = max.get_mut(arg);
                        if *part_val > *val {*part_val = *val;}
                    },
                    Less => {
                        // part.arg >= val
                        let part_val = min.get_mut(arg);
                        if *part_val < *val {*part_val = *val;}
                    }
                }
            }
        }
        // the last check should succeed
        if let Some((arg,cmp,val)) = &ops[*num].check {
            match cmp {
                Greater => {
                    // part.arg > val
                    let part_val = min.get_mut(arg);
                    if *part_val < *val {*part_val = *val + 1;}
                },
                Less => {
                    // part.arg < val
                    let part_val = max.get_mut(arg);
                    if *part_val > *val {*part_val = *val - 1;}
                },
            }
        }
    }
    (min, max)
}

fn solve2(wfs: &Vec<Workflow>) -> usize {
    paths(&wfs)
        .into_iter()
        .map(|p| {
            let (min, max) = check_path(&wfs, &p);
            if min.x > max.x || min.m > max.m || min.a > max.a || min.s > max.s {
                0
            } else {
                (max.x - min.x + 1)
                    * (max.m - min.m + 1)
                    * (max.a - min.a + 1)
                    * (max.s -min.s + 1)
            }
        }).sum()
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let (wf_str, part_str) = input.split_once("\n\n").unwrap();
    let wfs : Vec<Workflow> = wf_str.lines().map(parse_workflow).collect();
    let parts : Vec<Part> = part_str.lines().map(parse_part).collect();

    println!("{}", solve1(&parts, &wfs));
    println!("{}", solve2(&wfs));

    Ok(())
}

#[test]
fn test() {
    let workflows = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}";
    let parts = "\
{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    let wfs : Vec<Workflow> = workflows.lines().map(parse_workflow).collect();
    let parts : Vec<Part> = parts.lines().map(parse_part).collect();

    assert_eq!(19114, solve1(&parts, &wfs));
    assert_eq!(167409079868000, solve2(&wfs));
}
