use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (command, amount) = line.split_at(line.find(" ").unwrap());
        let amount = amount.trim().parse::<i32>().unwrap();
        match command {
            "forward" => {
                horizontal = horizontal + amount;
                depth = depth + aim * amount;
            },
            "down" => {
                aim = aim + amount;
            },
            "up" => {
                aim = aim - amount;
            }
            _ => {
                panic!("Unknown command {}", command);
            }
        }
        println!("{} - {}: horizontal: {}; depth: {}; aim: {};", command, amount, horizontal, depth, aim);
    }

    let result = horizontal * depth;
    println!("horizontal: {}; depth: {}; aim: {}; result: {}", horizontal, depth, aim, result);
}
