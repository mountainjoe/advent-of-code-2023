
use std::{collections::{HashMap, HashSet}, ops::RangeInclusive};

use clap::Parser as _;
use pest::Parser;



#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    puzzle_input: String
}

#[derive(pest_derive::Parser)]
#[grammar = "engine.pest"]
pub struct EngineParser;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct EngineNumber { line_num: usize, spos: usize, epos: usize, val: usize }

#[derive(Debug)]
struct EngineSymbol { line_num: usize, spos: usize, sym: char }

#[derive(Debug)]
enum Eng {
    Symbol(EngineSymbol), 
    Number(EngineNumber)
}

#[derive(Debug)]
struct Gear {
    r1: usize,
    r2: usize,
    line_num: usize,
    col_num: usize
}

impl Gear {
    fn new(pos: (usize,usize), r1: usize, r2: usize) -> Self {
        Gear {
            r1,
            r2,
            line_num: pos.0,
            col_num: pos.1
        }
    }
}

fn main() {
    let args = Args::parse();
    let engine_text = std::fs::read_to_string(args.puzzle_input).unwrap();

    let ep = EngineParser::parse(Rule::engine, &engine_text).expect("bad parse");
    //eprintln!("Engine: {:?}", ep);
    let (engine_nums, engine_syms): (Vec<_>, Vec<_>) = ep.map(|p| {
        match p.as_rule() {
            Rule::sym => {
                let (line, col) = p.line_col();
                Eng::Symbol(EngineSymbol { line_num: line, spos: col, sym: p.as_str().chars().next().unwrap() })
            },
            Rule::number => {
                let (line, col) = p.line_col();
                let len = p.as_str().len();
                Eng::Number(EngineNumber{ line_num: line, spos: col, epos: col + len - 1, val: p.as_str().parse().unwrap()})
            },
            _ => unreachable!()
        }
    }).partition(|e| match e {
        Eng::Number(_) => true,
        _ => false
    });

    let engine_nums: Vec<_> = engine_nums.into_iter().map(|e| match e {
        Eng::Number(en) => en,
        _ => unreachable!()
    }).collect();
    //eprintln!("Engine nums: {:?}", engine_nums);

    let engine_syms: Vec<_> = engine_syms.into_iter().map(|e| match e {
        Eng::Symbol(s) => s,
        _ => unreachable!()
    }).collect();
    //eprintln!("Engine syms: {:?}", engine_syms);

    // now looking for * characters adjacent to exactly two numbers.

    // I'm thinking I put an Eng::Number into a hashmap for all of its positions
    let nums_map = create_engine_number_map(&engine_nums);
    //eprintln!("EngineNumberMap: {:#?}", nums_map);

    let stars: Vec<_> = engine_syms.into_iter().filter(|s| s.sym == '*').collect();
    //eprintln!("stars: {:#?}", stars);

    // for each star
    //   find all adjacent numbers, 
    //   // then return tuple of two numbers, or none... 
    let gears: Vec<Option<Gear>> = stars.into_iter().map(|s| star_to_maybe_gear(s, &nums_map)).collect();
    //eprintln!("Gears: {:#?}", gears);

    let sum = gears.iter().fold(0, |acc,g| match g {
        Some(Gear { r1, r2, line_num: _, col_num: _}) => acc + r1*r2,
        None => acc
    });

    eprintln!("sum is {}", sum); 
// 69113781 was too low; I used `break` in my for loop to generate candidates, always skipping candidate to right of *
// 81939900 is correct answer
}

fn create_engine_number_map(engine_nums: &Vec<EngineNumber>) -> HashMap<(usize,usize), EngineNumber> {
    let mut r = HashMap::new();

    for n in engine_nums {
        // insert the EngineNumber for every position it could be in.
        for col in std::ops::RangeInclusive::new(n.spos, n.epos) {
            r.insert((n.line_num, col), EngineNumber {..*n});
        }
    }

    r
}

fn candidates(es: &EngineSymbol) -> Vec<(usize,usize)> {
    let mut r = Vec::new();

    for l in RangeInclusive::new(es.line_num - 1, es.line_num + 1) {
        for c in RangeInclusive::new(es.spos - 1, es.spos + 1) {
            if !(l == es.line_num && c == es.spos) {
                r.push((l,c));
            }
        }
    }

    r
}

fn star_to_maybe_gear(es: EngineSymbol, nm: &HashMap<(usize,usize),EngineNumber>) -> Option<Gear> {
    let candidates = candidates(&es);
    //eprintln!("candidates: {:?}", candidates);
    let mut match_set = HashSet::<EngineNumber>::new();

    for c in candidates {
        if let Some(n) = nm.get(&c) {
            //eprintln!("candidate match {:?}", n);
            match_set.insert(n.clone());
        }
    }

    if match_set.len() == 2 {
        let mut it = match_set.iter();
        let r1 = it.next().unwrap(); // we know there are exactly two elements
        let r2 = it.next().unwrap(); 
        
        Some(Gear::new( (es.line_num, es.spos), r1.val, r2.val))
    } else {
        eprintln!("rejected: {:?} {:?}", es, match_set);
        None
    }
}