use std::fs::File;
use std::io::BufReader;
use std::string::String;
use std::io::prelude::*;
use std::vec::Vec;
use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

extern crate either;
use either::*;

#[derive(Debug, Clone, PartialEq)]
struct Mask {
    value : String
}

#[derive(Debug, Clone, PartialEq)]
struct Mem {
    index: i64,
    value: i64
}

type Instruction = Either<Mask, Mem>;

fn get_input() -> Vec<Instruction> {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let re = [Regex::new(r"(mask) = (.+)").unwrap(), Regex::new(r"(mem)\[(\d+)\] = (\d+)").unwrap()];

    let output : Vec<Instruction> = contents.split("\n").map(|line| {
        let mut ret : Option<Instruction> = None;
        for r in &re {
            if let Some(captures) = r.captures(line) {
                match captures.get(1).unwrap().as_str() {
                    "mask" => { ret = Some(Left(Mask{value : captures.get(2).unwrap().as_str().to_string()})) },
                    "mem" => {
                            ret = Some(Right(Mem{
                                index: captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                                value: captures.get(3).unwrap().as_str().parse::<i64>().unwrap()}))
                            },
                    _ => panic!("Unreachable case")
                }
            }
            else {
                continue;
            }
        }
        ret.unwrap()
    }).collect();

    output
}

fn part1(input : &Vec<Instruction>) {
    let max_mem = input.iter()
            .filter(|i| i.is_right())
            .max_by_key(|&x| x.as_ref().right().unwrap().index)
            .unwrap().clone().right().unwrap().index;

    let mut memory = vec!(0; (max_mem + 1) as usize);

    let mut mask = input[0].clone().left().unwrap().value;
    for i in input {
        match i {
            Left(ins) => { mask = ins.clone().value; },
            Right(ins) => {
                memory[ins.index as usize] = ins.value;
                for (index, bit) in mask.chars().enumerate() {
                    let target_bit = 35 - index;
                    match bit {
                        '1' => {
                            memory[ins.index as usize] |= 1 << target_bit;
                        },
                        '0' => {
                            memory[ins.index as usize] &= !(1 << target_bit);
                        },
                        'X' => {},
                        _ => { panic!("unreachable"); }
                    }
                }
            }
        }
    }
    println!("{}", memory.iter().sum::<i64>());
}

fn combinations(start: usize, base : i64, floats : &Vec<i64>, floats_out : &mut HashSet<i64>) {
    if start == floats.len() {
        return;
    }

    for f in &floats[start..] {
        let c = base | (1 << f);
        floats_out.insert(c);
        combinations(start + 1, c, floats, floats_out);

        let d = base & !(1 << f);
        floats_out.insert(d);
        combinations(start + 1, d, floats, floats_out);
    }
}

fn part2(input : &Vec<Instruction>) {
    let mut memory = HashMap::<i64, i64>::new();

    let mut mask = input[0].clone().left().unwrap().value;
    let mut floats = Vec::<i64>::new();
    let mut float_combinations = HashSet::<i64>::new();
    
    for (progress, i) in input.iter().enumerate() {
        match i {
            Left(ins) => { mask = ins.clone().value; },
            Right(ins) => {
                let mut bitmask : i64 = 0;
                let mut x_count : i64 = 0;

                
                floats.clear();
                for (index, bit) in mask.chars().enumerate() {
                    let target_bit = 35 - (index as i64);
                    match bit {
                        '1' => {
                            bitmask |= 1i64 << target_bit;
                        },
                        '0' => {}
                        'X' => { floats.push(target_bit); },
                        _ => { panic!("unreachable"); }
                    }
                }
                
                float_combinations.clear();
                combinations(0, bitmask | ins.index, &floats, &mut float_combinations);

                for f in &float_combinations {
                    memory.insert(*f, ins.value);
                }
            }
        }
    }
    println!("{}", memory.iter().fold(0, |acc, x| acc + x.1));
}

fn main() {
    let input = get_input();
    part1(&input);
    part2(&input);
}
