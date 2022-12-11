use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use lazy_static::lazy_static;

use aoc2022::{Monkey, Test, Operator, OtherOperator};
use regex::Regex;

lazy_static! {
    static ref NUMBER_REGEX:regex::Regex = Regex::new(r"(\d+)").unwrap();
}
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    println!("Running {}", program);

    let mut monkeys = HashMap::new();

    let file = File::open("input_day11")?;
    let input = BufReader::new(file).lines().map(|line|line.unwrap()).collect::<Vec<String>>();

    let items_regex = Regex::new(r"(?:, )?(\d+)+").unwrap();

    for i in (0..input.len()).step_by(7){
        let monkey_number = input.get(i).unwrap();
        let starting = input.get(i+1).unwrap();
        let items = items_regex.captures_iter(starting).map(|c| {
            let a = str::parse::<f64>(&c[1]).unwrap();
            return a;
        }).collect::<Vec<f64>>();
        let operation = input.get(i+2).unwrap();
        let operator = if operation.contains('+') { Operator::Add} else { Operator::Multi };
        let value;
        let other_operator;
        if NUMBER_REGEX.is_match(operation){
            value = get_any_number(operation) as f64;
            other_operator = OtherOperator::Value;
        }else{
            other_operator = OtherOperator::Me;
            value = 0.0;
        }
        let divide_by = input.get(i+3).unwrap();
        get_any_number(divide_by);
        let if_true = input.get(i+4).unwrap();
        let if_false = input.get(i+5).unwrap();
        monkeys.insert(get_any_number(monkey_number), Monkey{items: items,
            test: Test{divide_by: get_any_number(divide_by) as f64,if_true: get_any_number(if_true), if_false: get_any_number(if_false)}
            ,function: returns_closure(operator, other_operator), 
            value: value});
    }

   

    part1(monkeys.clone());
    part2(monkeys.clone());
    Ok(())
}

fn returns_closure(a: Operator, b: OtherOperator) -> fn(x: f64, y: f64) -> f64 {
    if a == Operator::Multi && b == OtherOperator::Me{
        return |x, _y| x * x;
    }else if a == Operator::Multi && b == OtherOperator::Value {
        return |x, y| x * y;
    }else if a == Operator::Add && b == OtherOperator::Value {
        return  |x, y| x + y;
    }else{
        return |x, _y| x + x;
    }
}
fn get_any_number(string: &String)-> i32{
    let a = NUMBER_REGEX.captures(string).unwrap();
    return a[1].parse().unwrap();
}


fn part1(mut input: HashMap<i32,Monkey>){
    let mut result = HashMap::new();
    for i  in 0..8{
        result.insert(i, 0);
    }   
    for _ in 0..20{
        for i in 0..8{
            let mut monkey = input.get(&i).unwrap().clone();
            let sum =result.get(&i).unwrap();
            result.insert(i, sum+monkey.items.len());

            for item in monkey.items.clone(){
                let worry_level = monkey.inspect(item);
                let target = monkey.get_target(worry_level);
                let mut target_monkey = input.get(&target).unwrap().clone();
                target_monkey.add_item(worry_level);
                input.insert(target, target_monkey);
            }
            monkey.clear_items();
            input.insert(i, monkey);
        }
    }
    let mut values = result.into_iter().map(|p| p.1).collect::<Vec<usize>>();
    values.sort_by(|a, b| b.cmp(a));
    

    println!("Result1: {}", values.get(0).unwrap()*values.get(1).unwrap());
}
fn part2(mut input: HashMap<i32,Monkey>){
    let mut result = HashMap::new();
    for i  in 0..8{
        result.insert(i, 0);
    }   
    for round in 0..10000{
        println!("Round: {}",round);
        for i in 0..8{
            let mut monkey = input.get(&i).unwrap().clone();
            let sum =result.get(&i).unwrap();
            result.insert(i, sum+monkey.items.len());

            for item in monkey.items.clone(){
                let worry_level = monkey.inspect2(item);
                let target = monkey.get_target(worry_level);
                let mut target_monkey = input.get(&target).unwrap().clone();
                target_monkey.add_item(worry_level);
                input.insert(target, target_monkey);
            }
            monkey.clear_items();
            input.insert(i, monkey);
        }
    }
    let mut values = result.into_iter().map(|p| p.1).collect::<Vec<usize>>();
    values.sort_by(|a, b| b.cmp(a));
    

    println!("Result2: {}", values.get(0).unwrap()*values.get(1).unwrap());
}