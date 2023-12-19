use std::collections::HashSet;

use clap::Parser as _;
use pest::{Parser, iterators::{Pairs, Pair}};

#[derive(clap::Parser)]
#[command(author,version,about,long_about=None)]
struct Args {
    puzzle_input: String
}

#[derive(pest_derive::Parser)]
#[grammar = "cards.pest"]
pub struct CardsParser;

fn main() {
    let args = Args::parse();
    let cards_text = std::fs::read_to_string(args.puzzle_input).unwrap();

    let cp = CardsParser::parse(Rule::cards, &cards_text).expect("bad parse");
    //eprintln!("pairz: {}", cp);

    let cards = parse_pairs(cp);
    //eprintln!("cards: {:?}", cards);

    // ok, now we just do scoring
    let scores: Vec<_> = cards.iter().map(|c| score_card(c)).collect();
    eprintln!("scores: {:?}", scores);

    let sum: usize = scores.iter().sum();
    eprintln!("sum: {}", sum);
}

#[derive(Debug)]
struct Card {
    n: usize,
    winners: HashSet<usize>,
    ours: Vec<usize>
}

fn parse_pairs(ps: Pairs<Rule>) -> Vec<Card> {

    ps.map(|p| card_from_pair(p.into_inner())).collect()
}

fn card_from_pair(mut p: Pairs<Rule>) -> Card {
    //eprintln!("card: {}", p);
    
    // first thing is the card number
    let card_number: usize = p.next().unwrap().as_str().parse().unwrap();
    p.next(); // colon

    let mut winners = HashSet::<usize>::new();
    loop {
        let pair = p.next().unwrap(); // we know that there are numbers until a bar
        match pair.as_rule() {
            Rule::number => {
                winners.insert(pair.as_str().parse().unwrap());
                ()
            },
            Rule::bar => break,
            _ => unreachable!()
        }
    }
    
    let ours: Vec<usize> = p.map(|n| n.as_str().parse().unwrap()).collect();

    Card {
        n: card_number,
        winners,
        ours
    }
}

fn score_card(card: &Card) -> usize {
    // first map ours to wins
    let wins = card.ours.iter().filter(|n| card.winners.contains(n)).count();
    match wins {
        0 => 0,
        1 => 1,
        _ => {
            2_usize.pow( (wins - 1) as u32)
        }
    }
}