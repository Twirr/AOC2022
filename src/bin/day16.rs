use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::time::Instant;
use lazy_static::lazy_static;

use priority_queue::PriorityQueue;

use regex::Regex;

lazy_static! {
    static ref NUMBER_REGEX:regex::Regex = Regex::new(r"(\d+)").unwrap();
    static ref ONE_REGEX:regex::Regex = Regex::new(r"1").unwrap();
    static ref VALVE_REGEX:regex::Regex = Regex::new(r"([A-Z][A-Z])").unwrap();
}
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    println!("Running {}", program);
    let start = Instant::now();
    
    let file = File::open("input_day16")?;
    let mut input = HashMap::new();
    BufReader::new(file).lines().for_each(|line|{
        let s = line.unwrap();
        let flow = get_any_number(&s);
        let valves = get_valves(&s);
        input.insert(valves.0, (valves.1,flow));
    });

    
    let valves = input.clone().into_iter().filter(|a| a.1.1 > 0 || a.0 == "AA").map(|a| a.0).collect::<Vec<String>>();
    
    
    let part1_input = reduce(input.clone(), valves.clone());
    let part1 = find_max_flow(part1_input.clone(),30);

    println!("Result1: {}",part1);

    let valves_wo_aa = valves.clone().into_iter().filter(|a| a != "AA").collect::<Vec<String>>();
    let combinations = combine(valves_wo_aa.len()/2, valves_wo_aa);
    let mut best_combination = 0;
    println!("{} combination to test", combinations.len());
    let mut tested_combinations = 0;
    for combination in combinations{
        let mut a_vec = combination.0;
        a_vec.push("AA".to_string());

        let mut input_a: HashMap<String,(Vec<(String,u32)>,u64)> = HashMap::new();
        part1_input.clone().into_iter().filter(|v| a_vec.contains(&v.0)).for_each(|v| {
            let from = v.0;
            let to = v.1.0.into_iter().filter(|d| a_vec.contains(&d.0)).collect();
            let flow = v.1.1;
            input_a.insert(from, (to,flow));
        });
        let a = find_max_flow(input_a,26);
        let mut b_vec = combination.1;
        b_vec.push("AA".to_string());

        let mut input_b: HashMap<String,(Vec<(String,u32)>,u64)> = HashMap::new();
        part1_input.clone().into_iter().filter(|v| b_vec.contains(&v.0)).for_each(|v| {
            let from = v.0;
            let to = v.1.0.into_iter().filter(|d| b_vec.contains(&d.0)).collect();
            let flow = v.1.1;
            input_b.insert(from, (to,flow));
        });
        let b = find_max_flow(input_b,26);
        if a+b > best_combination{
            best_combination = a+b;
        }
        //println!("Testing combinations {:?} and {:?} with result {}", a_vec, b_vec, a+b);
        tested_combinations +=1;
        if tested_combinations % 10 == 0{
            println!("Tested {} combinations", tested_combinations);
        }
    }
    println!("Result2: {}",best_combination);

    


    //part2(new_input.clone());
    
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    Ok(())
}
fn reduce(input: HashMap<String,(Vec<String>,u64)>, valves: Vec<String> ) -> HashMap<String, (Vec<(String, u32)>, u64)>{
    let mut new_input = HashMap::new();
    for valve in input.clone(){
        let mut valve_paths: Vec<(String,u32)> = Vec::new();
        //Find closest path to each valve
        for other_valve in valves.clone(){
            if valve.0 == other_valve{
                continue;
            }
            let path = find_shortest_path(valve.0.clone(),other_valve.clone(), input.clone());
            valve_paths.push((other_valve,path));
        }
        //println!("{:?}",valve_paths.clone());
        new_input.insert(valve.0, (valve_paths,valve.1.1));
    }
    return new_input;
}

fn combine(length: usize, valves: Vec<String>) -> Vec<(Vec<String>, Vec<String>)>{
    let mut combinations = Vec::new();
    let valves_size = valves.len();
    let base: i32 = 2;
    for i in 1..base.pow(valves_size as u32){
        let binary_str = format!("{i:b}");
        let matches = ONE_REGEX.captures_iter(&binary_str).count();
        if matches == length{
            let mut first_set = Vec::new();
            let mut second_set = Vec::new();
            for j in 0..valves_size{
                let include = binary_str.chars().nth(j);
                if include.is_some() && include.unwrap() == '1'{
                    first_set.push(valves.get(j).unwrap().clone());
                }else{
                    second_set.push(valves.get(j).unwrap().clone());
                }
            }
            combinations.push((first_set,second_set));
        }
    }
    return combinations;
}


