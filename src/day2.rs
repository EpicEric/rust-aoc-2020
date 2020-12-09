use std::{
    iter::Iterator,
};
use regex::Regex;

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
    super::file::read_file("./inputs/day2.txt")
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
