
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
    distance: u64
}

const EXAMPLE_RACE_PARAMS: [RaceParam; 1] = [
    RaceParam{ time: 71530, distance: 940200},
];

const INPUT_RACE_PARAMS: [RaceParam; 1] = [
    RaceParam { time: 59796575, distance: 597123410321328 },
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
            race_distance(push_time.into(), race.time.into()) > race.distance
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
    // day06part1 = 220320, brute force 228ms
    // day06part2 = 34454850, brute force 2.477s
    eprintln!("final result: {}", product); 

}

fn race_distance(push_time: u64, total_time: u64) -> u64 {
    assert!(push_time <= total_time);

    push_time * (total_time - push_time)
}
