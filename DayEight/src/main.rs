use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut trees: Vec<Vec<usize>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                trees.push(
                    ip.chars()
                        .map(|s| s.to_digit(10).unwrap() as usize)
                        .collect::<Vec<usize>>(),
                );
            }
        }
    }

    part_one(&trees);
    part_two(&trees);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_one(trees: &Vec<Vec<usize>>) -> usize {
    let mut total_visible: usize = 4 * (trees.len() - 1);

    let mut count = 0;

    println!("{:?}", trees.iter().map(|x| x[2]).collect::<Vec<usize>>());
    for row in 1..trees.len() - 1 {
        for col in 1..trees[row].len() - 1 {
            let tree = trees[row][col];

            if tree > *trees[row][0..col].iter().max().unwrap() {
                total_visible += 1;
                continue;
            }
            if tree > *trees[row][col + 1..trees.len()].iter().max().unwrap() {
                total_visible += 1;
                continue;
            }
            if tree
                > *trees.iter().map(|x| x[col]).collect::<Vec<usize>>()[0..row]
                    .iter()
                    .max()
                    .unwrap()
            {
                total_visible += 1;
                continue;
            }
            if tree
                > *trees.iter().map(|x| x[col]).collect::<Vec<usize>>()[row + 1..trees.len()]
                    .iter()
                    .max()
                    .unwrap()
            {
                total_visible += 1;
                continue;
            }
        }
    }

    println!("{}", total_visible);
    total_visible
}

fn part_two(trees: &Vec<Vec<usize>>) -> usize {
    let mut highest_scenic_score = 0;

    for row in 1..trees.len() - 1 {
        for col in 1..trees[row].len() - 1 {
            let tree = trees[row][col];

            let left_val: usize = trees[row][0..col]
                .iter()
                .rev()
                .position(|x| x >= &tree)
                .unwrap_or(col - 1)
                + 1;

            let right_val: usize = trees[row][col + 1..]
                .iter()
                .position(|x| x >= &tree)
                .unwrap_or(trees.len() - col - 2)
                + 1;

            let top_val: usize = trees.iter().map(|x| x[col]).collect::<Vec<usize>>()[0..row]
                .iter()
                .rev()
                .position(|x| x >= &tree)
                .unwrap_or(row - 1)
                + 1;

            let bottom_val: usize = trees.iter().map(|x| x[col]).collect::<Vec<usize>>()[row + 1..]
                .iter()
                .position(|x| x >= &tree)
                .unwrap_or(trees.len() - row - 2)
                + 1;

            let scenic_score: usize = left_val * right_val * top_val * bottom_val;

            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }

    println!("{}", highest_scenic_score);
    highest_scenic_score
}
