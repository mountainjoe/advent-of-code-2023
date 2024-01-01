use std::ops::Range;

use clap::Parser as _;
use pest::{Parser, iterators::Pairs};

#[derive(clap::Parser)]
#[command(author,version,about,long_about=None)]
struct Args {
    puzzle_input: String
}

#[derive(pest_derive::Parser)]
#[grammar = "gardening.pest"]
pub struct GardenParser;

fn main() {
    let args = Args::parse();
    let cards_text = std::fs::read_to_string(args.puzzle_input).unwrap();

    let mut ps = GardenParser::parse(Rule::Garden, &cards_text).expect("bad parse");
    //eprintln!("pairz: {}", ps);


    let mut seeds = seeds_from(ps.next().unwrap().into_inner());
    // now I want to try to simplify the Seedsets... but how?

    seeds.sort_by_key(|f| f.start);

    //eprintln!("Seeds: {:#?}", seeds);

    let ranges: Vec<_> = seeds.iter().map(|s| Range { start: s.start, end: s.start + s.len}).collect();
    //eprintln!("Ranges: {:#?}", ranges);

    let seed_soil_map = GardenMap::from(ps.next().unwrap().into_inner());
    //eprintln!("Seed Soil Map: {:?}", seed_soil_map);

    let soil_fertilizer_map = GardenMap::from(ps.next().unwrap().into_inner());

    let fertilizer_water_map = GardenMap::from(ps.next().unwrap().into_inner());

    let water_light_map = GardenMap::from(ps.next().unwrap().into_inner());

    let light_temperature_map = GardenMap::from(ps.next().unwrap().into_inner());

    let temperature_humidity_map = GardenMap::from(ps.next().unwrap().into_inner());

    let humidity_location_map = GardenMap::from(ps.next().unwrap().into_inner());
    eprintln!("Humidity Location Map: {:#?}", humidity_location_map);
 
    let answer = ranges.into_iter().flatten()
        .map(|s| seed_soil_map.map(s))
        .map(|s| soil_fertilizer_map.map(s))
        .map(|s| fertilizer_water_map.map(s))
        .map(|s| water_light_map.map(s))
        .map(|s| light_temperature_map.map(s))
        .map(|s| temperature_humidity_map.map(s))
        .map(|s| humidity_location_map.map(s))
        .min()
        .unwrap();

    println!("Lowest location is {}", answer); // day05part1 answer was 175622908
    // day05part2 answer was 5200543 ; achieved via brute force and letting the laptop run.
}

fn seeds_from(ps: Pairs<Rule>) -> Vec<SeedSet> {
    //eprintln!("seeds:from({})", ps);
    ps.map(|sp| SeedSet::from(sp.into_inner())).collect()
}

#[derive(Debug)]
struct SeedSet {
    start: u32,
    len: u32
}

impl SeedSet {
    fn from(mut ps: Pairs<Rule>) -> Self {
        SeedSet {
            start: ps.next().unwrap().as_str().parse().unwrap(),
            len: ps.next().unwrap().as_str().parse().unwrap()
        }
    }
}


#[derive(Debug)]
struct GardenMap {
    //source: String, // these are for debugging only
    //destination: String,

    map: Vec<Triple>
}

impl GardenMap {
    fn map(&self, src: u32) -> u32 {
        // loop in our ranges, and if none match, then it goes straight through.
        match self.map.iter().find_map(|t| t.map(src)) {
            None => src,
            Some(x) => x
        }
    }

    fn from(ps: Pairs<Rule>) -> Self {
        //eprintln!("GardenMap::from({})", ps);
        GardenMap { map: ps.map(|t| Triple::from(t.into_inner())).collect() }
    }
}

#[derive(Debug)]
struct Triple {
    dst_start: u32,
    src_start: u32,
    len: u32,
}

impl Triple {
    fn map(&self, src: u32) -> Option<u32> {
        if src >= self.src_start && (src - self.src_start < self.len) {
            Some(src - self.src_start + self.dst_start)
        } else {
            None
        }
    }

    fn from(mut ps: Pairs<Rule>) -> Self {
        //eprintln!("Triple::from({})", ps);

        Triple {
            dst_start: ps.next().unwrap().as_str().parse().unwrap(),
            src_start: ps.next().unwrap().as_str().parse().unwrap(),
            len: ps.next().unwrap().as_str().parse().unwrap()
        }
    }
}