use std::fs::File;
use std::io::BufReader;
use std::string::String;
use std::io::prelude::*;
use std::vec::Vec;

extern crate termion;
use termion::clear::All;

use std::{thread, time};


#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    Floor,
    Empty,
    Occupied
}

fn get_input() -> (Vec<State>, i32, i32) {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let height = contents.split("\n").count() as i32;

    let output : Vec<State> = contents.chars().filter(|c| *c != '\n').map(|c| {
        match c {
            'L' => State::Empty,
            '#' => State::Occupied,
            _ => State::Floor
        }
    }).collect();

    let width = output.len() as i32 / height;

    (output, width, height)
}

fn part1(input : Vec<State>, width : i32, height : i32)
{

    // 8 surrounding squares.
    let adjacent = [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1),
    ];

    let mut previous = input.clone();

    loop {
        let mut area = previous.clone();

        for y in 0..height {
            for x in 0..width {
                let seat = x + y * width;
                let state = previous[seat as usize];
                let mut new_state = state;
                match state {
                    State::Floor => { continue; },
                    State::Empty => {
                        let occupied = adjacent.iter().any(|a| {
                            let new_x = x + a.0;
                            let new_y = y + a.1;
                            if new_x < 0 || new_x >= width || new_y < 0 || new_y >= height {
                                return false;
                            }
    
                            let new_seat = new_x + new_y * width;
                            return previous[new_seat as usize] == State::Occupied;
                        });
    
                        if !occupied {
                            new_state = State::Occupied;
                        }
                    },
                    State::Occupied => {
                        let occupy_count = adjacent.iter().filter(|a| {
                            let new_x = x + a.0;
                            let new_y = y + a.1;
                            if new_x < 0 || new_x >= width || new_y < 0 || new_y >= height {
                                return false;
                            }
    
                            let new_seat = new_x + new_y * width;
                            return previous[new_seat as usize] == State::Occupied;
                        }).count();
    
                        if occupy_count >= 4 {
                            new_state = State::Empty;
                        }
                    }
                }
    
                area[seat as usize] = new_state;
            }
        }
        if area == previous {
            break;
        }
        previous = area.clone();
    }
    println!("{}", previous.iter().filter(|a| **a == State::Occupied).count());
}

fn part2(input : Vec<State>, width : i32, height : i32)
{

    // 8 surrounding squares.
    let adjacent = [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1),
    ];

    let mut previous = input.clone();

    loop {
        let mut area = previous.clone();

        for y in 0..height {
            for x in 0..width {
                let seat = x + y * width;
                let state = previous[seat as usize];
                let mut new_state = state;
                match state {
                    State::Floor => { continue; },
                    State::Empty => {
                        let occupied = adjacent.iter().any(|a| {
                            let mut new_x = x;
                            let mut new_y = y;
                            loop {
                                new_x += a.0;
                                new_y += a.1;
                                if new_x < 0 || new_x >= width || new_y < 0 || new_y >= height {
                                    return false;
                                }
        
                                let new_seat = new_x + new_y * width;
                                if previous[new_seat as usize] == State::Occupied {
                                    return true;
                                }
                                if previous[new_seat as usize] == State::Empty {
                                    return false;
                                }
                            }
                        });
    
                        if !occupied {
                            new_state = State::Occupied;
                        }
                    },
                    State::Occupied => {
                        let occupy_count = adjacent.iter().filter(|a| {
                            let mut new_x = x;
                            let mut new_y = y;
                            loop {
                                new_x += a.0;
                                new_y += a.1;
                                if new_x < 0 || new_x >= width || new_y < 0 || new_y >= height {
                                    return false;
                                }
        
                                let new_seat = new_x + new_y * width;
                                if previous[new_seat as usize] == State::Occupied {
                                    return true;
                                }
                                if previous[new_seat as usize] == State::Empty {
                                    return false;
                                }
                            }
                        }).count();
    
                        if occupy_count >= 5 {
                            new_state = State::Empty;
                        }
                    }
                }
    
                area[seat as usize] = new_state;
            }
        }
        if area == previous {
            break;
        }
        print!("{}", termion::clear::All);
        thread::sleep(time::Duration::from_millis(10));
        for y in 0..height {
            for x in 0..width {
                print!("{}", match area[(x + width * y) as usize] {
                    State::Occupied => '#',
                    State::Empty => 'L',
                    State::Floor => '.'
                });
            }
            print!("\n");
        }
        print!("\n");

        previous = area.clone();
    }

    
    println!("{}", previous.iter().filter(|a| **a == State::Occupied).count());
}

fn main() {
    let (input, width, height) = get_input();
    part1(input.clone(), width, height);
    part2(input.clone(), width, height);
}