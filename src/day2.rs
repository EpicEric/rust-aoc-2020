use std::{
    iter::Iterator,
    fs::File,
    io::{self, BufRead},
    path::Path,
};
use regex::Regex;

// See "impl Trait" for more information on the return type:
// https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html
fn read_file() -> impl Iterator<Item=String> {
    let path = Path::new("./inputs/day2.txt");
    let file = File::open(&path).expect("couldn't open file");
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("couldn't parse line"))
}

#[derive(Debug)]
struct PasswordValidation {
    password: String,
    policy_char: char,
    policy_first: usize,
    policy_second: usize,
}

impl From<String> for PasswordValidation {
    fn from(item: String) -> Self {
        let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").expect("invalid regex");
        let caps = re.captures(&item).expect("regex did not match line");
        PasswordValidation {
            password: caps[4].to_string(),
            policy_char: caps[3].chars().nth(0).expect("policy_char is missing"),
            policy_first: caps[1].parse().expect("policy_first is not an int"),
            policy_second: caps[2].parse().expect("policy_second is not an int"),
        }
    }
}

fn get_data() -> impl Iterator<Item=PasswordValidation> {
    read_file()
        .map(|l| PasswordValidation::from(l))
}

pub fn main () {
    let passwords = get_data();
    let valid_passwords = passwords.filter(|pass| {
        // let char_count = pass.password.as_str().chars().filter(|c| c == &pass.policy_char).count();
        // (char_count >= pass.policy_first) && (char_count <= pass.policy_second)
        let get_char_from_password = |policy_pos: usize| -> char { pass.password.as_str().chars().nth(policy_pos - 1).expect("policy_pos is out of range") };
        (get_char_from_password(pass.policy_first) == pass.policy_char) ^ (get_char_from_password(pass.policy_second) == pass.policy_char)
    });
    println!("Valid passwords: {}", valid_passwords.count())
}
