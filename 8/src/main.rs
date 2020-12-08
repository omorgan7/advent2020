use std::fs::File;
use std::io::BufReader;
use std::string::String;
use std::io::prelude::*;
use std::vec::Vec;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Instruction {
  Nop(i32),
  Acc(i32),
  Jmp(i32)
}


fn get_input() -> Vec<Instruction> {
  let file = File::open("input.txt").unwrap();
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents).unwrap();

  let re = Regex::new(r"(.+) ([+-]\d+)").unwrap();
  contents.split("\n").map(|line| {
    let captures = re.captures(line).unwrap();
    let instruction_name = captures.get(1).unwrap().as_str();
    let value = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

    match instruction_name {
        "nop" => Instruction::Nop(value),
        "acc" => Instruction::Acc(value),
        "jmp" => Instruction::Jmp(value),
        _ => panic!()
    }
  }).collect()
}

fn part1(input : &Vec<Instruction>) {
    let mut instruction_set = HashSet::<i32>::new();

    let mut program_counter : i32 = 0;
    let mut accumulator : i32 = 0;
    loop {
        let i = input[program_counter as usize];
        if instruction_set.contains(&program_counter) {
            break;
        }

        instruction_set.insert(program_counter);
        match i {
            Instruction::Nop(_) => {
                program_counter += 1;
            },
            Instruction::Acc(value) => {
                accumulator += value;
                program_counter += 1;
            },
            Instruction::Jmp(value) => {
                program_counter += value;
            }
        }
    }

    println!("{}", accumulator);
}

fn part2(mut input : Vec<Instruction>) {
    let mut instruction_set = HashSet::<i32>::new();

    for last_instruction in 0..input.len() {
        
        match input[last_instruction] {
            Instruction::Nop(value) => { input[last_instruction] = Instruction::Jmp(value); },
            Instruction::Jmp(value) => { input[last_instruction] = Instruction::Nop(value); },
            Instruction::Acc(_) => {}
        }

        let mut program_counter : i32 = 0;
        let mut accumulator : i32 = 0;
        let mut halted = false;
        loop {
            if program_counter as usize >= input.len() {
                halted = true;
                break;
            }
            let i = input[program_counter as usize];
    
            if instruction_set.contains(&program_counter) {
                instruction_set.clear();
                break;
            }
    
            instruction_set.insert(program_counter);
            match i {
                Instruction::Nop(_) => {
                    program_counter += 1;
                },
                Instruction::Acc(value) => {
                    accumulator += value;
                    program_counter += 1;
                },
                Instruction::Jmp(value) => {
                    program_counter += value;
                }
            }
        }
        if halted {
            println!("{}", accumulator);
            break;
        }
        else {
            // swap back to the old value.
            match input[last_instruction] {
                Instruction::Nop(value) => { input[last_instruction] = Instruction::Jmp(value); },
                Instruction::Jmp(value) => { input[last_instruction] = Instruction::Nop(value); },
                Instruction::Acc(_) => {}
            }
        }
    }
}

fn main() {
    let input = get_input();
    part1(&input);
    part2(input);
}