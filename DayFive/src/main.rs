use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let stacks: Vec<Vec<char>> = get_stacks("./stacks.txt");
    let instructions: Vec<Vec<usize>> = get_input("./input.txt");

    let final_stacks: Vec<Vec<char>> = apply_input_part_one(&instructions, &stacks);

    let mut final_format: String = String::new();

    for row in final_stacks {
        final_format.push(row[row.len() - 1]);
    }
    println!("{}", final_format);

    let final_stacks_two: Vec<Vec<char>> = apply_input_part_two(&instructions, &stacks);

    let mut final_format_two: String = String::new();
    for row in final_stacks_two {
        final_format_two.push(row[row.len() - 1]);
    }
    println!("{}", final_format_two);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_stacks(filename: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                stacks.push(ip);
            }
        }
    }

    let mut final_stacks: Vec<Vec<char>> = vec![vec![]; 9];

    stacks.reverse();

    for row in &stacks[1..] {
        let mut stack: char;
        let mut count: usize = 0;
        for bo in (0..row.len()).step_by(4) {
            stack = row.chars().nth(bo + 1).unwrap();
            if stack != ' ' {
                final_stacks[count].push(stack);
            }
            count += 1;
        }
    }

    final_stacks
}

fn get_input(filename: &str) -> Vec<Vec<usize>> {
    let mut instructions: Vec<Vec<usize>> = vec![];
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let mut step: Vec<usize> = Vec::new();
                let split: Vec<&str> = ip.split(" ").collect::<Vec<&str>>();
                step.push(split[1].parse::<usize>().unwrap());
                step.push(split[3].parse::<usize>().unwrap());
                step.push(split[5].parse::<usize>().unwrap());
                instructions.push(step);
            }
        }
    }
    instructions
}

fn apply_input_part_one(instructions: &Vec<Vec<usize>>, stacks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut final_stacks: Vec<Vec<char>> = stacks.clone();

    for step in instructions {
        for i in 0..step[0] {
            let val: char = final_stacks[step[1] - 1].pop().unwrap();
            final_stacks[step[2] - 1].push(val);
        }
    }

    final_stacks
}

fn apply_input_part_two(instructions: &Vec<Vec<usize>>, stacks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut final_stacks: Vec<Vec<char>> = stacks.clone();

    for step in instructions {
        let count: usize = final_stacks[step[1] - 1].len();
        let vals: Vec<char> = final_stacks[step[1] - 1]
            .drain(count - step[0]..)
            .collect::<Vec<char>>();

        final_stacks[step[2] - 1].extend(&vals);
    }

    final_stacks
}
