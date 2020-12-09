use std::{
    collections::{HashSet, HashMap},
    iter::Iterator,
    fs::File,
    io::{self, BufRead},
    path::Path,
};
use regex::Regex;

fn read_file() -> impl Iterator<Item=String> {
    let path = Path::new("./inputs/day7.txt");
    let file = File::open(&path).expect("couldn't open file");
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("couldn't parse line"))
}

fn parse_rule(rule: String) -> (String, Vec<(usize, String)>) {
    // X bags contain Y1 Z1 bags, Y2 Z2 bag.
    //  =>
    // (X, [(Y1, Z1), (Y2, Z2)])
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+) ([a-z]+ [a-z]+) bags?\.?$").expect("invalid parse_rule regex");
    }
    let mut split = rule.splitn(2, " bags contain ");
    let container = split.next().expect("missing container in rule");
    let contents = split.next().expect("missing contents in rule");
    let parsed_contents = contents.split(", ")
        .filter(|item| item.trim() != "no other bags.")
        .map(|item| -> (usize, String) {
            let caps = RE.captures(item).expect("regex did not match line");
            (caps[1].parse().expect("bag count is not int"), caps[2].to_string())
        })
        .collect();
    (String::from(container), parsed_contents)
}

// Part 1
fn get_contained_by_map() -> HashMap<String, Vec<String>> {
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    read_file()
        .map(parse_rule)
        .for_each(|rule| {
            let (container, contents) = rule;
            contents.iter().for_each(|content| {
                let (_, bag) = content;
                match rules.remove(bag) {
                    Some(mut vec) => {
                        vec.push(container.clone());
                        rules.insert(bag.clone(), vec);
                    },
                    _ => {
                        rules.insert(bag.clone(), vec![container.clone()]);
                    },
                }
            });
        });
    rules
}

// Part 1
fn get_colors_containing(wanted_color: String) -> HashSet<String> {
    let mut result: HashSet<String> = HashSet::new();
    let mut analyzed: Vec<String> = Vec::new();
    let mut to_analyze: Vec<String> = vec![wanted_color];
    let map = get_contained_by_map();
    loop {
        if to_analyze.is_empty() {
            break;
        }
        let current_color = to_analyze.pop().expect("to_analyze was empty");
        match map.get(&current_color) {
            None => (),
            Some(colors) => {
                colors.iter().for_each(|c| { result.insert(c.clone()); });
                colors.iter().filter(|c| !analyzed.contains(c)).for_each(|c| to_analyze.push(c.clone()));
            }
        }
        analyzed.push(current_color);
    }
    result
}

// Part 2
fn get_contains_map() -> HashMap<String, Vec<(usize, String)>> {
    let mut rules = HashMap::new();
    read_file()
        .map(parse_rule)
        .for_each(|rule| {
            let (container, contents) = rule;
            rules.insert(container, contents);
        });
    rules
}

// Part 2
fn get_total_bags_inside(wanted_color: &String) -> usize {
    lazy_static! {
        static ref MAP: HashMap<String, Vec<(usize, String)>> = get_contains_map();
    }
    let mut total = 0;
    MAP.get(wanted_color).expect("unknown color in contains_map").iter().for_each(|(count, color)| {
        total += count * (1 + get_total_bags_inside(color))
    });
    total
}

pub fn main () {
    // Part 1
    // let colors = get_colors_containing(String::from("shiny gold"));
    // println!("All colors that contain a shiny gold bag: {}", colors.len());

    // Part 2
    let contained = get_total_bags_inside(&String::from("shiny gold"));
    println!("Bags contained by the gold bag: {}", contained)
}