fn find_max_flow(input: HashMap<String,(Vec<(String,u32)>,u64)>, start_time: u64) -> u64{
    /*for a in input.clone(){
        println!("{}  -> {:?}",a.0,a.1);
    }*/
    let mut cache: HashMap<(String,Vec<String>,u64), u64> = HashMap::new();
    let mut queue = PriorityQueue::new();
    let mut terminated_routes: u128 = 0;
    let number_of_valves = input.clone().len();
    let mut best_result = 0;
    
    queue.push(("AA".to_string(),0,0, Vec::new()),0);
    while !queue.is_empty(){
        //let valve = input.get(cell).unwrap();
        let tmp = queue.pop().unwrap();
        let valve = tmp.0;
        let cell = valve.0;
        let time = valve.1;
        let current_flow = valve.2;
        let mut visited = valve.3.clone();
        let key = (cell.to_string(), visited.clone(),time);
        //println!("Visiting {} at time {}",cell,time);

        
        if time >= start_time || visited.len() == number_of_valves{
            if current_flow > best_result {
                best_result = current_flow;;
            }
            terminated_routes +=1;
            if terminated_routes % 100000 == 0{
                println!("Terminated {} routes", terminated_routes);
            }
            continue;
        }
        if !visited.contains(&cell){
            if cache.contains_key(&key){
                continue;
            }
            let input_valve = input.get(&cell).unwrap();

            visited.push(cell.to_string());
            visited.sort();
            let new_flow = current_flow + (start_time-time) * input_valve.1;
            for s in input_valve.0.clone(){
                queue.push((s.0 ,time+1+s.1 as u64, new_flow, visited.clone()), new_flow);
            }
        }

        let mut tmp_time = time;
        while tmp_time < start_time{
            //Push all worse times in same state to cache
            let tmp_key = (cell.to_string(),visited.clone(),time);
            let s = cache.get(&tmp_key);
            if s.is_some() && *s.unwrap() > current_flow{
                break;
            }else{
                cache.insert(tmp_key, current_flow);
            }
            tmp_time+=1;
        }

    }

    return best_result;
}

fn part2(input: HashMap<String,(Vec<(String,u32)>,u64)>){
    let mut cache: HashMap<(String,Vec<String>,u64,String), u64> = HashMap::new();
    let mut best_result = 0;
    let mut queue = PriorityQueue::new();
    let mut terminated_routes: u128 = 0;
    let number_of_valves = input.clone().len();
    
    queue.push(("AA".to_string(),0,0,Vec::new(),"AA".to_string(),0),0);
    while !queue.is_empty(){
        //let valve = input.get(cell).unwrap();
        let tmp = queue.pop().unwrap();
        let valve = tmp.0;
        let cell = valve.0;
        let elephant_cell = valve.4;
        let time = valve.1;
        let elephant_time = valve.5;
        let current_flow = valve.2;
        let mut visited = valve.3.clone();
        //println!("Visiting {} at time {} and ele_time {}",cell,time,elephant_time);
        //println!("Visited {:?}",visited);
        if visited.len() == number_of_valves{
            if current_flow > best_result{
                best_result = current_flow;
            }
            terminated_routes +=1;
            if terminated_routes % 100000 == 0{
                println!("Terminated {} routes", terminated_routes);
            }
            continue;
        }
        if !visited.contains(&cell) && !visited.contains(&elephant_cell) && (cell != elephant_cell || cell == "AA"){
            let key = (cell.to_string(), visited.clone(),time,elephant_cell.to_string());
            if cache.contains_key(&key){
                continue;
            }
            let input_valve = input.get(&cell).unwrap();
            let elephant_valve = input.get(&elephant_cell).unwrap();

            visited.push(cell.to_string());
            if cell != "AA"{
                visited.push(elephant_cell.to_string());
            }
            visited.sort();

            let new_ele_flow;
            if elephant_time > 26{
                new_ele_flow = 0;
            }else{
                new_ele_flow =  (26-elephant_time) * elephant_valve.1;
            }

            let new_me_flow;
            if time > 26{
                new_me_flow = 0;
            }else{
                new_me_flow =  (26-time) * input_valve.1;
            }
            let new_flow = current_flow + new_ele_flow + new_me_flow;

            for s in input_valve.0.clone(){
                for a in elephant_valve.0.clone(){
                    queue.push((s.0.clone() ,time+1+s.1 as u64, new_flow, visited.clone(),a.0, elephant_time+1+a.1 as u64), new_flow);
                }
            }
        }

        let mut tmp_time = time;
        while tmp_time < 26{
            //Push all worse times in same state to cache
            let tmp_key = (cell.to_string(),visited.clone(),time,elephant_cell.to_string());
            let s = cache.get(&tmp_key);
            if s.is_some() && *s.unwrap() > current_flow{
                break;
            }else{
                cache.insert(tmp_key, current_flow);
            }
            tmp_time+=1;
        }

    }
    let mut result = cache.into_iter().map(|a| a.1).collect::<Vec<u64>>();
    result.sort();
    
    println!("Result2: {}",result.last().unwrap());
}

fn get_any_number(string: &String)-> u64{
    let a = NUMBER_REGEX.captures(string).unwrap();
    return a[1].parse().unwrap();
}

fn get_valves(string: &String)-> (String,Vec<String>){
    let mut items = VALVE_REGEX.captures_iter(string).map(|c| {
        let a = c[1].to_string();
        return a;
    }).collect::<Vec<String>>();
    
    return (items.remove(0), items);
}

fn find_shortest_path(from: String, to: String, input: HashMap<String,(Vec<String>,u64)>) -> u32{
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue = PriorityQueue::new();

    queue.push(from, u32::max_value());

    while  !queue.is_empty() {
        let cell = queue.pop().unwrap();
        let distance = cell.1;

        visited.insert(cell.0.clone());
        
        for neighbor in get_unvisited_neighbors(cell.0.clone(), visited.clone(), input.clone()){
            if neighbor == to{
                return u32::max_value()-distance +1;
            }
            queue.push(neighbor, distance-1);
        }
    }
    return u32::max_value();
}

fn get_unvisited_neighbors(cell: String, visited: HashSet<String>, input: HashMap<String,(Vec<String>,u64)>) -> Vec<String>{
     let a = &input.get(&cell).unwrap().0;
     let mut b = Vec::new();
     for s in a{
        if !visited.contains(s){
            b.push(s.to_string());
        }
     }
     return b;
}

