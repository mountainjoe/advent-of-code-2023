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
    rounds: Vec<Round>,
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

    let games = reader.lines().map(|res| parse_game(&res.unwrap()).unwrap()).collect::<Vec<_>>();
    //eprintln!("{:?}", games);

    // this is the first part of day02
    //let sum: u32 = games.iter().filter(|g| is_legal_game(&g)).map(|g| g.num).sum();

    let power_sum: u32 = games.iter().map(|g| game_power(&g)).sum();

    println!("power sum: {}", power_sum);
}

fn game_power(game: &Game) -> u32 {
    let max_round = game.rounds.iter().fold(Round::new(), |acc, r| {
        Round {
            red: std::cmp::max(acc.red, r.red),
            green: std::cmp::max(acc.green, r.green),
            blue: std::cmp::max(acc.blue, r.blue)
        }
    });

    let power_sum = max_round.red * max_round.green * max_round.blue;
    //eprintln!("Game {} Max Round = {:?} Power = {}", game.num, max_round, power_sum);

    power_sum    
}

fn _is_legal_game(game: &Game) -> bool {
    let limit_round = Round {
        red: 12,
        green: 13,
        blue: 14
    };

    game.rounds.iter().all(|r| {
        r.red <= limit_round.red &&
        r.green <= limit_round.green &&
        r.blue <= limit_round.blue
    })
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
                rounds: v
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