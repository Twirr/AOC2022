use std::collections::{HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use lazy_static::lazy_static;
use priority_queue::PriorityQueue;

use regex::Regex;

lazy_static! {
    static ref NUMBER_REGEX:regex::Regex = Regex::new(r"(\d+)").unwrap();
}
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    println!("Running {}", program);

    let file = File::open("input_day12")?;
    let input = BufReader::new(file).lines().map(|line|{
        return line.unwrap().chars().collect::<Vec<char>>();
    }).collect::<Vec<Vec<char>>>();
   

    part1(input.clone());
    part2(input.clone());
    Ok(())
}

fn part1(input: Vec<Vec<char>>){
    let x_max = input.len();
    let y_max = input.get(0).unwrap().len();
    'bit_loop: for i in 0..x_max{
        for j in 0..y_max{
            if input.get(i).unwrap().get(j).unwrap() == &'S'{
                let shortest_path = find_shortest_path(input, (i,j));
                println!("Result1: {}",shortest_path);
                break 'bit_loop;
            }
        }
    }

}
fn part2(input: Vec<Vec<char>>){
    let x_max = input.len();
    let y_max = input.get(0).unwrap().len();
    let mut possible_starting_pos = Vec::new();
    for i in 0..x_max{
        for j in 0..y_max{
            if input.get(i).unwrap().get(j).unwrap() == &'a'{
                possible_starting_pos.push((i,j));
            }
        }
    }
    let mut shortest_path = u32::max_value();
    for start_pos in possible_starting_pos{
        let new_path = find_shortest_path(input.clone(), start_pos);
        if new_path < shortest_path{
            shortest_path = new_path;
        }
    }
    println!("Result2: {}",shortest_path);
}

fn get_unvisited_neighbors(cell: (usize,usize), visited: HashSet<(usize,usize)>, x_max: usize, y_max: usize) -> Vec<(usize,usize)>{
    let mut neighbors = Vec::new();
    let x = cell.0;
    let y = cell.1;
    if x > 0 && !visited.contains(&(x-1, y)){
        neighbors.push((x-1, y));
    }
    if x+1 < x_max && !visited.contains(&(x+1, y)){
        neighbors.push((x+1, y));
    }
    if y > 0 && !visited.contains(&(x, y-1)){
        neighbors.push((x, y-1));
    }
    if y+1 < y_max && !visited.contains(&(x, y+1)){
        neighbors.push((x, y+1));
    }
    
    return neighbors;
}

fn find_shortest_path(input: Vec<Vec<char>>, starting_pos: (usize,usize)) -> u32{
    let x_max = input.len();
    let y_max = input.get(0).unwrap().len();
    let mut visited: HashSet<(usize,usize)> = HashSet::new();
    let mut queue = PriorityQueue::new();

    queue.push(starting_pos, u32::max_value());

    while  !queue.is_empty() {
        let cell = queue.pop().unwrap();
        let x: usize = cell.0.0;
        let y: usize = cell.0.1;
        let distance = cell.1;
        let current_char = *input.get(x).unwrap().get(y).unwrap();
        let current_as_number;
        if current_char == 'S'{
            current_as_number = 'a' as u32;
        }else{
            current_as_number = current_char as u32;
        }


        visited.insert(cell.0);
        
        for neighbor in get_unvisited_neighbors(cell.0, visited.clone(),x_max,y_max){
            let neighbor_char = *input.get(neighbor.0).unwrap().get(neighbor.1).unwrap();
            let neighbor_as_number;
            if neighbor_char == 'E' {
                if current_char == 'y' || current_char == 'z'{
                    return u32::max_value()-distance +1;
                }
                neighbor_as_number = 'z' as u32;
            }else if neighbor_char == 'S'{
                neighbor_as_number = 'a' as u32;
            }else{
                neighbor_as_number = neighbor_char as u32;
            }
            if neighbor_as_number <= current_as_number+1{
                queue.push(neighbor, distance-1);
            }
        }
    }
    return u32::max_value();
}




