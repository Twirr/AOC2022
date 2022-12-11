use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    println!("Running {}", program);

    let file = File::open("input_day9")?;
    let input = BufReader::new(file).lines().map(|line|{
        let string = line.unwrap();
        let mut split = string.split(' ').into_iter();
        let operation = split.next().unwrap().to_string();
        let direction = split.next().unwrap().parse().unwrap();
        return (operation, direction)
    }).collect::<Vec<(String, i32)>>();

    part1(input.clone());

    part2(input.clone());
    Ok(())
}
fn part1(input: Vec<(String, i32)>){
    let mut head_pos = (0,0);
    let mut tail_pos = (0,0);
    let mut visited_cells: HashSet<(i32,i32)> = HashSet::new();

    for op in input{
        let tmp = op.0;
        for _ in 0..op.1{
            head_pos = move_head((tmp.clone(), 1), head_pos);
            tail_pos = move_tail(head_pos, tail_pos);
    
            //add visited
            visited_cells.insert(tail_pos);
        }
    }
    println!("Result1: {}", visited_cells.len());
}
fn part2(input: Vec<(String, i32)>){
    let mut head_pos = (0,0);
    let mut tail_pos = Vec::new();
    for _ in 0..9{
        tail_pos.push((0,0));
    }
    let mut visited_cells: HashSet<(i32,i32)> = HashSet::new();

    for op in input{
        let tmp = op.0;
        for _ in 0..op.1{
            let mut new_tail = Vec::new();
            head_pos = move_head((tmp.clone(), 1), head_pos);
            new_tail.push(move_tail(head_pos, *tail_pos.get(0).unwrap()));
            for i in 1..9{
                new_tail.push(move_tail(*new_tail.get(i-1).unwrap(), *tail_pos.get(i).unwrap()));
            }
    
            //add visited
            visited_cells.insert(*new_tail.last().unwrap());
            tail_pos = new_tail;

        }
    }
    println!("Result2: {}", visited_cells.len())
}
fn move_head(op: (String, i32), current_pos: (i32,i32)) -> (i32,i32){
    match op.0.as_str(){
        "U" => return (current_pos.0, current_pos.1+op.1),
        "D" => return (current_pos.0, current_pos.1-op.1),
        "L" => return (current_pos.0-op.1, current_pos.1),
        "R" => return (current_pos.0+op.1, current_pos.1),
        _ => panic!()
    }

}
fn move_tail(head_pos: (i32,i32), tail_pos: (i32,i32)) -> (i32,i32){
    let delta = (head_pos.0-tail_pos.0, head_pos.1-tail_pos.1);
    let move_right = delta.0 > 1;
    let move_up = delta.1 > 1;
    let move_left = delta.0 < -1;
    let move_down = delta.1 < -1;
    if !(move_right || move_up || move_left || move_down){
        return tail_pos;
    }

    let mut new_pos_x = head_pos.0;
    let mut new_pos_y = head_pos.1;
    if move_up{
        new_pos_y = head_pos.1-1;
    }
    if move_down{
        new_pos_y = head_pos.1+1;
    } 
    if move_left{
        new_pos_x = head_pos.0+1;
    }
    if move_right{
        new_pos_x = head_pos.0-1;
    }
    return (new_pos_x,new_pos_y);
}