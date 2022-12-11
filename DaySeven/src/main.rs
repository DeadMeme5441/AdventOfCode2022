use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut command_list: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                command_list.push(ip);
            }
        }
    }
    parse_commands(command_list);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_commands(command_list: Vec<String>) -> bool {
    let mut directories: HashMap<String, usize> = HashMap::new();
    let mut current_path: Vec<&str> = Vec::new();

    for command in command_list.iter() {
        let split: Vec<&str> = command.split(" ").collect::<Vec<&str>>();
        if split[0] == "$" {
            if split[1] == "cd" {
                if split[2] != ".." {
                    current_path.push(split[2]);
                    let current_str: String = current_path.concat();
                    directories.insert(current_str, 0);
                    println!("{:?}", current_path);
                } else if split[2] == ".." {
                    current_path.pop();
                }
            } else if split[1] == "ls" {
                continue;
            }
        } else {
            if split[0] == "dir" {
                continue;
            } else {
                let val: usize = split[0].parse::<usize>().unwrap();
                let mut current_str: String = String::from("");
                for dir in current_path.iter() {
                    current_str += dir;
                    *directories.get_mut(&current_str.to_string()).unwrap() += val;
                }
            }
        }
    }

    let mut total_val: u32 = 0;

    // for (dir, size) in directories.into_iter() {
    //     count2 += 1;
    //     if size <= 100000 {
    //         total_val += size as u32;
    //     }
    // }
    // println!("{}", count2);

    // println!("{}", total_val);

    part_two(directories);

    false
}

fn part_two(directories: HashMap<String, usize>) -> usize {
    let final_size: usize = 0;

    let total_size: usize = *directories.get(&"/".to_string()).unwrap();

    let available_size: usize = 70000000 - total_size;

    println!("Total : {} , Available : {}", total_size, available_size);

    let mut lowest_value: usize = total_size;
    for (dir, size) in directories.into_iter() {
        if size > (30000000 - available_size) {
            if size < lowest_value {
                lowest_value = size;
            }
        }
    }
    println!("{}", lowest_value);
    final_size
}
