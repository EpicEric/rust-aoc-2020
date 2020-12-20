use std::{
    collections::HashMap,
};
use regex::Regex;

#[derive(Debug, Clone)]
enum Rule {
    Option(Vec<Rule>),
    Concat(Vec<usize>),
    Char(char),
}

fn get_data() -> (HashMap<usize, Rule>, Vec<String>) {
    let mut lines = super::file::read_file("./inputs/day19.txt");

    // Parse rules
    let mut rules_map: HashMap<usize, Rule> = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        match line.splitn(2, ": ").collect::<Vec<_>>()[..] {
            [key_str, rule_set_str] => {
                let key: usize = key_str.parse().expect("invalid rule number");

                match rule_set_str.chars().collect::<Vec<_>>()[..] {
                    ['"', c, '"'] => {
                        rules_map.insert(key, Rule::Char(c));
                    },
                    [] => panic!("empty rule set"),
                    _ => {
                        let mut rule_vec: Vec<Rule> = rule_set_str.split(" | ").map(|concat| -> Rule {
                            Rule::Concat(concat.split(' ').map(|r| r.parse::<usize>().expect("invalid rule in rule set")).collect())
                        }).collect();
                        if rule_vec.len() == 1 {
                            rules_map.insert(key, rule_vec.pop().unwrap());
                        } else {
                            rules_map.insert(key, Rule::Option(rule_vec));
                        }
                    },
                };
            },
            _ => panic!("invalid rule line"),
        };
    }

    // Part 2 -- Substitute fixed rules
    /*
    // Non-normal rule 8
    rules_map.insert(8, Rule::Option([
        Rule::Concat([42].to_vec()),
        Rule::Concat([42, 8].to_vec()),
    ].to_vec()));
    */
    // Chomsky normalized rule 8
    // ASSUMPTION: "42 => A B | C D | E F | ..."
    let mut rule_8_vec = match rules_map.get(&42).unwrap() {
        Rule::Option(vec) => vec.clone(),
        _ => panic!("rule 42 in different format than expected"),
    };
    rule_8_vec.push(Rule::Concat([42, 8].to_vec()));
    rules_map.insert(8, Rule::Option(rule_8_vec));
    /*
    // Non-normal rule 11
    rules_map.insert(11, Rule::Option([
        Rule::Concat([42, 31].to_vec()),
        Rule::Concat([42, 11, 31].to_vec()),
    ].to_vec()));
    */
    // Chomsky normalized rule 11
    let intermediate_rule = rules_map.keys().max().unwrap() + 100_000;
    rules_map.insert(11, Rule::Option([
        Rule::Concat([42, 31].to_vec()),
        Rule::Concat([42, intermediate_rule].to_vec()),
    ].to_vec()));
    rules_map.insert(intermediate_rule, Rule::Concat([11, 31].to_vec()));

    // Part 2 -- Normalize [Rx -> Ra Ry ; Rz -> Rw Ra ; Ra -> Rb | Rc] into [Rx -> Rb Ry | Rc Ry; Rz -> Rw Rb | Rw Rc]
    // ASSUMPTION: Rules to find equal the format "X => A | B | C | ..." with single rule on every option of the right-side
    let rules_to_normalize_1a: Vec<(usize, usize)> = rules_map.iter().flat_map(|(k, r)| -> Option<Vec<(usize, usize)>> {
        match r {
            Rule::Option(rulesets) => {
                let mut normalization_vec: Vec<(usize, usize)> = Vec::new();
                rulesets.iter().for_each(|rs| {
                    match rs {
                        Rule::Concat(r) => {
                            if r.len() == 1 {
                                normalization_vec.push((*k, r[0]));
                            }
                        },
                        _ => (),
                    }
                });
                Some(normalization_vec)
            },
            _ => None,
        }
    }).flatten().collect();
    let mut rules_to_normalize_1b: HashMap<usize, Vec<usize>> = HashMap::new();
    rules_to_normalize_1a.iter().for_each(|(k, v)| {
        rules_to_normalize_1b.entry(*k).or_insert(Vec::new()).push(*v);
    });
    // println!("{:?}", rules_to_normalize_1b);
    rules_to_normalize_1b.iter().for_each(|(k, v)| {
        rules_map.remove(k);
        let rules_to_change: Vec<_> = rules_map.iter().flat_map(|(x, r)| -> Option<(usize, Rule)> {
            match r {
                Rule::Concat(subrules) => {
                    match &subrules[..] {
                        [a, b] if a == k && b == k => {
                            // ASSUMPTION: Rule to normalize is "X => Y | Z"
                            match &v[..] {
                                [c, d] => {
                                    Some((*x, Rule::Option(vec![
                                        Rule::Concat(vec![*c, *c]),
                                        Rule::Concat(vec![*c, *d]),
                                        Rule::Concat(vec![*d, *c]),
                                        Rule::Concat(vec![*d, *d]),
                                    ])))
                                },
                                _ => panic!("unexpected format for rule in normalization step 1 (Concat)"),
                            }
                        },
                        [a, b] if a == k => {
                            Some((*x, Rule::Option(v.iter().map(|n| Rule::Concat(vec![*n, *b])).collect())))
                        },
                        [a, b] if b == k => {
                            Some((*x, Rule::Option(v.iter().map(|n| Rule::Concat(vec![*a, *n])).collect())))
                        },
                        _ => None,
                    }
                },
                Rule::Option(rulesets) => {
                    let mut option_vec: Vec<Rule> = Vec::new();
                    rulesets.iter().for_each(|rs| {
                        // ASSUMPTION: All option rules only include concat rulesets
                        match rs {
                            Rule::Concat(subrules) => {
                                match &subrules[..] {
                                    [a, b] if a == k && b == k => {
                                        // ASSUMPTION: Rule to normalize is "X => Y | Z"
                                        match &v[..] {
                                            [c, d] => {
                                                option_vec.push(Rule::Concat(vec![*c, *c]));
                                                option_vec.push(Rule::Concat(vec![*c, *d]));
                                                option_vec.push(Rule::Concat(vec![*d, *c]));
                                                option_vec.push(Rule::Concat(vec![*d, *d]));
                                            },
                                            _ => panic!("unexpected format for rule in normalization step 1 (Concat)"),
                                        }
                                    },
                                    [a, b] if a == k => {
                                        v.iter().for_each(|n| option_vec.push(Rule::Concat(vec![*n, *b])));
                                    },
                                    [a, b] if b == k => {
                                        v.iter().for_each(|n| option_vec.push(Rule::Concat(vec![*a, *n])));
                                    },
                                    _ => option_vec.push(Rule::Concat(subrules.clone())),
                                }
                            }
                            _ => panic!("unexpected non-concat rule inside option rule in normalization step 1")
                        }
                    });
                    Some((*x, Rule::Option(option_vec)))
                },
                _ => None,
            }
        }).collect();
        for (x, r) in rules_to_change.iter() {
            rules_map.insert(*x, r.clone());
        }
    });

    // Parse messages
    let messages: Vec<String> = lines.collect();

    // println!("{:?}", rules_map);
    // rules_map.iter().for_each(|(k, v)| {
    //     println!("{} {:?}", k, v);
    // });

    (rules_map, messages)
}

