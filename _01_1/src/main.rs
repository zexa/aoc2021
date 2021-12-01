use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut increases: i32 = 0;
    let mut prev = None;
    for line in reader.lines() {
        let curr: i32 = line.unwrap().parse::<i32>().unwrap();
        print!("{} ", curr);

        if prev.is_none() {
            prev = Some(curr);
            print!("\n");
            continue;
        }

        print!("(prev: {}, curr: {}) ", prev.unwrap(), curr);
        if curr > prev.unwrap() {
            increases = increases + 1;
            print!("(increase)");
        } else {
            print!("(decrease)");
        }

        prev = Some(curr);

        print!("\n");
    }

    println!("total: {}", increases);
}
