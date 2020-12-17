use std::{
    collections::HashMap,
};
use regex::Regex;
// use permutohedron::{
//     heap_recursive,
//     control::Control,
// };

#[derive(Debug, Clone)]
struct Field {
    name: String,
    possible_ranges: [(usize, usize); 2],
}

fn parse_data() -> (Vec<Field>, Vec<usize>, Vec<Vec<usize>>) {
    let mut lines = super::file::read_file("./inputs/day16.txt");
    
    // Parse fields
    let mut fields: Vec<Field> = Vec::new();
    loop {
        match &lines.next().unwrap()[..] {
            "" => break,
            line => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").expect("invalid field regex");
                }
                let caps = RE.captures(&line).expect("invalid field");
                let name = caps[1].to_string();
                let first_range = (caps[2].parse::<usize>().unwrap(), caps[3].parse::<usize>().unwrap());
                let second_range = (caps[4].parse::<usize>().unwrap(), caps[5].parse::<usize>().unwrap());
                fields.push(Field {
                    name: name,
                    possible_ranges: [first_range, second_range],
                });
            },
        }
    }

    // Parse own ticket
    assert_eq!(&lines.next().unwrap()[..], "your ticket:");
    let own_ticket: Vec<usize> = lines.next().unwrap().split(',').map(|v| v.parse::<usize>().unwrap()).collect();
    assert_eq!(&lines.next().unwrap()[..], "");
    
    // Parse nearby tickets
    assert_eq!(&lines.next().unwrap()[..], "nearby tickets:");
    let mut nearby_tickets: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        nearby_tickets.push(line.split(',').map(|v| v.parse::<usize>().unwrap()).collect());
    }

    (fields, own_ticket, nearby_tickets)
}


// Part 1
fn get_error_rate(fields: &Vec<Field>, tickets: &Vec<Vec<usize>>) -> usize {
    let valid_ranges: Vec<&(usize, usize)> = fields.iter().flat_map(|f| &f.possible_ranges).collect();
    let is_valid_value = |value: usize| -> bool {
        valid_ranges.iter().any(|(min, max)| value >= *min && value <= *max)
    };
    tickets.iter().flatten().filter(|v| !is_valid_value(**v)).sum()
}

// Part 2
fn get_valid_tickets(fields: &Vec<Field>, tickets: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let valid_ranges: Vec<&(usize, usize)> = fields.iter().flat_map(|f| &f.possible_ranges).collect();
    let is_valid_value = |value: usize| -> bool {
        valid_ranges.iter().any(|(min, max)| value >= *min && value <= *max)
    };
    tickets.iter().filter(|t| !t.iter().any(|v| !is_valid_value(*v))).map(|t| t.clone()).collect()
}

// fn get_ranges_for_columns(matrix: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
//     let mut column_ranges = Vec::with_capacity(matrix[0].len());
//     for i in 0..matrix[0].len() {
//         let mut iter = matrix.iter();
//         let first = iter.next().unwrap();
//         let mut min = first[i];
//         let mut max = first[i];
//         for line in iter {
//             if line[i] < min {
//                 min = line[i]
//             }
//             if line[i] > max {
//                 max = line[i]
//             }
//         }
//         column_ranges.push((min, max));
//     }
//     column_ranges
// }

fn get_correct_field_order<'a>(fields: &'a Vec<Field>, tickets: &Vec<Vec<usize>>) -> Vec<&'a Field> {
    // First attempt: Too slow!
    // let mut fields_copy = fields.clone();
    // let column_ranges = get_ranges_for_columns(tickets);
    // let mut found: Option<Vec<Field>> = None;
    // heap_recursive(&mut fields_copy, |permutation| {
    //     if column_ranges.iter().enumerate().all(|(i, range)| -> bool {
    //         let (first_min, first_max) = permutation[i].possible_ranges[0];
    //         let (second_min, second_max) = permutation[i].possible_ranges[1];
    //         (range.0 >= first_min && range.1 <= first_max) || (range.0 >= second_min && range.1 <= second_max)
    //     }) {
    //         found = Some(permutation.to_vec());
    //         return Control::Break(())
    //     }
    //     Control::Continue
    // });
    // return found.expect("couldn't find valid field order");

    let mut possible_fields_per_column: Vec<Vec<&Field>> = tickets[0].iter().map(|_| fields.iter().collect()).collect();
    for ticket in tickets.iter() {
        for (i, value) in ticket.iter().enumerate() {
            possible_fields_per_column[i] = possible_fields_per_column[i].iter().filter(|field| -> bool {
                let (first_min, first_max) = field.possible_ranges[0];
                let (second_min, second_max) = field.possible_ranges[1];
                (*value >= first_min && *value <= first_max) || (*value >= second_min && *value <= second_max)
            }).map(|field| *field).collect()
        }
    }
    // println!("{:?}", possible_fields_per_column.iter().map(|f| f.len()).collect::<Vec<_>>());
    while possible_fields_per_column.iter().any(|c| c.len() > 1) {
        let clearable_fields: Vec<(usize, &Field)> = possible_fields_per_column.iter().enumerate().filter(|(_, c)| c.len() == 1).map(|(i, c)| (i, c[0])).collect();
        for (i, field) in clearable_fields {
            for j in 0..possible_fields_per_column.len() {
                if j != i {
                    possible_fields_per_column[j].retain(|&f| f.name != field.name);
                }
            }
        }
    }
    // println!("{:?}", possible_fields_per_column);
    let correct_fields: Vec<&Field> = possible_fields_per_column.iter().flatten().map(|f| f.clone()).collect();
    assert_eq!(correct_fields.len(), fields.len());
    correct_fields
}

pub fn main () {
    let data = parse_data();
    // println!("{:?}", data.0);
    // println!("{:?}", data.1);
    // println!("{:?}", data.2);

    // Part 1
    // println!("Error rate: {}", get_error_rate(&data.0, &data.2));

    // Part 2
    let valid_tickets = get_valid_tickets(&data.0, &data.2);
    let fields = get_correct_field_order(&data.0, &valid_tickets);
    // println!("Field order: {:?}", fields.iter().map(|f| &f.name).collect::<Vec<_>>());
    let departure_field_indexes: Vec<usize> = fields.iter().enumerate().filter(|(_, field)| field.name.starts_with("departure")).map(|(i, _)| i).collect();
    assert_eq!(departure_field_indexes.len(), 6);
    let departure_product: usize = departure_field_indexes.iter().map(|i| data.1[*i]).product();
    println!("Departure fields product: {}", departure_product)
}