// Part 1
fn get_validation_function_part_1(rules: &HashMap<usize, Rule>) -> impl Fn(&str) -> bool {
    let mut s = String::from("^");
    s += &build_regex_string_for_rule(rules, &0);
    s += "$";
    let re: Regex = Regex::new(&s).expect("invalid rule regex");
    move |message| {
        re.is_match(message)
    }
}

fn build_regex_string_for_rule(rules: &HashMap<usize, Rule>, starting_rule: &usize) -> String {
    match rules.get(starting_rule).expect("cannot find rule") {
        Rule::Char(c) => {
            let mut s = String::with_capacity(1);
            s.push(*c);
            s
        },
        Rule::Concat(subrules) => {
            subrules.iter().map(|r| build_regex_string_for_rule(rules, r)).fold(String::new(), |acc, x| acc + &x )
        },
        Rule::Option(rulesets) => {
            let option = rulesets.iter().map(|rs| {
                match rs {
                    Rule::Concat(subrules) => {
                        subrules.iter().map(|r| build_regex_string_for_rule(rules, r)).fold(String::new(), |acc, x| acc + &x )
                    }
                    _ => panic!("invalid Option rule")
                }
            }).collect::<Vec<_>>().join("|");
            let mut s = String::from("(");
            s += &option;
            s + ")"
        },
    }
}

