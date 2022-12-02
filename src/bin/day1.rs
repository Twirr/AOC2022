use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    println!("Running {}", program);

    let file = File::open("input_day1")?;
    let original_data = BufReader::new(file).lines().map(|line| line.unwrap()).collect::<Vec<String>>();

    part1(original_data.clone());

    part2(original_data.clone());
    Ok(())
}
fn part1(original_data: Vec<String>){
    let mut largest_sum = 0;
    let mut current_sum = 0;

    for line in original_data {
        if line.is_empty() {
            if current_sum > largest_sum {
                largest_sum = current_sum;
            }
            current_sum = 0;
        }else {
            current_sum += line.parse::<i32>().unwrap();
        }
    }


    println!("Result1: {}", largest_sum);
}

fn part2(original_data: Vec<String>){
    let mut current_sum = 0;
    let mut elfs = Vec::new();

    for line in original_data {
        if line.is_empty() {
            elfs.push(current_sum);
            current_sum = 0;
        }else {
            current_sum += line.parse::<i32>().unwrap();
        }
    }
    elfs.sort_by(|a,b| b.cmp(a));
    let sum:i32 = elfs[0..3].iter().sum();


    println!("Result2: {}", sum);
}

