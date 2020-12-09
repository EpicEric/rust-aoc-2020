
fn get_data() -> Vec<i32> {
    super::file::read_file("./inputs/day1.txt").collect::<Vec<String>>()
        .iter().map(|l| l.parse::<i32>().expect("line is not an int"))
        .collect()
}

pub fn main() {
    let list = get_data();
    // Part 1
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
    // Part 2
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

