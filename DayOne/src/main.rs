use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut largest: u32 = 0;
        let mut sum: u32 = 0;
        let mut total_carried: Vec<u32> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    if sum > largest {
                        largest = sum;
                    }
                    sum = 0;
                } else {
                    sum += ip.parse::<u32>().unwrap();
                    total_carried.push(sum);
                }
            }
        }
        println!("{}", largest);
        total_carried.sort();
        total_carried.reverse();
        println!(
            "{:?}",
            total_carried[0] + total_carried[1] + total_carried[2]
        );
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
