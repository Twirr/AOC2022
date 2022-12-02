use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    println!("Running {}", program);

    let file = File::open("input_day2")?;
    let original_data = BufReader::new(file).lines().map(|line| line.unwrap()).collect::<Vec<String>>();

    part1(original_data.clone());

    part2(original_data.clone());
    Ok(())
}
fn part1(original_data: Vec<String>){
    let mut sum = 0;

    for line in original_data {
        let points = match line.as_str(){
            "A X" => 3+1,
            "A Y" => 6+2,
            "A Z" => 0+3,
            "B X" => 0+1,
            "B Y" => 3+2,
            "B Z" => 6+3,
            "C X" => 6+1,
            "C Y" => 0+2,
            "C Z" => 3+3,
            _ =>{
                panic!();
            }
        };
        sum+=points;
    }


    println!("Result1: {}", sum);
}

fn part2(original_data: Vec<String>){
    let mut sum = 0;

    for line in original_data {
        let points = match line.as_str(){
            "A X" => 0+3,
            "A Y" => 3+1,
            "A Z" => 6+2,
            "B X" => 0+1,
            "B Y" => 3+2,
            "B Z" => 6+3,
            "C X" => 0+2,
            "C Y" => 3+3,
            "C Z" => 6+1,
            _ =>{
                panic!();
            }
        };
        sum+=points;
    }
    println!("Result2: {}", sum);
}

