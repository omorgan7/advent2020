use std::fs::File;
use std::io::BufReader;
use std::string::String;
use std::io::prelude::*;
use std::vec::Vec;

fn get_input() -> Vec<i64> {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
  
    contents.split("\n").map(|line| {
      line.parse::<i64>().unwrap()
    }).collect()
  }

fn part1(input : &Vec<i64>) -> i64 {

    
    let preamble_length = 25;
    let mut answer = -1;

    for n in preamble_length..input.len() {
        let first = n - preamble_length;
        
        let slice = &input[first..n];
        let test = input[n];

        let mut valid = false;
        for i in 0..preamble_length {
            for j in i..preamble_length {
                if slice[i] + slice[j] == test && slice[i] != slice[j] {
                    valid = true;
                    break;
                }
            }
        }

        if valid == false {
            answer = test;
            break;
        }
    }
    return answer;
}

fn part2(input : &Vec<i64>, target : i64) {

    let mut start = 0;
    let mut end = 1;

    let mut sum = input[start];
    loop {
        if sum == target {
            break;
        }
        if sum < target {
            sum += input[end];
            end += 1;
        }
        if sum > target {
            sum -= input[start];
            start += 1
        }
    }

    
    println!("{}", input[start..end].iter().min().unwrap() + input[start..end].iter().max().unwrap());
}

fn main() {
    let input = get_input();

    let output = part1(&input);
    println!("{}", output);
    part2(&input, output);
}
