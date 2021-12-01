use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Window {
    starting_index: usize,
    pub numbers: [Option<i32>; 3],
}

impl Window {
    pub fn new(starting_index: usize) -> Self {
        Self {
            starting_index,
            numbers: [None, None, None],
        }
    }

    pub fn add(&mut self, number: i32) {
        if self.numbers[0].is_none() {
            self.numbers[0] = Some(number);
            println!("Added to {:?}", self);
            return;
        }

        if self.numbers[1].is_none() {
            self.numbers[1] = Some(number);
            println!("Added to {:?}", self);
            return;
        }

        if self.numbers[2].is_none() {
            self.numbers[2] = Some(number);
            println!("Added to {:?}", self);
            return;
        }
    }

    pub fn is_ready(&self) -> bool {
        self.numbers[0].is_some() && self.numbers[1].is_some() && self.numbers[2].is_some()
    }

    pub fn sum(&self) -> i32 {
        self.numbers[0].unwrap() + self.numbers[1].unwrap() + self.numbers[2].unwrap()
    }
}

fn main() {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut windows: Vec<Window> = vec![];
    for (index, line) in reader.lines().enumerate() {
        let curr: i32 = line.unwrap().parse::<i32>().unwrap();
        println!("{}", curr);
        windows.push(Window::new(index));
        for window in &mut windows {
            window.add(curr);
        }
    }

    let sums = windows
        .iter()
        .filter(|window| window.is_ready())
        .map(|window| window.sum())
        .collect::<Vec<i32>>();
    let mut increases: i32 = 0;
    let mut prev = None;
    for curr in sums {
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
