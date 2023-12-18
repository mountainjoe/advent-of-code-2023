use std::{collections::HashMap, ops::RangeInclusive};

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

#[derive(Debug)]
enum Eng {
    Symbol { line_num: usize, spos: usize, sym: char },
    Number { line_num: usize, spos: usize, epos: usize, val: usize }
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
                Eng::Symbol { line_num: line, spos: col, sym: p.as_str().chars().next().unwrap() }
            },
            Rule::number => {
                let (line, col) = p.line_col();
                let len = p.as_str().len();
                Eng::Number { line_num: line, spos: col, epos: col + len - 1, val: p.as_str().parse().unwrap() }
            },
            _ => unreachable!()
        }
    }).partition(|e| match e {
        Eng::Number { line_num: _, spos: _, epos: _, val: _} => true,
        _ => false
    });
    //eprintln!("Engine nums: {:?}", engine_nums);
    //eprintln!("Engine syms: {:?}", engine_syms);

    // map<(line,pos),Option<char>> for symbol presence
    let symbol_map: HashMap<(usize,usize), char> = engine_syms.into_iter().map(
        |s| match s {
            Eng::Symbol { line_num, spos, sym } => ( (line_num,spos), sym ),
            _ => unreachable!() // we know we partitioned only the Eng::Symbols
        }).collect();
    
    // now iterate over the engine_nums,
    //   filter on "has_adjacent_symbol"
    //     which works by constructing an iterator of candidate positions,
    //          iter.any on map lookups for the symbol
    //   then sum everything
    let sum: usize = engine_nums.into_iter().filter(|n| has_adjacent_symbol(&symbol_map, n)).map(
        |n| match n {
            Eng::Number { line_num: _, spos: _, epos: _, val } => val,
            _ => unreachable!() // we know we patitioned only the Eng::Numbers
        }
    ).sum();

    eprintln!("sum is {}", sum);
}

fn has_adjacent_symbol(symbol_map: &HashMap<(usize,usize), char>, num: &Eng) -> bool {
    // don't need to worry about edges because first pos=1, and there won't
    // be any hits past the line width or pos=0.
    match num {
        Eng::Number { line_num, spos, epos, val: _ } => {
            let candidates = make_candidates(*line_num, *spos, *epos);
            candidates.iter().any(|t| symbol_map.contains_key(t))
        },
        _ => unreachable!()
    }
}

fn make_candidates(line: usize, spos: usize, epos: usize) -> Vec<(usize,usize)> {

    let prev_line = std::iter::repeat(line - 1);
    let lr_range = RangeInclusive::new(spos - 1, epos +1);
    let pz = prev_line.zip(lr_range);

    let next_line = std::iter::repeat(line + 1);
    let lr_range = RangeInclusive::new(spos - 1, epos +1);
    let nz = next_line.zip(lr_range);

    let mut r: Vec<_> = pz.chain(nz).collect();

    r.push((line, spos - 1));
    r.push((line, epos + 1));

    //eprintln!("candidates: {},{},{}: {:?}", line, spos, epos, r);

    r
}