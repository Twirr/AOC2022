use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    println!("Running {}", program);

    let file = File::open("input_day5_starting_state")?;
    let starting_state = BufReader::new(file).lines().map(|line|{
        return line.unwrap().chars().collect::<Vec<char>>();
    }).collect::<Vec<Vec<char>>>();

    let mut stacks:HashMap<String,Vec<char>> = HashMap::new();
    let columns = starting_state.get(0).unwrap().len();
    let rows = starting_state.len();
    for col in 0..columns{
        let mut current_stack = Vec::new();
        let mut stack_index= ' ';
        for row in (0..rows).rev(){
            let c = *starting_state.get(row).unwrap().get(col).unwrap();
            if c == ' '{
                break;
            }
            if row == rows-1{
                stack_index = c;
            }else{
                current_stack.push(c)
            }
        }
        stacks.insert(stack_index.to_string(), current_stack);
    }
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let file = File::open("input_day5_operation")?;
    let operations = BufReader::new(file).lines().map(|line|{
        let str_line = line.unwrap();
        let cap = re.captures(&str_line).unwrap();
        let from_key = cap[2].to_string();
        let to_key = cap[3].to_string();
        let steps = str::parse::<u32>(&cap[1]).unwrap();
        return(from_key,to_key,steps);
    }).collect::<Vec<(String,String,u32)>>();
    
    part1(stacks.clone(), operations.clone());

    part2(stacks.clone(),operations.clone());
    Ok(())
}

fn part1(mut stacks:HashMap<String,Vec<char>>, operations:Vec<(String,String,u32)>){
    for operation in operations{
        let mut from = stacks.get(&operation.0).unwrap().clone();
        let mut to = stacks.get(&operation.1).unwrap().clone();
        for _ in 0..operation.2{
            to.push(from.pop().unwrap());
        }
        stacks.insert(operation.0, from);
        stacks.insert(operation.1, to);
    }

    
    print!("Result1: ");
    for i in 1..10{
        let a = stacks.get(&i.to_string()).unwrap().clone().pop().unwrap();
        print!("{}",a)
    }
    println!("");
}

fn part2(mut stacks:HashMap<String,Vec<char>>, operations:Vec<(String,String,u32)>){

    for operation in operations{
        let mut from = stacks.get(&operation.0).unwrap().clone();
        let mut to = stacks.get(&operation.1).unwrap().clone();

        let mut stack = Vec::new();
        for _ in 0..operation.2{
            stack.push(from.pop().unwrap());
        }
        for _ in 0..operation.2{
            to.push(stack.pop().unwrap());
        }

        stacks.insert(operation.0, from);
        stacks.insert(operation.1, to);
    }

    
    print!("Result2: ");
    for i in 1..10{
        let a = stacks.get(&i.to_string()).unwrap().clone().pop().unwrap();
        print!("{}",a)
    }
    println!("");
}

