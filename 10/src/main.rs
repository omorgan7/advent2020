use std::fs::File;
use std::io::BufReader;
use std::string::String;
use std::io::prelude::*;
use std::vec::Vec;
use std::collections::HashMap;

fn get_input() -> Vec<i64> {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    contents.split("\n").map(|line| {
        line.parse::<i64>().unwrap()
    }).collect()
}

fn part1(input : &Vec<i64>) {

    let mut sorted = input.clone();
    sorted.sort();

    sorted.insert(0, 0);
    sorted.push(sorted[sorted.len() - 1] + 3);

    let mut one_diffs = 0;
    let mut three_diffs = 0;
    for i in 1..sorted.len() {
        one_diffs += (sorted[i] - sorted[i - 1] == 1) as i32;
        three_diffs += (sorted[i] - sorted[i - 1] == 3) as i32;
    }
    println!("{}", one_diffs * three_diffs);
}

fn permutation(input : &Vec<i64>, n : usize, lookup : &mut HashMap<usize, i64>) -> i64 {
    if n >= (input.len() - 1) {
        return 1;
    }

    let mut permutation_count : i64 = 0;
    for j in 1..4 {
        if n + j >= input.len() {
            break;
        }

        if input[n + j] - input[n] < 4 {
            if let Some(count) = lookup.get(&(n + j)) {
                permutation_count += count;
            }
            else {
                permutation_count += permutation(input, n + j, lookup);
            }
        }
    }

    lookup.insert(n, permutation_count);
    return permutation_count;
}

fn part2(input : &Vec<i64>) {
    let mut sorted = input.clone();
    sorted.sort();

    sorted.insert(0, 0);
    sorted.push(sorted[sorted.len() - 1] + 3);

    let mut lookup_table = vec![0 as i64; sorted.len() - 1];

    // only 1 path to reach the 0.
    lookup_table[0] = 1;

    for i in 0..sorted.len() - 1 {
        for j in (i as i64 - 3)..(i as i64) {
            if j < 0 {
                continue;
            }
            if sorted[i] - sorted[j as usize] < 4 {
                lookup_table[i] += lookup_table[j as usize];
            }
        }
    }

    println!("{}", lookup_table[lookup_table.len() - 1]);
}

fn main() {
    let input = get_input();
    part1(&input);
    part2(&input);
}
