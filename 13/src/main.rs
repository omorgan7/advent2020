use std::fs::File;
use std::io::BufReader;
use std::string::String;
use std::io::prelude::*;
use std::vec::Vec;

fn part1() {

    fn get_input() -> (Vec<i64>, i64) {
        let file = File::open("input.txt").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
    
        let mut it = contents.split("\n");
    
        let target = it.next().unwrap().parse::<i64>().unwrap();
        let output = it.next().unwrap()
                        .split(",")
                        .filter(|s| *s != "x")
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect();
    
        (output, target)
    }

    let (input, target) = get_input();
    let mut nearest_id : Option<i64> = None;
    let mut nearest_time : i64 = std::i64::MAX;
    for id in input {
        let lower_bound = (target / id) * id;
        let upper_bound = ((target / id) + 1) * id;

        if lower_bound >= target && lower_bound - target < nearest_time {
            nearest_id = Some(id);
            nearest_time = lower_bound - target;
            continue;
        }

        if upper_bound >= target && upper_bound - target < nearest_time {
            nearest_id = Some(id);
            nearest_time = upper_bound - target;
        }
    }
    println!("{}", nearest_id.unwrap() * nearest_time);
}

fn part2() {

    fn get_input() -> Vec<(i64, i64)> {
        let file = File::open("input.txt").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
    
        let mut it = contents.split("\n");
    
        it.next();
        let output = it.next().unwrap()
                        .split(",")
                        .enumerate()
                        .filter_map(|(i, s)| {
                            if s == "x" {
                                return None;
                            }

                            Some((i as i64, s.parse::<i64>().unwrap()))
                        })
                        .collect();
    
        output
    }
    let input = get_input();

    let mut first : i64 = 0;
    let mut multiplier = input[0].1;
    let mut next = 1;

    loop {
        first += multiplier;

        if (first + input[next].0) % input[next].1 == 0 {
            multiplier *= input[next].1;
            next += 1;
        }

        if next == input.len() {
            break;
        }
    }
    println!("{}", first);
}

fn main() {
    part1();
    part2();
}
