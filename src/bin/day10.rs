use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    println!("Running {}", program);

    let file = File::open("input_day10")?;
    let input = BufReader::new(file).lines().map(|line|{
        let string = line.unwrap();
        let mut split = string.split(' ').into_iter();
        let operation = split.next().unwrap().to_string();
        let mut value = 0;
        if operation == "addx"{
            value = split.next().unwrap().parse().unwrap();
        }
        return (operation, value)
    }).collect::<Vec<(String, i32)>>();

    part1(input.clone());

    part2(input.clone());
    Ok(())
}
fn part1(input: Vec<(String, i32)>){
    let mut current_cycle = 0;
    let mut x = 1;
    let mut signal_strs = Vec::new();
    for op in input{
        let cycle_steps;
        match op.0.as_str() {
            "addx" => cycle_steps=2,
            "noop" => cycle_steps=1,
            _ => panic!()
        }
        for _ in 0..cycle_steps{
            current_cycle+=1;
            if (current_cycle-20) % 40 == 0{
                let sig_str = x*current_cycle;
                signal_strs.push(sig_str);
            }
        }
        x+=op.1;
    }
    let sum: i32 = signal_strs[0..6].into_iter().sum();

    println!("Result1: {}", sum);
}
fn part2(input: Vec<(String, i32)>){
    let mut current_cycle = 0;
    let mut x = 1;
    let mut result = Vec::new();
    let mut row = Vec::new();

    for op in input{
        let cycle_steps;
        match op.0.as_str() {
            "addx" => cycle_steps=2,
            "noop" => cycle_steps=1,
            _ => panic!()
        }
        for _ in 0..cycle_steps{
            let column = current_cycle % 40;
            if current_cycle != 0 && column == 0{
                result.push(row);
                row = Vec::new();
            }
            if column <= x+1 && column >= x-1{
                row.push('#');
            }else{
                row.push('.');
            }
            current_cycle+=1;
        }
        x+=op.1;
    }
    result.push(row);

   
    for r in result{
        for c in r{
            print!("{}",c);    
        }
        println!("")
    }
}
