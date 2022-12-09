use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    println!("Running {}", program);

    let file = File::open("input_day8")?;
    let input = BufReader::new(file).lines().map(|line|{
        return line.unwrap().chars().map(|c| c as i32).collect::<Vec<i32>>();
    }).collect::<Vec<Vec<i32>>>();

    part1(input.clone());

    part2(input.clone());

    Ok(())
}

fn part1(input: Vec<Vec<i32>>){
    let mut result = Vec::new();
    let rows = input.len();
    let columns = input.get(0).unwrap().len();
    
    for i in 0..rows{
        let row_size = input.get(i).unwrap().len();
        let mut row = Vec::with_capacity(row_size);
        for _ in 0..row_size{
            row.push(false);
        }
        result.push(row);
    }

   

    //From LEFT
    for i in 0..rows{
        let mut prev_height = i32::min_value();
        let mut current_height;
        for j in 0..columns{
            current_height = *input.get(i).unwrap().get(j).unwrap();
            if current_height > prev_height{
                let t = result.get_mut(i).unwrap().get_mut(j).unwrap();
                *t = true;
                prev_height = current_height;
            }
        }
    }
     //From RIGHT
     for i in 0..rows{
        let mut prev_height = i32::min_value();
        let mut current_height;
        for j in (0..columns).rev(){
            current_height = *input.get(i).unwrap().get(j).unwrap();
            if current_height > prev_height{
                let t = result.get_mut(i).unwrap().get_mut(j).unwrap();
                *t = true;
                prev_height = current_height;
            }
        }
    }

    //From TOP
    for j in 0..columns{
        let mut prev_height = i32::min_value();
        let mut current_height;
        for i in 0..rows{
            current_height = *input.get(i).unwrap().get(j).unwrap();
            if current_height > prev_height{
                let t = result.get_mut(i).unwrap().get_mut(j).unwrap();
                *t = true;
                prev_height = current_height;
            }
        }
    }

    //From BOTTOM
    for j in 0..columns{
        let mut prev_height = i32::min_value();
        let mut current_height;
        for i in (0..rows).rev(){
            current_height = *input.get(i).unwrap().get(j).unwrap();
            if current_height > prev_height{
                let t = result.get_mut(i).unwrap().get_mut(j).unwrap();
                *t = true;
                prev_height = current_height;
            }
        }
    }

    let mut sum = 0;
    for i in 0..rows{
        let a = result.get(i).unwrap();
        for j in 0..columns{

            let res = a.get(j).unwrap();
            if *res{
                sum +=1;
            }
        }
    }
    println!("Result1: {}", sum);
}

fn part2(input: Vec<Vec<i32>>){
    let mut best_view = 0;
    let rows = input.len();
    let columns = input.get(0).unwrap().len();

    for i in 0..rows{
        for j in 0..columns{
            let our_height = *input.get(i).unwrap().get(j).unwrap();
            
            let mut visibility_south = 0;
            for k in i+1..rows{
                visibility_south+=1;
                let height = *input.get(k).unwrap().get(j).unwrap();
                if height >= our_height{
                    break;
                }
            }

            let mut visibility_north = 0;
            for k in (0..i).rev(){
                visibility_north+=1;
                let height = *input.get(k).unwrap().get(j).unwrap();
                if height >= our_height{
                    break;
                }
            }

            let mut visibility_east = 0;
            for l in j+1..columns{
                visibility_east+=1;
                let height = *input.get(i).unwrap().get(l).unwrap();
                if height >= our_height{
                    break;
                }
            }

            let mut visibility_west = 0;
            for l in (0..j).rev(){
                visibility_west+=1;
                let height = *input.get(i).unwrap().get(l).unwrap();
                if height >= our_height{
                    break;
                }
            }
            let view = visibility_south*visibility_north*visibility_west*visibility_east;
            if view > best_view{
                best_view = view;
            }
        }
    }
    println!("Result2: {}", best_view);
}