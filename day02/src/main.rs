use std::fs::File;
use std::io::{BufRead,BufReader};

use clap::Parser as _;
use pest::iterators::Pairs;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    puzzle_input: String
}

#[derive(pest_derive::Parser)]
#[grammar = "marbles.pest"]
pub struct MarblesParser;


#[derive(Debug)]
struct Game {
    num: u32,
    pulls: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32
}

impl Round {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0
        }
    }

    fn add(&self, p: Pull) -> Self {
        Self {
            red: self.red + match p {
                Pull::Red(n) => n,
                _ => 0
            },
            green: self.green + match p {
                Pull::Green(n) => n,
                _ => 0
            },
            blue: self.blue + match p {
                Pull::Blue(n) => n,
                _ => 0
            }
        }
    }
}

#[derive(Debug)]
enum Pull {
    Red(u32),
    Green(u32),
    Blue(u32)
}


fn main() {
    let args = Args::parse();

    let file = File::open(&args.puzzle_input);
    if let Err(e) = file {
        eprintln!("Failed to open {:?}: {:?}", &args.puzzle_input, e);
        return
    }

    let file = file.unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(g) = parse_game(&line.unwrap()) {
            println!("{:?}", g);
        }
    }
}

fn parse_game(line: &str) -> Result<Game,String> {
    use pest::Parser;

    match MarblesParser::parse(Rule::game, &line) {
        Ok(mut pairs) => {
            // so this is the game
            let pair = pairs.next().unwrap();
            let mut inner = pair.into_inner();
            //println!("inner: {}", &inner);
            // inner is a sequence of pairs: number, marbseq+
            let n: u32 = inner.next().unwrap().as_str().parse().unwrap();
            let v = game_marble_sequences(inner);

            Ok(Game {
                num: n,
                pulls: v
            })
        },
        Err(error) => {
            eprintln!("Error: {:?}", error);
            Err(String::from("some error printed"))
        }
    }
}

fn game_marble_sequences(game_pairs: Pairs<'_, Rule>) -> Vec<Round> {
    game_pairs.map(|p| marble_sequence(p.into_inner())).collect()
}

fn marble_sequence(p: Pairs<'_,Rule>) -> Round {
    p.map(|p| marble_pull(p.into_inner())).fold(Round::new(), |acc, p| {
        acc.add(p)
    })
}

fn marble_pull(mut p: Pairs<'_,Rule>) -> Pull {
    let n: u32 = p.next().unwrap().as_str().parse().unwrap();

    match p.next().unwrap().into_inner().next().unwrap().as_rule() {
        Rule::red => Pull::Red(n),
        Rule::blue => Pull::Blue(n),
        Rule::green => Pull::Green(n),
        _ => unreachable!()
    }
}