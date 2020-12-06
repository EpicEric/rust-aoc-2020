use std::{
    iter::Iterator,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

// See "impl Trait" for more information on the return type:
// https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html
fn read_file() -> impl Iterator<Item=String> {
    let path = Path::new("./inputs/day3.txt");
    let file = File::open(&path).expect("couldn't open file");
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("couldn't parse line"))
}

fn get_map() -> Vec<Vec<bool>> {
    read_file()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect()
}

static SLOPE_LIST: [(usize, usize); 5] = [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2),
];

pub fn main () {
    let map = get_map();
    let height = map.len();
    let width = map[0].len();
    let mut results: [usize; 5] = [0; 5];
    for (i, (step_right, step_down)) in SLOPE_LIST.iter().enumerate() {
        let mut tree_count = 0usize;
        let mut x = 0usize;
        let mut y = 0usize;
        loop {
            x = (x + step_right) % width;
            y = y + step_down;
            if y >= height {
                break;
            }
            if map[y][x] {
                tree_count += 1;
            }
        }
        results[i] = tree_count;
        println!("Slope #{}: encountered {} trees", i, tree_count)
    }
    println!("Product of all results: {}", results.iter().product::<usize>())
}
