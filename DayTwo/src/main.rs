use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut all_turns: Vec<(char, char)> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let turn: Vec<&str> = ip.split_whitespace().collect();
                all_turns.push((
                    turn[0].chars().collect::<Vec<char>>()[0],
                    turn[1].chars().collect::<Vec<char>>()[0],
                ));
            }
        }
    }

    let mut total_points_one = 0;
    let mut total_points_two = 0;
    for turn in all_turns {
        total_points_one += compute_points_part_one(turn);
        total_points_two += compute_points_part_two(turn);
    }
    println!("{}", total_points_one);
    println!("{}", total_points_two);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn compute_points_part_one(turn: (char, char)) -> u32 {
    let mut points: u32 = 0;

    match turn {
        ('A', 'X') => points = 4,
        ('B', 'X') => points = 1,
        ('C', 'X') => points = 7,
        ('A', 'Y') => points = 8,
        ('B', 'Y') => points = 5,
        ('C', 'Y') => points = 2,
        ('A', 'Z') => points = 3,
        ('B', 'Z') => points = 9,
        ('C', 'Z') => points = 6,
        _ => {}
    }

    points
}

fn compute_points_part_two(turn: (char, char)) -> u32 {
    let mut points: u32 = 0;
    match turn {
        ('A', 'X') => points = 3,
        ('A', 'Y') => points = 4,
        ('A', 'Z') => points = 8,
        ('B', 'X') => points = 1,
        ('B', 'Y') => points = 5,
        ('B', 'Z') => points = 9,
        ('C', 'X') => points = 2,
        ('C', 'Y') => points = 6,
        ('C', 'Z') => points = 7,
        _ => {}
    }
    points
}
