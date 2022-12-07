use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::{ HashSet};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    println!("Running {}", program);

    let file = File::open("input_day6")?;
    let mut headerFound = false;
    BufReader::new(file).lines().for_each(|line|{
        let all_the_chars = line.unwrap();
        for i in 0..all_the_chars.len(){
            if !headerFound && validateHeader(&all_the_chars[i..i+4]){
                println!("Result1: {}",i+4);
                headerFound = true;
            }

            if validateMessage(&all_the_chars[i..i+14]){
                println!("Result2: {}",i+14);
                break;
            }
        }
    });

    part1();

    part2();
    Ok(())
}

fn validateHeader(maybeeKey: &str ) -> bool{
    let set = maybeeKey.chars().collect::<HashSet<char>>();
    return set.len() == 4;
}
fn validateMessage(maybeeKey: &str ) -> bool{
    let set = maybeeKey.chars().collect::<HashSet<char>>();
    return set.len() == 14;
}

