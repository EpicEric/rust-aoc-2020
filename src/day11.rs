use std::{
    iter::from_fn,
};

#[derive(Debug, Clone, Eq, PartialEq)]
enum WaitingAreaPixel {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

fn get_initial_waiting_area() -> Vec<Vec<WaitingAreaPixel>> {
    super::file::read_file("./inputs/day11.txt")
        .map(|l| -> Vec<WaitingAreaPixel> {
            l.chars().map(|c| -> WaitingAreaPixel {
                match c {
                    'L' => WaitingAreaPixel::EmptySeat,
                    '.' => WaitingAreaPixel::Floor,
                    _ => panic!("Invalid character"),
                }
            }).collect()
        }).collect()
}

// Part 1
fn get_neighbors_iter_part1<'a>(i: usize, j: usize, waiting_area: &'a Vec<Vec<WaitingAreaPixel>>) -> impl Iterator<Item=(usize, usize)> + 'a {
    /* pos:
        1 2 3
        4 _ 5
        6 7 8
    */
    let mut pos = 0usize;
    let iter = from_fn(move || {
        loop {
            pos += 1;
            match pos {
                1 => {
                    if i > 0 && j > 0 {
                        return Some((i - 1, j - 1));
                    }
                }
                2 => {
                    if i > 0 {
                        return Some((i - 1, j));
                    }
                }
                3 => {
                    if i > 0 && j < waiting_area[0].len() - 1 {
                        return Some((i - 1, j + 1));
                    }
                }
                4 => {
                    if j > 0 {
                        return Some((i, j - 1));
                    }
                }
                5 => {
                    if j < waiting_area[0].len() - 1 {
                        return Some((i, j + 1));
                    }
                }
                6 => {
                    if i < waiting_area.len() - 1 &&j > 0 {
                        return Some((i + 1, j - 1));
                    }
                }
                7 => {
                    if i < waiting_area.len() - 1 {
                        return Some((i + 1, j));
                    }
                }
                8 => {
                    if i < waiting_area.len() - 1 && j < waiting_area[0].len() - 1 {
                        return Some((i + 1, j + 1));
                    }
                }
                _ => return None,
            }
        }
    });
    iter
}

// Part 2
fn get_neighbors_iter_part2<'a>(i: usize, j: usize, waiting_area: &'a Vec<Vec<WaitingAreaPixel>>) -> impl Iterator<Item=(usize, usize)> + 'a {
    /* pos:
        1 2 3
        4 _ 5
        6 7 8
    */
    let mut pos = 0usize;
    let iter = from_fn(move || {
        loop {
            pos += 1;
            match pos {
                1 => {
                    let mut i2 = (i as isize) - 1;
                    let mut j2 = (j as isize) - 1;
                    while i2 >= 0 && j2 >= 0 {
                        match waiting_area[i2 as usize][j2 as usize] {
                            WaitingAreaPixel::Floor => {
                                i2 -= 1;
                                j2 -= 1;
                                continue
                            },
                            _ => return Some((i2 as usize, j2 as usize)),
                        }
                    }
                }
                2 => {
                    let mut i2 = (i as isize) - 1;
                    while i2 >= 0 {
                        match waiting_area[i2 as usize][j] {
                            WaitingAreaPixel::Floor => {
                                i2 -= 1;
                                continue
                            },
                            _ => return Some((i2 as usize, j)),
                        }
                    }
                }
                3 => {
                    let mut i2 = (i as isize) - 1;
                    let mut j2 = j + 1;
                    while i2 >= 0 && j2 <= waiting_area[0].len() - 1 {
                        match waiting_area[i2 as usize][j2] {
                            WaitingAreaPixel::Floor => {
                                i2 -= 1;
                                j2 += 1;
                                continue
                            },
                            _ => return Some((i2 as usize, j2)),
                        }
                    }
                }
                4 => {
                    let mut j2 = (j as isize) - 1;
                    while j2 >= 0 {
                        match waiting_area[i][j2 as usize] {
                            WaitingAreaPixel::Floor => {
                                j2 -= 1;
                                continue
                            },
                            _ => return Some((i, j2 as usize)),
                        }
                    }
                }
                5 => {
                    let mut j2 = j + 1;
                    while j2 <= waiting_area[0].len() - 1 {
                        match waiting_area[i][j2] {
                            WaitingAreaPixel::Floor => {
                                j2 += 1;
                                continue
                            },
                            _ => return Some((i, j2)),
                        }
                    }
                }
                6 => {
                    let mut i2 = i + 1;
                    let mut j2 = (j as isize) - 1;
                    while i2 <= waiting_area.len() - 1 && j2 >= 0 {
                        match waiting_area[i2][j2 as usize] {
                            WaitingAreaPixel::Floor => {
                                i2 += 1;
                                j2 -= 1;
                                continue
                            },
                            _ => return Some((i2, j2 as usize)),
                        }
                    }
                }
                7 => {
                    let mut i2 = i + 1;
                    while i2 <= waiting_area.len() - 1 {
                        match waiting_area[i2 as usize][j] {
                            WaitingAreaPixel::Floor => {
                                i2 += 1;
                                continue
                            },
                            _ => return Some((i2 as usize, j)),
                        }
                    }
                }
                8 => {
                    let mut i2 = i + 1;
                    let mut j2 = j + 1;
                    while i2 <= waiting_area.len() - 1 && j2 <= waiting_area[0].len() - 1 {
                        match waiting_area[i2][j2] {
                            WaitingAreaPixel::Floor => {
                                i2 += 1;
                                j2 += 1;
                                continue
                            },
                            _ => return Some((i2, j2)),
                        }
                    }
                }
                _ => return None,
            }
        }
    });
    iter
}

