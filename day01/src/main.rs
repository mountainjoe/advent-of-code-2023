use std::fs::File;
use std::io::{BufRead,BufReader};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    puzzle_input: String
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

    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        println!("input_line: {}", line);
        let twochars = line_to_two_chars(&line);
        println!("twochars: {:?}", twochars);
        let num = two_chars_to_number(&twochars);
        println!("value: {:?}", num);
        sum += num;
    }

    println!("sum: {}", sum);
}

// convert a text line into the two characters of interest
fn line_to_two_chars(line: &str) -> (char, char) {
    let s_index = line.find(|c: char| c.is_ascii_digit()).unwrap(); // panic if not found; presume good input
    let e_index = line.rfind(|c: char| c.is_ascii_digit()).unwrap();

    (
        char::from_u32(line.as_bytes()[s_index] as u32).unwrap(),
        char::from_u32(line.as_bytes()[e_index] as u32).unwrap()
    )
}

// take two characters and turn them into the desired number (concatenate)
fn two_chars_to_number(two_chars: &(char,char)) -> u32 {
    10 * two_chars.0.to_digit(10).unwrap() + two_chars.1.to_digit(10).unwrap()
}