fn get_validation_function_part_2<'a>(rules: &'a HashMap<usize, Rule>) -> impl Fn(&str) -> bool + 'a {
    let flattened_rules: Vec<(usize, (usize, usize))> = rules.iter().map(|(k, v)| -> Vec<(usize, (usize, usize))> {
        match v {
            Rule::Char(_) => vec![],
            Rule::Concat(subrules) => {
                match &subrules[..] {
                    [first, second] => vec![(*k, (*first, *second))],
                    _ => panic!("non-normalized rule in Concat"),
                }
            },
            Rule::Option(rulesets) => {
                rulesets.iter().map(|rs| {
                    match rs {
                        Rule::Concat(subrules) => {
                            match &subrules[..] {
                                [first, second] => (*k, (*first, *second)),
                                _ => panic!("non-normalized rule in Option<Concat>"),
                            }
                        },
                        _ => panic!("non-concat inside of option")
                    }
                }).collect()
            }
        }
    }).flatten().collect();
    let mut char_rules: HashMap<char, Vec<usize>> = HashMap::new();
    rules.iter().filter_map(|(k, v)| match v { Rule::Char(c) => Some((c, k)), _ => None } ).for_each(|(c, k)| {
        char_rules.entry(*c).or_insert(Vec::new()).push(*k);
    });

    // TODO: This is very slow, although it does find a result in the end.
    move |message| {
        let mut cyk_matrix: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; message.len()]; message.len()];
        // Simply using a hashmap is even slower!!
        // let mut cyk_matrix_2: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

        // Step 1
        for (s, c) in message.chars().enumerate() {
            cyk_matrix[0][s] = char_rules.get(&c).unwrap().clone();
            // cyk_matrix_2.insert((0, s), char_rules.get(&c).unwrap().clone());
        }

        // println!("{:?}", cyk_matrix[0]);

        // Step 2
        let n = message.len();
        for l in 2..n + 1 { // Length of span
            for s in 1..n - l + 2 { // Start of span
                for p in 1..l { // Partition of span
                    let found: Vec<_> = flattened_rules.iter().filter(|(_, (b, c))| {
                        cyk_matrix[p - 1][s - 1].contains(&b) && cyk_matrix[l - p - 1][s + p - 1].contains(&c)
                        // match (cyk_matrix_2.get(&(p - 1, s - 1)), cyk_matrix_2.get(&(l - p - 1, s + p - 1))) {
                        //     (Some(possible_b), Some(possible_c)) => possible_b.contains(&b) && possible_c.contains(&c),
                        //     _ => false,
                        // }
                    }).map(|(a, _)| *a).collect();
                    for i in found.iter() {
                        cyk_matrix[l - 1][s - 1].push(*i);
                        // cyk_matrix_2.entry((l - 1, s - 1)).or_insert(Vec::new()).push(*i);
                    };
                }
            }
            // println!("{:?}", cyk_matrix[l-1]);
        }

        let is_valid = cyk_matrix[message.len() - 1][0].contains(&0);
        // let is_valid = match cyk_matrix_2.get(&(message.len() - 1, 0)) {
        //     Some(x) => x.contains(&0),
        //     _ => false,
        // };
        is_valid
    }
}

pub fn main () {
    let (rules, messages) = get_data();
    // println!("{:?}\n", rules);
    // println!("{:?}", messages);

    // let validator = get_validation_function_part_1(&rules);
    let validator = get_validation_function_part_2(&rules);
    let valid_messages = messages.iter().filter(|m| validator(m)).count();
    println!("Valid messages: {}", valid_messages);
}
