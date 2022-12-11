use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let test_str: String = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
    let mut actual_string: String = String::from("");
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                actual_string = ip;
            }
        }
    }

    println!("{}", get_marker_part_one(&test_str));
    println!("{}", get_marker_part_one(&actual_string));
    println!("{}", get_marker_part_two(&test_str));
    println!("{}", get_marker_part_two(&actual_string));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_marker_part_one(in_str: &str) -> usize {
    let mut count = 4;
    for pos in 0..in_str.len() - 3 {
        if in_str[pos..pos + 4]
            .chars()
            .collect::<HashSet<char>>()
            .len()
            == 4
        {
            return count;
        }
        count += 1;
    }
    count
}
fn get_marker_part_two(in_str: &str) -> usize {
    let mut count = 14;
    for pos in 0..in_str.len() - 13 {
        if in_str[pos..pos + 14]
            .chars()
            .collect::<HashSet<char>>()
            .len()
            == 14
        {
            return count;
        }
        count += 1;
    }
    count
}
