use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::str::Chars;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    println!("Running {}", program);

    let file = File::open("input_day3")?;
    let original_data = BufReader::new(file).lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    

    part1(original_data.clone());

    part2(original_data.clone());
    Ok(())
}
fn part1(original_data: Vec<String>){
    let pairs = original_data.into_iter().map(|line|{ 
            let pointer_pair = line.split_at(line.len()/2);
            return (String::from(pointer_pair.0),String::from(pointer_pair.1));
        }).collect::<Vec<(String,String)>>();

    let mut sum = 0;
    for pair in pairs{
        
        let set: HashSet<_> = pair.0.chars().into_iter().collect();
        let union = intersection(set, pair.1.chars());
        sum+=to_prio(union);
    }
    println!("Result1: {}", sum);
}

fn part2(original_data: Vec<String>){
    let mut groups = Vec::new();
    for i in (0..original_data.len()).step_by(3){
        let a = original_data.get(i);
        let b = original_data.get(i+1);
        let c = original_data.get(i+2);
        groups.push((a.unwrap(),b.unwrap(),c.unwrap()))
    }
    let mut sum = 0;

    for group in groups{
        let set: HashSet<_> = group.0.chars().into_iter().collect();
        let first_union: HashSet<char> = intersection(set, group.1.chars());
        let second_union: HashSet<char> = intersection(first_union, group.2.chars());
        
        sum+=to_prio(second_union);
    }

    println!("Result2: {}", sum);
}

fn intersection(set: HashSet<char>, chars: Chars<'_>) -> HashSet<char>{
    let mut union: HashSet<char> = HashSet::new();
    chars.for_each(|c | {
        if set.contains(&c) {
            union.insert(c);
        }
    });
    return union;
}

fn to_prio(union: HashSet<char>) -> u32{
    let mut sum = 0;
    union.into_iter().for_each(|c| {
        let ascii = c as u32;
        if ascii > 95 { // a = 97 -> a = 1
            sum+= ascii-96;
        }else{ // A = 65 -> A = 27
            sum+= ascii-38;
        }
    });
    return sum
}

