use std::{
    iter::Iterator,
};

fn to_seat_id(boarding_pass: String) -> usize {
    usize::from_str_radix(
        &boarding_pass
            .replace("F", "0")
            .replace("B", "1")
            .replace("L", "0")
            .replace("R", "1"),
        2).unwrap()
}

struct Seat {
    id: usize
}

impl Seat {
    fn row(&self) -> usize {
        &self.id >> 3
    }

    fn column(&self) -> usize {
        &self.id % 8
    }
}

impl From<String> for Seat {
    fn from(item: String) -> Self {
        Self { id: to_seat_id(item) }
    }
}

fn get_seats() -> impl Iterator<Item=Seat> {
    super::file::read_file("./inputs/day5.txt")
        .map(Seat::from)
}

pub fn main () {
    let mut seats: Vec<Seat> = get_seats().collect();
    seats.sort_by_key(|s| s.id);
    println!("Min seat ID: {}", seats.first().unwrap().id);
    println!("Max seat ID: {}", seats.last().unwrap().id);
    // for seat in seats {
    //     println!("Row {} column {} seat ID {}", seat.row(), seat.column(), seat.id)
    // }
    for i in 1..seats.len() {
        if seats[i].id - seats[i - 1].id > 1 {
            println!("Missing seat(s) between {} and {}!", seats[i - 1].id, seats[i].id)
        } 
    }
}
