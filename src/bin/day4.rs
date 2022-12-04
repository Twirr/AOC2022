use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

use aoc2022::Interval;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    println!("Running {}", program);

    let file = File::open("input_day4")?;
    let original_data = BufReader::new(file).lines().map(|line|{
        let unwrapped = line.unwrap();
        let mut split = unwrapped.split(',').into_iter();
        let left = split.next().unwrap();
        let right = split.next().unwrap();
        return (Interval::from_string(left),Interval::from_string(right));
    }).collect::<Vec<(Interval,Interval)>>();
    

    part1(original_data.clone());

    part2(original_data.clone());
    Ok(())
}
fn part1(intervals: Vec<(Interval,Interval)>){
    let mut sum = 0;
    for pair in intervals{
        if pair.0.overlap_comletely(pair.1){
            sum += 1;
        }
    }

    println!("Result1: {}", sum);
}

fn part2(intervals:  Vec<(Interval,Interval)>){
    let mut sum = 0;
    for pair in intervals{
        if pair.0.overlap(pair.1){
            sum += 1;
        }
    }

    println!("Result2: {}", sum);
}

