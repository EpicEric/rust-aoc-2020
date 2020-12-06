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
    let path = Path::new("./inputs/day4.txt");
    let file = File::open(&path).expect("couldn't open file");
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("couldn't parse line"))
}

static REQUIRED_FIELDS: [&str; 7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
];
static OPTIONAL_FIELDS: [&str; 1] = [
    "cid",
];

fn validate_passport(passport: &Vec<String>) -> bool {
    let mut validated_fields: Vec<&str> = Vec::new();
    for field_description in passport.iter() {
        let mut field_split = field_description.splitn(2, ':');
        let name = field_split.next().expect("badly formed field name");
        let value = field_split.next().expect("badly formed field value");
        // Check that no fields are duplicates
        if validated_fields.contains(&name) {
            return false
        }
        // Check that field is a known field
        if !REQUIRED_FIELDS.contains(&name) && !OPTIONAL_FIELDS.contains(&name) {
            return false;
        }
        // Validate the field
        match name {
            "byr" => { // (Birth Year) - four digits; at least 1920 and at most 2002.
                match value.parse::<usize>() {
                    Err(_) => return false,
                    Ok(year) => {
                        if year < 1920 || year > 2002 {
                            return false
                        }
                    },
                }
            }
            "iyr" => { // (Issue Year) - four digits; at least 2010 and at most 2020.
                match value.parse::<usize>() {
                    Err(_) => return false,
                    Ok(year) => {
                        if year < 2010 || year > 2020 {
                            return false
                        }
                    },
                }
            }
            "eyr" => { // (Expiration Year) - four digits; at least 2020 and at most 2030.
                match value.parse::<usize>() {
                    Err(_) => return false,
                    Ok(year) => {
                        if year < 2020 || year > 2030 {
                            return false
                        }
                    },
                }
            }
            "hgt" => { // (Height) - a number followed by either cm or in:
                       // If cm, the number must be at least 150 and at most 193.
                       // If in, the number must be at least 59 and at most 76.
                let (value_num, unit) = value.split_at(value.len() - 2);
                match value_num.parse::<usize>() {
                    Err(_) => return false,
                    Ok(height) => {
                        match unit {
                            "cm" => {
                                if height < 150 || height > 193 {
                                    return false
                                }
                            },
                            "in" => {
                                if height < 59 || height > 76 {
                                    return false
                                }
                            },
                            _ => return false,
                        }
                    },
                }
            }
            "hcl" => { // (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                if !Regex::new(r"^#[0-9a-f]{6}$").expect("invalid pid regex").is_match(&value) {
                    return false
                }
            }
            "ecl" => { // (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value) {
                    return false
                }
            }
            "pid" => { // (Passport ID) - a nine-digit number, including leading zeroes.
                if !Regex::new(r"^\d{9}$").expect("invalid pid regex").is_match(&value) {
                    return false
                }
            }
            _ => (),
        }
        validated_fields.push(name)
    }
    // Check that required fields exist
    for required_field in REQUIRED_FIELDS.iter() {
        if !validated_fields.contains(required_field) {
            return false;
        }
    }
    true
}

fn get_valid_passports() -> Vec<Vec<String>> {
    let mut valid_passports: Vec<Vec<String>> = Vec::new();
    let mut current_passport: Vec<String> = Vec::new();
    read_file().for_each(|l| {
        if l == "" {
            if validate_passport(&current_passport) {
                // TODO: Find out how to give up current reference instead of copying
                valid_passports.push(current_passport.to_vec())
            }
            current_passport = Vec::new()
        } else {
            l.split(' ').for_each(|f| current_passport.push(String::from(f)))
        }
    });
    // Final passport
    if validate_passport(&current_passport) {
        // TODO: Find out how to give up current reference instead of copying
        valid_passports.push(current_passport.to_vec())
    };
    valid_passports
}

pub fn main () {
    let passports = get_valid_passports();
    println!("Valid passports: {}", passports.len())
}
