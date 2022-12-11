use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{thread, time};

fn main() {
    let mut instructions: Vec<(char, isize)> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<&str> = ip.split(" ").collect::<Vec<&str>>();
                instructions.push((
                    split[0].chars().next().unwrap(),
                    split[1].parse::<isize>().unwrap(),
                ));
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

fn part_one(instructions: &Vec<(char, isize)>) {
    let mut head_position: (isize, isize) = (0, 0);
    let mut tail_position: (isize, isize) = (0, 0);
    let mut unique_position: HashSet<(isize, isize)> = HashSet::new();

    for line in instructions {
        if line.0 == 'R' {
            for i in 0..line.1 {
                (head_position, tail_position) =
                    compute_movement(&(head_position.0, head_position.1 + 1), &tail_position);
                unique_position.insert(tail_position);
            }
        }
        if line.0 == 'L' {
            for i in 0..line.1 {
                (head_position, tail_position) =
                    compute_movement(&(head_position.0, head_position.1 - 1), &tail_position);
                unique_position.insert(tail_position);
            }
        }
        if line.0 == 'U' {
            for i in 0..line.1 {
                (head_position, tail_position) =
                    compute_movement(&(head_position.0 + 1, head_position.1), &tail_position);
                unique_position.insert(tail_position);
            }
        }
        if line.0 == 'D' {
            for i in 0..line.1 {
                (head_position, tail_position) =
                    compute_movement(&(head_position.0 - 1, head_position.1), &tail_position);
                unique_position.insert(tail_position);
            }
        }
    }

    println!("{:?}", unique_position.len());
}

fn part_two(instructions: &Vec<(char, isize)>) {
    let mut rope: Vec<(isize, isize)> = vec![(0, 0); 10];

    let mut unique_position: HashSet<(isize, isize)> = HashSet::new();

    for line in instructions {
        if line.0 == 'R' {
            for i in 0..line.1 {
                for knot in 0..rope.len() - 1 {
                    if knot == 0 {
                        rope[knot].1 += 1;
                    }
                    (rope[knot], rope[knot + 1]) = compute_movement(&rope[knot], &rope[knot + 1]);
                    if knot == rope.len() - 2 {
                        unique_position.insert(rope[knot + 1]);
                    }
                }
            }
        } else if line.0 == 'L' {
            for i in 0..line.1 {
                for knot in 0..rope.len() - 1 {
                    if knot == 0 {
                        rope[knot].1 -= 1;
                    }
                    (rope[knot], rope[knot + 1]) = compute_movement(&rope[knot], &rope[knot + 1]);
                    if knot == rope.len() - 2 {
                        unique_position.insert(rope[knot + 1]);
                    }
                }
            }
        } else if line.0 == 'U' {
            for i in 0..line.1 {
                for knot in 0..rope.len() - 1 {
                    if knot == 0 {
                        rope[knot].0 += 1;
                    }
                    (rope[knot], rope[knot + 1]) = compute_movement(&rope[knot], &rope[knot + 1]);
                    if knot == rope.len() - 2 {
                        unique_position.insert(rope[knot + 1]);
                    }
                }
            }
        } else if line.0 == 'D' {
            for i in 0..line.1 {
                for knot in 0..rope.len() - 1 {
                    if knot == 0 {
                        rope[knot].0 -= 1;
                    }
                    (rope[knot], rope[knot + 1]) = compute_movement(&rope[knot], &rope[knot + 1]);
                    if knot == rope.len() - 2 {
                        unique_position.insert(rope[knot + 1]);
                    }
                }
            }
        }
    }

    println!("{:?}", unique_position.len());
    display_plot_two(&unique_position);
}

fn compute_movement(
    head_position: &(isize, isize),
    tail_position: &(isize, isize),
) -> ((isize, isize), (isize, isize)) {
    let new_head_position = *head_position;

    let mut new_tail_position: (isize, isize) = *tail_position;

    if ((new_head_position.0 - new_tail_position.0).abs() == 2
        && (new_head_position.1 != new_tail_position.1))
        || ((new_head_position.1 - new_tail_position.1).abs() == 2
            && (new_head_position.0 != new_tail_position.0))
    {
        if new_head_position.0 > new_tail_position.0 && new_head_position.1 > new_tail_position.1 {
            new_tail_position.0 += 1;
            new_tail_position.1 += 1;
        } else if new_head_position.0 > new_tail_position.0
            && new_head_position.1 < new_tail_position.1
        {
            new_tail_position.0 += 1;
            new_tail_position.1 -= 1;
        } else if new_head_position.0 < new_tail_position.0
            && new_head_position.1 > new_tail_position.1
        {
            new_tail_position.0 -= 1;
            new_tail_position.1 += 1;
        } else if new_head_position.0 < new_tail_position.0
            && new_head_position.1 < new_tail_position.1
        {
            new_tail_position.0 -= 1;
            new_tail_position.1 -= 1;
        }
    } else {
        if (new_head_position.0 - new_tail_position.0).abs() > 1 {
            if new_head_position.0 > new_tail_position.0 {
                new_tail_position.0 += 1;
            } else if new_head_position.0 < new_tail_position.0 {
                new_tail_position.0 -= 1;
            }
        }
        if (new_head_position.1 - new_tail_position.1).abs() > 1 {
            if new_head_position.1 > new_tail_position.1 {
                new_tail_position.1 += 1;
            } else if new_head_position.1 < new_tail_position.1 {
                new_tail_position.1 -= 1;
            }
        }
    }
    (new_head_position, new_tail_position)
}

fn display_plot(head_position: &(isize, isize), tail_position: &(isize, isize)) {
    for i in (0..6).rev() {
        for j in 0..6 {
            if (i, j) == *head_position {
                print!("H");
            } else if (i, j) == *tail_position {
                print!("T");
            } else if (i, j) == (0, 0) {
                print!("s");
            } else {
                print!(".");
            }
        }
        println!();
    }

    thread::sleep(time::Duration::from_secs(1));
}

fn display_plot_two(rope: &HashSet<(isize, isize)>) {
    let x: (isize, isize) = (
        *rope
            .iter()
            .map(|x| x.0)
            .collect::<Vec<isize>>()
            .iter()
            .min()
            .unwrap(),
        *rope
            .iter()
            .map(|x| x.0)
            .collect::<Vec<isize>>()
            .iter()
            .max()
            .unwrap(),
    );
    let y: (isize, isize) = (
        *rope
            .iter()
            .map(|x| x.1)
            .collect::<Vec<isize>>()
            .iter()
            .min()
            .unwrap(),
        *rope
            .iter()
            .map(|x| x.1)
            .collect::<Vec<isize>>()
            .iter()
            .max()
            .unwrap(),
    );

    println!("{:?} to {:?}", x, y);
    for i in (x.0..x.1 + 1).rev() {
        for j in y.0..y.1 + 1 {
            if i == 0 && j == 0 {
                print!("s");
                continue;
            } else if rope.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    thread::sleep(time::Duration::from_secs(1));
}
