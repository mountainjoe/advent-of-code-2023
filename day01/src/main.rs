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

    // idea for possibly more efficient strategy!
    // try searching from left, find a match, and then search for matches in substrings of that match...
    // which should traverse the string via regex reasonably efficiently (not as good as straight iterator,
    // but that didn't work)

    let regex_string = r"0|1|one|2|two|3|three|4|four|5|five|6|six|7|seven|8|eight|9|nine";
    let re_norm = Regex::new(regex_string).unwrap();
    let regex_string = regex_string.chars().rev().collect::<String>();
    let re_rev = Regex::new(&regex_string).unwrap();

    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        println!("input_line: {}", line);
        
        let value = process_line(&re_norm, &re_rev, &line);
        println!("  {}", value);
        sum += value;
    }

    println!("sum: {}", sum); // day 1 part 2: 53539 is correct answer
}

fn process_line<'a>(re_norm: &Regex, re_rev: &Regex, haystack: &'a str) -> u32 { // TODO return Result<> from everything

    // so I've got the digit matches, right?

    // first number, we want the first entry in the iterator
    let first = re_norm.find(haystack).unwrap().as_str();
    let haystack = haystack.chars().rev().collect::<String>();
    let last = re_rev.find(&haystack).unwrap().as_str().chars().rev().collect::<String>();


    println!("  {}, {}", first, last);
    tuple_to_value(&(first, &last))
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