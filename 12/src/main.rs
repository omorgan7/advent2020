use std::fs::File;
use std::io::BufReader;
use std::string::String;
use std::io::prelude::*;
use std::vec::Vec;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32)
}

fn get_input() -> Vec<Instruction> {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let output : Vec<Instruction> = contents.split("\n").map(|line| {
        let number = line[1..].parse::<i32>().unwrap();
        match line.chars().nth(0).unwrap() {
            'N' => Instruction::N(number),
            'S' => Instruction::S(number),
            'E' => Instruction::E(number),
            'W' => Instruction::W(number),
            'L' => Instruction::L(number),
            'R' => Instruction::R(number),
            'F' => Instruction::F(number),
            _ => panic!("Matching case not found.")
        }
    }).collect();

    output
}

fn part1(input : &Vec<Instruction>) {
    let mut position : (i32, i32) = (0, 0);
    let mut bearing = 0;

    for i in input {
        match i {
            Instruction::N(number) => {
                position.1 += number;
            },
            Instruction::S(number) => {
                position.1 -= number;
            },
            Instruction::E(number) => {
                position.0 += number;
            },
            Instruction::W(number) => {
                position.0 -= number;
            },
            Instruction::L(number) => {
                bearing -= number;
                if bearing < 0 {
                    bearing += 360;
                }
            },
            Instruction::R(number) => {
                bearing += number;
                bearing %= 360;
            },
            Instruction::F(number) => {
                match bearing {
                    0 => {
                        position.0 += number;
                    },
                    90 => {
                        position.1 -= number;
                    },
                    180 => {
                        position.0 -= number;
                    },
                    270 => {
                        position.1 += number;
                    },
                    _ => panic!("Unreachable.")
                }
            }
        }
    }
    println!("{} {} {}", position.0, position.1, position.0.abs() + position.1.abs())
}

fn part2(input : &Vec<Instruction>) {

    let mut waypoint : (i32, i32) = (10, 1);
    let mut position : (i32, i32) = (0, 0);

    for i in input {
        match i {
            Instruction::N(number) => {
                waypoint.1 += number;
            },
            Instruction::S(number) => {
                waypoint.1 -= number;
            },
            Instruction::E(number) => {
                waypoint.0 += number;
            },
            Instruction::W(number) => {
                waypoint.0 -= number;
            },
            Instruction::R(number) => {
                let x = waypoint.0 as f64;
                let y = waypoint.1 as f64;

                waypoint.0 = (x * (*number as f64).to_radians().cos() + y * (*number as f64).to_radians().sin()).round() as i32;
                waypoint.1 = (-x * (*number as f64).to_radians().sin() + y * (*number as f64).to_radians().cos()).round() as i32;
            },
            Instruction::L(number) => {
                // easier to just use sin/cos at this point.
                let x = waypoint.0 as f64;
                let y = waypoint.1 as f64;

                waypoint.0 = (x * (-*number as f64).to_radians().cos() + y * (-*number as f64).to_radians().sin()).round() as i32;
                waypoint.1 = (-x * (-*number as f64).to_radians().sin() + y * (-*number as f64).to_radians().cos()).round() as i32;
            },
            Instruction::F(number) => {
                position.0 += number * waypoint.0;
                position.1 += number * waypoint.1;
            }
        }
    }
    println!("{} {} {}", position.0, position.1, position.0.abs() + position.1.abs());
}

fn main() {
    let input = get_input();
    part1(&input);
    part2(&input);
}