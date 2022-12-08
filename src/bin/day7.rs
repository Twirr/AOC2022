use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::{ HashSet, HashMap};
use aoc2022::Folder;
use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    println!("Running {}", program);

    let file = File::open("input_day7")?;
    let input = BufReader::new(file).lines().map(|line|{
       return line.unwrap();
    }).collect::<Vec<String>>();

    

    solve(input.clone());

    Ok(())
}

fn solve(input: Vec<String>){
    let mut sum = 0;
    let mut folder_closests_to_target = 0;
    let mut folder_closests_to_target_diff = i64::max_value();
    let target:i64 = 8748071;
    let file_pattern = Regex::new(r"(\d+) [a-z.]+").unwrap();
    let folder_pattern = Regex::new(r"\$ cd ([a-z])+").unwrap();
    let mut current_folder = Folder::create_folder();
    let mut folder_stack = Vec::new();
    for row in input{
        if file_pattern.is_match(&row){
            let file_size = file_pattern.captures(&row).unwrap()[1].parse().unwrap();
            current_folder.add_file(file_size)
        }

        if row.contains("cd .."){
            //We leave a folder, if we have size > 100000 add to sum, fetch our parent
            let size = current_folder.get_size();
            
            if size < 100000{
                sum +=size;
            }
            let diff_from_target = size-target;
            if diff_from_target >= 0 && diff_from_target < folder_closests_to_target_diff{
                folder_closests_to_target = size;
                folder_closests_to_target_diff = diff_from_target;
            }
            let mut parent: Folder = folder_stack.pop().unwrap();
            parent.add_file(size);
            current_folder = parent;
        }else if folder_pattern.is_match(&row){
            folder_stack.push(current_folder);

            let new_folder = Folder::create_folder();

            current_folder = new_folder;
        }
    }
    while !folder_stack.is_empty(){
        let size = current_folder.get_size();
       
        if size < 100000{
            sum +=size;
        }
        let diff_from_target = size-target;
        if diff_from_target >= 0 && diff_from_target < folder_closests_to_target_diff{
            folder_closests_to_target = size;
            folder_closests_to_target_diff = diff_from_target;
        }
        let mut parent: Folder = folder_stack.pop().unwrap();
        parent.add_file(size);
        current_folder = parent;
    }
    println!("Max size \t{}",current_folder.get_size());
    let available_size = 70000000-current_folder.get_size();
    println!("Available size\t{}",available_size);
    println!("Missing space\t {}",30000000-available_size);
    println!("Result1: \t {}", sum);
    println!("Result2: \t {}", folder_closests_to_target);
}


