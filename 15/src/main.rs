use std::collections::HashMap;

#[allow(non_upper_case_globals)]
const input : [i32; 6] = [1,0,15,2,10,13];


fn part1(target_count : i32) {
    let mut seen = HashMap::<i32, (i32, i32, i32)>::new();

    let mut last_number = 0;
    let mut turn_counter = 0;
    for starting_number in &input {
        turn_counter += 1;
        seen.insert(*starting_number, (1, turn_counter, turn_counter));
        last_number = *starting_number;
    }

    while turn_counter < target_count {
        turn_counter += 1;
        let (count, index, index_prev) = seen.get(&last_number).unwrap();
        let new_value =
            if *count == 1 {
                0
            }
            else {
                index - index_prev
            };

        if let Some((old_count, last_seen, _)) = seen.get(&new_value) {
            seen.insert(new_value, (old_count + 1, turn_counter, *last_seen));
        }
        else {
            seen.insert(new_value, (1, turn_counter, turn_counter));
        }
        last_number = new_value;
    }
    println!("{}", last_number);
}

fn main() {
    part1(2020);
    part1(30000000);
}