fn iterate_waiting_area(waiting_area: &Vec<Vec<WaitingAreaPixel>>) -> Vec<Vec<WaitingAreaPixel>> {
    let mut new_waiting_area: Vec<Vec<WaitingAreaPixel>> = Vec::with_capacity(waiting_area.len());
    for i in 0..waiting_area.len() {
        let line = &waiting_area[i];
        let mut new_line: Vec<WaitingAreaPixel> = Vec::with_capacity(line.len());
        for j in 0..line.len() {
            let new_value = match &line[j] {
                WaitingAreaPixel::Floor => WaitingAreaPixel::Floor,
                WaitingAreaPixel::EmptySeat => {
                    // let neighbors = get_neighbors_iter_part1(i, j, &waiting_area)
                    let neighbors = get_neighbors_iter_part2(i, j, &waiting_area)
                        .map(|(y, x)| &waiting_area[y][x]);
                    match neighbors.filter(|n| **n == WaitingAreaPixel::OccupiedSeat).count() == 0 {
                        true => WaitingAreaPixel::OccupiedSeat,
                        false => WaitingAreaPixel::EmptySeat,
                    }
                },
                WaitingAreaPixel::OccupiedSeat => {
                    // let neighbors = get_neighbors_iter_part1(i, j, &waiting_area)
                    let neighbors = get_neighbors_iter_part2(i, j, &waiting_area)
                        .map(|(y, x)| &waiting_area[y][x]);
                    // match neighbors.filter(|n| **n == WaitingAreaPixel::OccupiedSeat).count() < 4 {
                    match neighbors.filter(|n| **n == WaitingAreaPixel::OccupiedSeat).count() < 5 {
                        true => WaitingAreaPixel::OccupiedSeat,
                        false => WaitingAreaPixel::EmptySeat,
                    }
                },
            };
            new_line.push(new_value);
        }
        new_waiting_area.push(new_line);
    }
    new_waiting_area
}

fn eq_waiting_areas(first: &Vec<Vec<WaitingAreaPixel>>, second: &Vec<Vec<WaitingAreaPixel>>) -> bool {
    for i in 0..first.len() {
        for j in 0..first[i].len() {
            if first[i][j] != second[i][j] {
                return false
            }
        }
    }
    true
}

pub fn main () {
    let mut curr_area = get_initial_waiting_area();
    let mut iterations = 0usize;
    loop {
        let new_area = iterate_waiting_area(&curr_area);
        if eq_waiting_areas(&curr_area, &new_area) {
            break
        }
        curr_area = new_area;
        iterations += 1;

    }
    let occupied_seats = curr_area.iter().fold(0usize, |acc, l| acc + l.iter().filter(|n| **n == WaitingAreaPixel::OccupiedSeat).count());
    println!("After {} iterations, final waiting area has {} occupied seats.", iterations, occupied_seats)
}
