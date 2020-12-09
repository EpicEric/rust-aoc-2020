use std::{
    iter::Iterator,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

// See "impl Trait" for more information on the return type:
// https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html
pub fn read_file(filename: &str) -> impl Iterator<Item=String> {
    let path = Path::new(filename);
    let file = File::open(&path).expect("couldn't open file");
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("couldn't parse line"))
}
