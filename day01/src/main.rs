use std::fs::File;
use std::io::{BufRead,BufReader};

use clap::Parser;

use regex::Regex;

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

/* this fails

input_line: ttmtqrh3four4oneightrkv
  ("3","one")
    3, 1
  31
should be 38
 */

    // thought I cleverly added everything I needed
    let re = Regex::new(r"0|1|one|2|two|3|three|4|four|5|five|6|six|7|seven|8|eight|9|nine").unwrap();

    //let re = Regex::new(r"0|1|2|3|4|5|6|7|8|9").unwrap();

    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        println!("input_line: {}", line);
        
        let strings = process_line(&re, &line);
        println!("  ({:?},{:?})", strings.0, strings.1);

        let value = tuple_to_value(&strings);
        println!("  {}", value);
        sum += value;
    }

    println!("sum: {}", sum);
}

fn process_line<'a>(re: &Regex, haystack: &'a str) -> (&'a str,&'a str) { // TODO return Result<> from everything

    let mut matches = re.find_iter(haystack);
    // so I've got the digit matches, right?

    // first number, we want the first entry in the iterator
    let first = matches.next().unwrap().as_str();
    let mut last = first;

    // this could be improved, but would have to progressively generate haystacks from the back end
    // now we need the last entry
    while let Some(m) = matches.next() {
        last = m.as_str();
    }

    (first, last)
}

fn tuple_to_value(t: &(&str,&str)) -> u32 {
    println!("    {}, {}", str_to_num(t.0), str_to_num(t.1));
    10 * str_to_num(t.0) + str_to_num(t.1)
}

fn str_to_num(numstr: &str) -> u32 {
    match numstr {
        "0" => 0,
        "zero" => 0,
        "1" => 1,
        "one" => 1,
        "2" => 2,
        "two" => 2,
        "3" => 3,
        "three" => 3,
        "4" => 4,
        "four" => 4,
        "5" => 5,
        "five" => 5,
        "6" => 6,
        "six" => 6,
        "7" => 7,
        "seven" => 7,
        "8" => 8,
        "eight" => 8,
        "9" => 9,
        "nine" => 9,
        _ => panic!("illegal input")
    }
}