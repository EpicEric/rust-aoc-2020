#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Num(usize),
    OpAdd,
    OpMul,
    ParensOpen,
    ParensClose,
}

fn tokenize_line(line: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let iter = line.chars();
    let mut curr_num_token = String::new();
    for c in iter {
        match c {
            digit @ '0'..='9' => {
                curr_num_token.push(digit);
            }
            '+' => {
                if !curr_num_token.is_empty() {
                    tokens.push(Token::Num(curr_num_token.parse().expect("invalid num token before +")));
                    curr_num_token.clear();
                }
                tokens.push(Token::OpAdd);
            },
            '*' => {
                if !curr_num_token.is_empty() {
                    tokens.push(Token::Num(curr_num_token.parse().expect("invalid num token before *")));
                    curr_num_token.clear();
                }
                tokens.push(Token::OpMul);
            },
            '(' => {
                if !curr_num_token.is_empty() {
                    panic!("unexpected (");
                }
                tokens.push(Token::ParensOpen);
            },
            ')' => {
                if !curr_num_token.is_empty() {
                    tokens.push(Token::Num(curr_num_token.parse().expect("invalid num token before )")));
                    curr_num_token.clear();
                }
                tokens.push(Token::ParensClose);
            },
            ' ' => {
                if !curr_num_token.is_empty() {
                    tokens.push(Token::Num(curr_num_token.parse().expect("invalid num token before space")));
                    curr_num_token.clear();
                }
            },
            _ => panic!("invalid character"),
        }
    };
    if !curr_num_token.is_empty() {
        tokens.push(Token::Num(curr_num_token.parse().expect("invalid num token before space")));
    }
    tokens
}

fn evaluate_expression(expression: &Vec<Token>) -> usize {
    let mut stack: Vec<Token> = Vec::new();

    for token_iter in expression.iter() {
        let mut token = *token_iter;
        loop {
            let peek = stack.pop();
            match peek {
                Some(t) => stack.push(t),
                None => (),
            };
            match (token, peek) {

                // Part 1
                // (Token::Num(second), Some(Token::OpAdd)) | (Token::Num(second), Some(Token::OpMul)) => {
                //     let op = stack.pop().unwrap();
                //     let first_tok = stack.pop().unwrap();
                //     match (first_tok, op) {
                //         (Token::Num(first), Token::OpAdd) => stack.push(Token::Num(first + second)),
                //         (Token::Num(first), Token::OpMul) => stack.push(Token::Num(first * second)),
                //         (_, Token::OpAdd) => panic!("cannot add non-numerical values"),
                //         (_, Token::OpMul) => panic!("cannot multiply non-numerical values"),
                //         _ => panic!("impossible condition (operator disappeared from top of stack)"),
                //     };
                //     break;
                // },

                // Part 2
                (Token::Num(second), Some(Token::OpAdd)) => {
                    stack.pop(); // OpAdd
                    let first_tok = stack.pop().unwrap();
                    match first_tok {
                        Token::Num(first) => stack.push(Token::Num(first + second)),
                        _ => panic!("cannot add non-numerical values"),
                    };
                    break;
                },

                (Token::Num(_), _) | (Token::OpAdd, _) | (Token::OpMul, _) | (Token::ParensOpen, _) => {
                    stack.push(token);
                    break;
                }
                (Token::ParensClose, t) => {
                    // Part 1
                    // token = t.expect("no value before )");
                    // stack.pop();
                    // assert_eq!(stack.pop(), Some(Token::ParensOpen));
                    
                    // Part 2
                    token = t.expect("no value before )");
                    stack.pop();
                    loop {
                        match stack.pop() {
                            Some(Token::OpMul) => {
                                match (token, stack.pop()) {
                                    (Token::Num(second), Some(Token::Num(first))) => {
                                        token = Token::Num(first * second);
                                    },
                                    _ => panic!("cannot multiply non-numerical values")
                                }
                            },
                            Some(Token::ParensOpen) => break,
                            _ => panic!("unmatched parens")
                        }
                    }
                },
            }
        }
    }

    loop {
        match &stack.clone()[..] {
            // Only the final value must be remaining in the stack
            [Token::Num(result)] => return *result,
    
            // Part 2 -- doesn't affect part 1
            [.., Token::Num(first), Token::OpMul, Token::Num(second)] => {
                stack.pop();
                stack.pop();
                stack.pop();
                stack.push(Token::Num(first * second));
            },
    
            _ => panic!("failed to evaluate expression")
        };
    }
}

pub fn main () {
    let expressions = super::file::read_file("./inputs/day18.txt").map(tokenize_line);
    // for e in expressions {
    //     // println!("{:?}", e);
    //     println!("{}", evaluate_expression(&e));
    // };
    let sum = expressions.map(|e| evaluate_expression(&e)).sum::<usize>();
    println!("Sum of expressions: {}", sum);
}
