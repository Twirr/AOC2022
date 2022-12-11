use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

use aoc2022::Monkey;
use regex::Regex;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    println!("Running {}", program);

    let file = File::open("input_day11")?;
    let input = BufReader::new(file).lines().map(|line|line.unwrap()).collect::<Vec<String>>();

    //let mut monkey;
    let items_regex = Regex::new(r"  Starting items: ").unwrap();
    let items_regex2 = Regex::new(r"(?:, )?(\d+)+").unwrap();
    let mut items;

    for line in input{

        if items_regex.is_match(&line){
            items = items_regex2.captures_iter(&line).map(|c| {
                let a = str::parse::<i32>(&c[1]).unwrap();
                return a;
            }).collect::<Vec<i32>>();
        }
        //monkey = Monkey::new_monkey();
    }

    //part1(input.clone());
    //part2(input.clone());
    Ok(())
}
fn part1(input: Vec<(String, i32)>){

    //println!("Result1: {}", sum);
}
fn part2(input: Vec<(String, i32)>){

}
