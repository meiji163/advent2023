use std::io;
use std::fs;
use std::collections::{HashMap,VecDeque};

type Graph = HashMap<usize,Vec<usize>>;

#[derive(Debug,Eq,PartialEq,Clone,Hash)]
enum Module {
    FlipFlop { id: usize, name: String, state: bool },
    Conjunction { id: usize, name: String, state: Vec<(usize,bool)> },
    Broadcast { id: usize, name: String },
    Out { id: usize, name: String },
}
use Module::*;

impl Module {
    fn input(&mut self, port: usize, inp: bool) -> Option<bool> {
        match self {
            FlipFlop { state, ..}=> {
                if inp {
                    None
                } else {
                    *state= !*state;
                    Some(*state)
                }
            },
            Conjunction {state, ..} => {
                let (_, b) = state.iter_mut().find(|(id,_)| *id == port).unwrap();
                *b = inp;
                Some(state.iter().any(|(_,b)| !*b))
            },
            Broadcast {..} | Out {..} => {Some(inp)}
        }
    }
    fn id(&self) -> usize {
        match self {
            FlipFlop {id, ..}
            | Conjunction {id, ..}
            | Broadcast {id, ..}
            | Out {id, ..} => *id
        }
    }
    fn name(&self) -> &String {
        match self {
            FlipFlop {name, ..}
            | Conjunction {name, ..}
            | Broadcast {name, ..}
            | Out {name, ..} => name,
        }
    }
}

fn parse_module(id: usize, s: &str) -> Module {
    if s == "broadcaster" {
        return Broadcast{ id, name: s.to_string()};
    }
    let (prefix, name) = s.split_at(1);
    match prefix {
        "%" => FlipFlop{ id, name: name.to_string(), state: false },
        "&" => Conjunction { id, name: name.to_string(), state: Vec::new() },
        _ => {panic!("unknown module")}
    }
}

fn parse(s: &str) -> (Vec<Module>, Graph) {
    let mut outputs : Graph = HashMap::new();
    let mut inputs : Graph = HashMap::new();
    let lines : Vec<(&str,&str)> = s.lines().map(|s| s.split_once(" -> ").unwrap()).collect();
    // give the modules integer IDs
    let mut modules : Vec<Module> = lines
        .iter()
        .enumerate()
        .map(|(i, (left,_))| parse_module(i, left))
        .collect();
    for i in 0..modules.len() {
        let (_, out_str) = lines[i];
        let out_names : Vec<String> = out_str.split(", ").map(|s| s.to_string()).collect();
        let mut out_ids = vec![];
        for name in out_names.iter() {
            if let Some(j) = modules.iter().position(|m| m.name() == name) {
                out_ids.push(j);
            } else {
                // output node
                let last_id = modules[modules.len()-1].id();
                let out_mod = Out {id: last_id+1, name: name.to_string()};
                out_ids.push(out_mod.id());
                modules.push(out_mod);
            }
        }
        for id in out_ids.iter() {
            if let Some(ins) = inputs.get_mut(&id) {
                ins.push(i);
            } else {
                inputs.insert(*id, vec![i]);
            }
        }
        outputs.insert(i,out_ids);
    }
    // init conjunctions
    for m in modules.iter_mut() {
        if let Conjunction {id, state, ..} = m {
            *state = inputs
                .get(id)
                .unwrap()
                .iter()
                .map(|i| (*i,false))
                .collect();
        }
    }
    (modules, outputs)
}

fn send_pulse(in_id: usize, mods: &mut Vec<Module>, outs: &Graph) -> (usize,usize,bool) {
    let mut hit_rx = false;
    let mut lo = 0;
    let mut hi = 0;
    let mut q: VecDeque<(usize,usize,bool)> = VecDeque::new();
    q.push_back((0, in_id, false));
    while !q.is_empty() {
        let (port, id, pulse) = q.pop_front().unwrap();
        // if pulse {
        //     hi += 1;
        // } else {
        //     lo += 1;
        // }
        let module = &mut mods[id];
        if let Out {name, ..} = module {
            if name == "rx" && !pulse { hit_rx = true; }
            continue;
        }
        if let Some(out) = module.input(port, pulse) {
            for &next_id in outs.get(&id).unwrap() {
                q.push_back((id, next_id, out));
            }
        }
    }
    (lo, hi, hit_rx)
}

fn solve1(mods: &mut Vec<Module>, outs: &Graph ) -> usize {
    let mut los = 0;
    let mut his = 0;
    let start_id = mods
        .iter()
        .position(|m| m.name().as_str() == "broadcaster")
        .unwrap();
    for _ in 0..1000 {
        let (lo, hi, _) = send_pulse(start_id, mods, outs);
        los += lo;
        his += hi;
    }
    los * his
}

fn solve2(mods: &mut Vec<Module>, outs: &Graph) -> usize {
    let start_id = mods
        .iter()
        .position(|m| m.name().as_str() == "broadcaster")
        .unwrap();
    let mut n = 0;
    loop {
        if n % 100000 == 0 {println!("{}", n);}
        let (_, _, hit_rx) = send_pulse(start_id, mods, outs);
        n += 1;
        if hit_rx {break;}
    }
    n
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let (mut mods, outs) = parse(&input);
    //println!("{}", solve1(&mut mods, &outs));
    println!("{}", solve2(&mut mods, &outs));

    Ok(())
}

#[test]
fn test() {
    let s1 = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    let s2 = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    let (mut mods1, outs1) = parse(s1);
    assert_eq!(solve1(&mut mods1, &outs1), 4000*8000);

    let (mut mods2, outs2) = parse(s2);
    assert_eq!(solve1(&mut mods2, &outs2), 4250*2750);
}
