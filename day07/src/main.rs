use clap::Parser as _;
use pest::{Parser, iterators::Pairs};

#[derive(clap::Parser)]
#[command(author,version,about,long_about=None)]
struct Args {
    puzzle_input: String
}

#[derive(pest_derive::Parser)]
#[grammar = "hands.pest"]
pub struct HandsParser;

fn main() {
    let args = Args::parse();
    let hands_text = std::fs::read_to_string(args.puzzle_input).unwrap();

    let ps = HandsParser::parse(Rule::Hands, &hands_text).expect("bad parse");
    
    //eprintln!("pairs: {}", ps);

    let hands: Vec<_> = ps.map(|p| Hand::from(p.into_inner())).collect();
    eprintln!("Hands: {:#?}", hands);
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Hand {
    cards: [Card;5], // derived partial ord _should_ be left-to-right which matches problem spec
    bid: usize,
}

impl Hand {
    fn from(mut ps: Pairs<Rule>) -> Self {
        // expects an "inner"
        Self {
            cards: [
                Card::from(ps.next().unwrap().as_str()),
                Card::from(ps.next().unwrap().as_str()),
                Card::from(ps.next().unwrap().as_str()),
                Card::from(ps.next().unwrap().as_str()),
                Card::from(ps.next().unwrap().as_str()),
            ],
            bid: ps.next().unwrap().as_str().parse().unwrap()
        }
    }

    fn hand_type(&self) -> HandType {
        HandType::HighCard // this isn't right
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Card {
    Ace = 13,
    King = 12,
    Queen = 11,
    Jack = 10,
    Ten = 9,
    Nine = 8,
    Eight = 7,
    Seven = 6,
    Six = 5,
    Five = 4,
    Four = 3,
    Three = 2,
    Two = 1
}

impl Card {
    fn from(c: &str) -> Card {
        match c {
            "A" => Card::Ace,
            "K" => Card::King,
            "Q" => Card::Queen,
            "J" => Card::Jack,
            "T" => Card::Ten,
            "9" => Card::Nine,
            "8" => Card::Eight,
            "7" => Card::Seven,
            "6" => Card::Six,
            "5" => Card::Five,
            "4" => Card::Four,
            "3" => Card::Three,
            "2" => Card::Two,
            _ => panic!("Invalid card {}", c)
        }
    }
}