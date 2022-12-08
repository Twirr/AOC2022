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
    let mut header_found = false;
    BufReader::new(file).lines().for_each(|line|{
        let all_the_chars = line.unwrap();
        for i in 0..all_the_chars.len(){
            if !header_found && validate_header(&all_the_chars[i..i+4]){
                println!("Result1: {}",i+4);
                header_found = true;
            }

            if validate_message(&all_the_chars[i..i+14]){
                println!("Result2: {}",i+14);
                break;
            }
        }
    });
    Ok(())
}

fn validate_header(maybee_key: &str ) -> bool{
    let set = maybee_key.chars().collect::<HashSet<char>>();
    return set.len() == 4;
}
fn validate_message(maybee_key: &str ) -> bool{
    let set = maybee_key.chars().collect::<HashSet<char>>();
    return set.len() == 14;
}

