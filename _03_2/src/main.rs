use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // index represents pos, char represents (1/2), usize represents instances
    let mut maps: Vec<HashMap<char, usize>> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        for (index, char) in line.chars().enumerate() {
            if maps.get(index).is_none() {
                maps.push(HashMap::<char, usize>::new())
            }

            let map = maps.get_mut(index).unwrap();
            if map.get(&char).is_none() {
                map.insert(char, 1);
                continue;
            }

            let elem = map.get_mut(&char).unwrap();
            *elem = *elem + 1_usize;
        }
    }

    let mut gamma: Vec<char> = vec![];
    for map in maps.iter() {
        let one = map.get(&'1').unwrap_or(&0);
        let zero = map.get(&'0').unwrap_or(&0);
        if one > zero {
            gamma.push('1');
        } else {
            gamma.push('0');
        }
    }

    let mut epsilon: Vec<char> = vec![];
    for map in maps.iter() {
        let one = map.get(&'1').unwrap_or(&0);
        let zero = map.get(&'0').unwrap_or(&0);
        if one < zero {
            epsilon.push('1');
        } else {
            epsilon.push('0');
        }
    }

    //.parse::<i32>().unwrap();
    let gamma = gamma.into_iter().collect::<String>();
    let gamma = usize::from_str_radix(gamma.as_str(), 2).unwrap();

    let epsilon = epsilon.into_iter().collect::<String>();
    let epsilon = usize::from_str_radix(epsilon.as_str(), 2).unwrap();

    let result = gamma * epsilon;
    println!("Gamma: {}, Epsilon: {}, Result: {}", gamma, epsilon, result);
}
