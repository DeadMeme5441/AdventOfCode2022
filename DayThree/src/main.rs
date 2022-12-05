use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let priority_string: String =
        String::from("0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");

    let mut total_priority: usize = 0;

    let mut rucksacks_one: Vec<(String, String)> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let length: usize = ip.len();
                let (split1, split2) = ip.split_at(length / 2);
                if split1.len() != split2.len() {
                    println!("{}, {}", split1, split2);
                }
                rucksacks_one.push((split1.to_owned(), split2.to_owned()));
            }
        }
    }

    println!("{}", priority_part_one(rucksacks_one));

    let mut rucksacktwo: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        let mut temp_str: String = String::new();
        let mut count = 0;
        for line in lines {
            if let Ok(ip) = line {
                count += 1;
                if count % 3 != 0 {
                    temp_str += &ip.to_string();
                    temp_str += ",";
                } else {
                    temp_str += &ip.to_string();
                    rucksacktwo.push(temp_str);
                    temp_str = "".to_string();
                }
            }
        }
    }

    println!("{:?}", priority_part_two(rucksacktwo));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn priority_part_one(rucksacks: Vec<(String, String)>) -> usize {
    let priority_string: String =
        String::from("0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");

    let mut total_priority: usize = 0;

    for rucksack in rucksacks {
        if rucksack.0.len() != rucksack.1.len() {
            println!("{}, {}", rucksack.0, rucksack.1);
        }

        for letter in rucksack.0.chars().collect::<Vec<char>>() {
            if rucksack.1.contains(letter) {
                total_priority += priority_string.find(letter).unwrap();
                break;
            }
        }
    }

    total_priority
}

fn priority_part_two(rucksacktwo: Vec<String>) -> usize {
    let priority_string: String =
        String::from("0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");

    let mut total_prio: usize = 0;
    for group in rucksacktwo {
        let temp: Vec<&str> = group.split(",").collect::<Vec<&str>>();
        for letter in temp[0].chars() {
            if temp[1].contains(letter) && temp[2].contains(letter) {
                total_prio += priority_string.find(letter).unwrap();
                break;
            }
        }
    }

    total_prio
}
