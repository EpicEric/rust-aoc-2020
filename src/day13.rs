use std::{
    iter::from_fn,
    mem
};
use num_integer::lcm;

fn get_bus_schedule() -> (usize, Vec<Option<usize>>) {
    let mut file_iter = super::file::read_file("./inputs/day13.txt");
    let timestamp = file_iter.next().unwrap().parse::<usize>().expect("Invalid timestamp");
    let bus_id_list = file_iter.next().unwrap()
        .split(',')
        .map(|s| {
            match s {
                "x" => None,
                id => Some(id.parse::<usize>().expect("Invalid bus ID")),
            }
        })
        .collect();
    (timestamp, bus_id_list)
}

fn get_departure_time(timestamp: usize, bus_id: usize) -> usize {
    match timestamp % bus_id {
        0 => timestamp,
        x => timestamp + bus_id - x,
    }
}

// Part 1
fn find_smallest_wait_time(schedule: &(usize, Vec<usize>)) -> (usize, usize) {
    let (timestamp, bus_id_list) = schedule;
    let result = bus_id_list.iter().map(|id| (get_departure_time(*timestamp, *id), id)).min_by_key(|x| x.0).expect("");
    (result.0 - timestamp, result.1.clone())
}

// Part 2
fn find_sequential_bus_offsets(schedule: &Vec<Option<usize>>) -> usize {
    find_sequential_bus_offsets_with_initial_offset(schedule, &0)
}

fn find_sequential_bus_offsets_with_initial_offset(schedule: &Vec<Option<usize>>, offset: &usize) -> usize {
    let enumerated_valid_bus_ids: Vec<(usize, usize)> =
        schedule.iter()
            .enumerate()
            .filter(|(_, id)| id.is_some())
            .map(|(i, id)| (i, id.expect("Invalid bus ID")))
            .collect();
    // println!("{:?}", enumerated_valid_bus_ids);
    let (step_pos, mut step) = enumerated_valid_bus_ids.iter().max_by_key(|(_, id)| id).expect("No max bus ID");
    let mut curr_offset = offset - (offset % step) + step - step_pos; // TODO: Improve this? In worst case, it only does one extra check...
    // println!("{}", curr_offset);
    loop {
        // First attempt: Naïve checking. Too slow!
        // if enumerated_valid_bus_ids.iter().all(|(i, id)| (curr_offset + i) % id == 0) {
        //     break
        // } else {
        //     curr_offset += step;
        // }

        // Second attempt: Increase step with Lowest Common Multiple to minimize search field
        let mut found = true;
        for (i, bus_id) in enumerated_valid_bus_ids.iter() {
            if (curr_offset + i) % bus_id == 0 {
                step = lcm(step, *bus_id);
            } else {
                found = false;
                break
            }
        }
        if found {
            break
        } else {
            curr_offset += step;
        }
    }
    curr_offset
}

pub fn main () {
    let schedule = get_bus_schedule();

    // Part 1
    // let (time_to_leave, bus_id) = find_smallest_wait_time(&(schedule.0, schedule.1.iter().flatten().collect()));
    // println!("Bus ID {} leaving in {} minutes", bus_id, time_to_leave);
    // println!(" - Product is {}", bus_id * time_to_leave)

    // Part 2
    // let offset = find_sequential_bus_offsets(&schedule.1);
    let offset = find_sequential_bus_offsets_with_initial_offset(&schedule.1, &100000000000000);
    println!("Found offset: {}", offset)
}
