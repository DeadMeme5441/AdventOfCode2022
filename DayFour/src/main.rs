use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut count_part_one: usize = 0;
    let mut count_part_two: usize = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let pair = ip.split(",").collect::<Vec<&str>>();
                let mut split1 = pair[0].split("-");
                let mem1: (u32, u32) = (
                    split1.next().unwrap().parse::<u32>().unwrap(),
                    split1.next().unwrap().parse::<u32>().unwrap(),
                );
                let mut split2 = pair[1].split("-");
                let mem2: (u32, u32) = (
                    split2.next().unwrap().parse::<u32>().unwrap(),
                    split2.next().unwrap().parse::<u32>().unwrap(),
                );

                if cleanup_part_one(mem1, mem2) == true {
                    count_part_one += 1;
                };
                if cleanup_part_two(mem1, mem2) == true {
                    count_part_two += 1;
                };
            }
        }
    }
    println!("{:?}", count_part_one);
    println!("{:?}", count_part_two);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn cleanup_part_one(mem1: (u32, u32), mem2: (u32, u32)) -> bool {
    if (mem1.0 <= mem2.0 && mem1.1 >= mem2.1) || (mem2.0 <= mem1.0 && mem2.1 >= mem1.1) {
        return true;
    }
    false
}

fn cleanup_part_two(mem1: (u32, u32), mem2: (u32, u32)) -> bool {
    if (mem1.0 <= mem2.0 && mem1.1 >= mem2.0) || (mem2.0 <= mem1.0 && mem2.1 >= mem1.0) {
        return true;
    }
    false
}
