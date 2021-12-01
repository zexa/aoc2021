use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default)]
struct Window {
    pub numbers: [Option<i32>; 3],
}

impl Window {
    pub fn add(&mut self, number: i32) {
        // ech, cant be bothered rn
        //let nums = self
        //    .numbers
        //    .iter()
        //    .filter(|number| number.is_none())
        //    .take(1)
        //    .map(|num| Some(number).as_ref())
        //    .collect();

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
        self.numbers.iter().all(|number| number.is_some())
    }

    pub fn sum(&self) -> i32 {
        self.numbers
            .iter()
            .map(|number| number.unwrap())
            .reduce(|sum, number| sum + number)
            .unwrap()
    }
}

fn main() {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut windows: Vec<Window> = vec![];
    for line in reader.lines() {
        let curr: i32 = line.unwrap().parse::<i32>().unwrap();
        println!("{}", curr);
        windows.push(Window::default());
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
