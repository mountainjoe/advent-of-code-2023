
use clap::{Parser,ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum InputType {
    Example, Input
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    input_type: InputType
}

#[derive(Debug, Clone, Copy)]
struct RaceParam {
    time: u32,
    distance: u32
}

const EXAMPLE_RACE_PARAMS: [RaceParam; 3] = [
    RaceParam{ time: 7, distance: 9},
    RaceParam{ time: 15, distance: 40},
    RaceParam { time: 30, distance: 200},
];

const INPUT_RACE_PARAMS: [RaceParam; 4] = [
    RaceParam { time: 59, distance: 597 },
    RaceParam { time: 79, distance: 1234 },
    RaceParam { time: 65, distance: 1032 },
    RaceParam { time: 75, distance: 1328 },
];

fn main() {
    let args = Args::parse();

    let input = match args.input_type {
        InputType::Example => EXAMPLE_RACE_PARAMS.to_vec(),
        InputType::Input => INPUT_RACE_PARAMS.to_vec()
    };

    eprintln!("input: {:#?}", input);

    let result: Vec<_> = input.iter().enumerate().map(|(_i,race)| {
        (0..(1+race.time)).map(|push_time| {
            race_distance(push_time, race.time) > race.distance
        }).filter(|b| *b).count()
    }).collect();

    eprintln!("result: {:#?}", result);

    // ultimately we're converting each race into an array of booleans which we count... 

    // but for some versions of this it'd be faster to find the solution
    // (push_time * (total_time - push_time) > distance)
    // -push_time**2 + push_time * total_time > distance
    // push_time**2 - push_time * total_time < distance
    // push_time**2 - push_time*total_time - distance < 0
    // x^2 - tx - d == 0 --> search around the solution to this quadratic equation?

    let product: usize = result.iter().product();
    eprintln!("final result: {}", product); // day06part1 = 220320, brute force 228ms
}

fn race_distance(push_time: u32, total_time: u32) -> u32 {
    assert!(push_time <= total_time);

    push_time * (total_time - push_time)
}
