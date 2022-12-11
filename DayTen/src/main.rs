use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut instructions: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                instructions.push(ip.to_string());
            }
        }
    }
    part_one(&instructions);
    part_two(&instructions);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_one(instructions: &Vec<String>) {
    let mut cycle: usize = 1;
    let mut val: isize = 1;
    let mut total_val: isize = 0;
    let cycles_list: Vec<usize> = vec![20, 60, 100, 140, 180, 220];

    for step in instructions.into_iter() {
        if step == "noop" {
            cycle += 1;
            if cycles_list.contains(&cycle) {
                total_val += cycle as isize * val;
                println!("{:?}", (cycle, val));
            }
        } else {
            cycle += 1;
            if cycles_list.contains(&cycle) {
                total_val += cycle as isize * val;
                println!("{:?}", (cycle, val));
            }
            cycle += 1;
            val += step.split(" ").collect::<Vec<&str>>()[1]
                .parse::<isize>()
                .unwrap();

            if cycles_list.contains(&cycle) {
                total_val += cycle as isize * val;
                println!("{:?}", (cycle, val));
            }
        }
    }

    println!("{:?}", total_val);
}

fn part_two(instructions: &Vec<String>) {
    let mut cycle: usize = 0;
    let mut val: isize = 1;
    let mut sprite_position: isize = 1;
    let mut display: Vec<bool> = vec![false; 240];

    for step in instructions.into_iter() {
        if step == "noop" {
            if [sprite_position - 1, sprite_position, sprite_position + 1]
                .contains(&(cycle as isize))
            {
                display[cycle] = true;
            }
            cycle += 1;
        } else {
            if [sprite_position - 1, sprite_position, sprite_position + 1]
                .contains(&(cycle as isize))
            {
                display[cycle] = true;
            }
            cycle += 1;

            if [sprite_position - 1, sprite_position, sprite_position + 1]
                .contains(&(cycle as isize))
            {
                display[cycle] = true;
            }
            cycle += 1;
            val += step.split(" ").collect::<Vec<&str>>()[1]
                .parse::<isize>()
                .unwrap();

            sprite_position = 40 * ((cycle as f32 / 40.0).floor() as isize) + val;
        }
    }
    println!("{:?}", display);

    for i in 0..6 {
        for j in 0..40 {
            if display[40 * i + j] == true {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
