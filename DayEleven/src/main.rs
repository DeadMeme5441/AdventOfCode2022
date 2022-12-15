use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: (char, String),
    test: usize,
    true_val: usize,
    false_val: usize,
    inspection: usize,
}

fn main() {
    let mut monkeys: Vec<Monkey> = Vec::new();
    if let Ok(lines) = read_lines("./jnput.txt") {
        let mut temp: Vec<String> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() != 0 {
                    temp.push(ip);
                }
                if temp.len() == 6 {
                    let mon: Monkey = Monkey {
                        items: temp[1].split(":").collect::<Vec<&str>>()[1]
                            .split(",")
                            .collect::<Vec<&str>>()
                            .iter()
                            .map(|x| x.trim().parse::<usize>().unwrap())
                            .collect::<Vec<usize>>(),
                        operation: (
                            temp[2].split(":").collect::<Vec<&str>>()[1]
                                .split(" ")
                                .collect::<Vec<&str>>()[4]
                                .chars()
                                .next()
                                .unwrap(),
                            temp[2].split(":").collect::<Vec<&str>>()[1]
                                .split(" ")
                                .collect::<Vec<&str>>()[5]
                                .trim()
                                .to_string(),
                        ),
                        test: temp[3].split(":").collect::<Vec<&str>>()[1]
                            .split(" ")
                            .collect::<Vec<&str>>()[3]
                            .parse::<usize>()
                            .unwrap(),
                        true_val: temp[4].split(":").collect::<Vec<&str>>()[1]
                            .split(" ")
                            .collect::<Vec<&str>>()[4]
                            .parse::<usize>()
                            .unwrap(),
                        false_val: temp[5].split(":").collect::<Vec<&str>>()[1]
                            .split(" ")
                            .collect::<Vec<&str>>()[4]
                            .parse::<usize>()
                            .unwrap(),
                        inspection: 0,
                    };
                    println!("{:?}", mon);
                    monkeys.push(mon);
                    temp = vec![];
                }
            }
        }
    }
    println!("{:?}", monkeys);
    // for round in 0..20 {
    //     part_one(&mut monkeys);
    // }

    // println!("{:?}", monkeys);

    let mut inspections: Vec<usize> = monkeys.iter().map(|x| x.inspection).collect::<Vec<usize>>();

    for round in 0..10000 {
        part_two(&mut monkeys);
    }

    inspections = monkeys.iter().map(|x| x.inspection).collect::<Vec<usize>>();

    println!("{:?}", inspections);

    inspections.sort();
    inspections.reverse();

    println!("{:?}", inspections[0] * inspections[1]);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_one(monkeys: &mut Vec<Monkey>) {
    for pos in 0..monkeys.len() {
        // println!("Monkey {}:", &pos);
        for item in monkeys[pos].items.clone() {
            monkeys[pos].inspection += 1;
            // println!("Monkey inspects an item with a worry level of {}", &item);
            let mut new_item_val = 0;
            if monkeys[pos].operation.0 == '+' {
                if monkeys[pos].operation.1 == "old" {
                    new_item_val = item + item;
                } else {
                    new_item_val = item + monkeys[pos].operation.1.parse::<usize>().unwrap();
                }
            }
            if monkeys[pos].operation.0 == '-' {
                if monkeys[pos].operation.1 == "old" {
                    new_item_val = item - item;
                } else {
                    new_item_val = item - monkeys[pos].operation.1.parse::<usize>().unwrap();
                }
            }
            if monkeys[pos].operation.0 == '*' {
                if monkeys[pos].operation.1 == "old" {
                    new_item_val = item * item;
                } else {
                    new_item_val = item * monkeys[pos].operation.1.parse::<usize>().unwrap();
                }
            }
            if monkeys[pos].operation.0 == '/' {
                if monkeys[pos].operation.1 == "old" {
                    new_item_val = item / item;
                } else {
                    new_item_val = item / monkeys[pos].operation.1.parse::<usize>().unwrap();
                }
            }
            // println!("Worry level is {}", &new_item_val);
            new_item_val /= 3;
            // println!("Bored, hence level is {}", &new_item_val);
            if new_item_val % monkeys[pos].test == 0 {
                let new_pos = monkeys[pos].true_val;
                monkeys[new_pos].items.push(new_item_val);
                monkeys[pos].items.retain(|x| *x != item);
                // println!("True, thrown to {}", new_pos);
            } else {
                let new_pos = monkeys[pos].false_val;
                monkeys[new_pos].items.push(new_item_val);
                monkeys[pos].items.retain(|x| *x != item);
                // println!("Not true, thrown to {}", new_pos);
            }
        }
    }
}

fn part_two(monkeys: &mut Vec<Monkey>) {
    let mut stress_val: usize = 1;
    for mon in monkeys.into_iter() {
        stress_val *= mon.test;
    }
    for pos in 0..monkeys.len() {
        // println!("Monkey {}:", &pos);
        for item in monkeys[pos].items.clone() {
            monkeys[pos].inspection += 1;
            // println!("Monkey inspects an item with a worry level of {}", &item);
            let mut new_item_val = 0;
            if monkeys[pos].operation.0 == '+' {
                if monkeys[pos].operation.1 == "old" {
                    new_item_val = item + item;
                } else {
                    new_item_val = item + monkeys[pos].operation.1.parse::<usize>().unwrap();
                }
            }
            if monkeys[pos].operation.0 == '-' {
                if monkeys[pos].operation.1 == "old" {
                    new_item_val = item - item;
                } else {
                    new_item_val = item - monkeys[pos].operation.1.parse::<usize>().unwrap();
                }
            }
            if monkeys[pos].operation.0 == '*' {
                if monkeys[pos].operation.1 == "old" {
                    new_item_val = item * item;
                } else {
                    new_item_val = item * monkeys[pos].operation.1.parse::<usize>().unwrap();
                }
            }
            if monkeys[pos].operation.0 == '/' {
                if monkeys[pos].operation.1 == "old" {
                    new_item_val = item / item;
                } else {
                    new_item_val = item / monkeys[pos].operation.1.parse::<usize>().unwrap();
                }
            }
            // println!("Worry level is {}", &new_item_val);
            // println!("Bored, hence level is {}", &new_item_val);
            new_item_val %= stress_val;
            if new_item_val % monkeys[pos].test == 0 {
                let new_pos = monkeys[pos].true_val;
                monkeys[new_pos].items.push(new_item_val);
                monkeys[pos].items.retain(|x| *x != item);
                // println!("True, thrown to {}", new_pos);
            } else {
                let new_pos = monkeys[pos].false_val;
                monkeys[new_pos].items.push(new_item_val);
                monkeys[pos].items.retain(|x| *x != item);
                // println!("Not true, thrown to {}", new_pos);
            }
        }
    }
}
