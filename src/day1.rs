use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_file() -> Vec<String> {
    let path = Path::new("./inputs/day1.txt");
    let file = File::open(&path).expect("couldn't open file");
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("couldn't parse line"))
        .collect()
}

fn get_data() -> Vec<i32> {
    read_file()
        .iter().map(|l| l.parse::<i32>().expect("line is not an int"))
        .collect()
}

pub fn main() {
    let list = get_data();
    // for j in 0..(list.len()-1) {
    //     let second = list[j];
    //     let third = 2020 - second;
    //     for k in j+1..(list.len()) {
    //         if list[k] == third {
    //             println!("{} * {} = {}", second, third, second * third);
    //             return;
    //         }
    //     }
    // }
    for i in 0..(list.len()-2) {
        let first = list[i];
        for j in i..(list.len()-1) {
            let second = list[j];
            let third = 2020 - first - second;
            for k in j+1..(list.len()) {
                if list[k] == third {
                    println!("{} * {} * {} = {}", first, second, third, first * second * third);
                    return;
                }
            }
        }
    }
    println!("Not found!")
}

