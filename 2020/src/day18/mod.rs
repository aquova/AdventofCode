use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Operator {
    Addition,
    Multiplication,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Token {
    Number(u64),
    LeftParen,
    RightParen,
    Op(Operator),
}

// Custom print for enum, for debug purposes
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sym = match self {
            Token::Number(n) => format!("{}", n),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::Op(op) => {
                match op {
                    Operator::Addition => "+".to_string(),
                    Operator::Multiplication => "*".to_string(),
                }
            }
        };
        write!(f, "{}", sym)
    }
}

pub fn day18p1() -> Option<u64> {
    let input = include_str!("input.txt").lines();

    let mut sum = 0;
    for line in input {
        let tokens = tokenize(line);
        sum += eval(0, &tokens).1;
    }

    Some(sum)
}

pub fn day18p2() -> Option<u64> {
    let input = include_str!("input.txt").lines();

    let mut sum = 0;
    for line in input {
        let mut tokens = tokenize(line);
        apply_parens(&mut tokens);
        // for t in tokens.iter() {
        //     print!("{}", t);
        // }
        // println!();
        sum += eval(0, &tokens).1;
    }

    Some(sum)
}

fn tokenize(line: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    // Assumes all tokens are one character
    for c in line.chars() {
        if c.is_numeric() {
            tokens.push(Token::Number(c.to_digit(10).unwrap() as u64));
        } else {
            match c {
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '+' => tokens.push(Token::Op(Operator::Addition)),
                '*' => tokens.push(Token::Op(Operator::Multiplication)),
                _ => ()
            }
        }
    }

    tokens
}

fn eval(mut i: usize, tokens: &[Token]) -> (usize, u64) {
    let mut curr_num: Option<u64> = None;
    let mut curr_op: Option<Operator> = None;
    while i < tokens.len() {
        match tokens[i] {
            Token::LeftParen => {
                let recur = eval(i + 1, tokens);
                i = recur.0;
                if let Some(op) = curr_op {
                    let left = curr_num.unwrap();
                    match op {
                        Operator::Addition => curr_num = Some(left + recur.1),
                        Operator::Multiplication => curr_num = Some(left * recur.1),
                    }
                } else {
                    curr_num = Some(recur.1);
                }
            },
            Token::RightParen => {
                return (i, curr_num.unwrap());
            },
            Token::Op(op) => {
                curr_op = Some(op);
            },
            Token::Number(n) => {
                if let Some(op) = curr_op {
                    let left = curr_num.unwrap();
                    match op {
                        Operator::Addition => curr_num = Some(left + n),
                        Operator::Multiplication => curr_num = Some(left * n),
                    }
                } else {
                    curr_num = Some(n);
                }
            },
        }
        i += 1;
    }
    (i, curr_num.unwrap())
}

fn apply_parens(tokens: &mut Vec<Token>) {
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == Token::Op(Operator::Addition) {
            // Find where to put ( on the left
            if let Token::Number(_) = tokens[i - 1] {
                tokens.insert(i - 1, Token::LeftParen);
                i += 1;
            } else if tokens[i - 1] == Token::RightParen { // Just else?
                let mut right_parens = 0;
                for j in (0..i).rev() {
                    if tokens[j] == Token::LeftParen {
                        right_parens -= 1;
                        if right_parens == 0 {
                            tokens.insert(j, Token::LeftParen);
                            i += 1;
                            break;
                        }
                    } else if tokens[j] == Token::RightParen {
                        right_parens += 1;
                    }
                }
            }

            // Find where to put ) on the right
            if let Token::Number(_) = tokens[i + 1] {
                tokens.insert(i + 2, Token::RightParen);
            } else if tokens[i + 1] == Token::LeftParen { // Just else?
                let mut left_parens = 0;
                for j in (i + 1)..tokens.len() {
                    if tokens[j] == Token::RightParen {
                        left_parens -= 1;
                        if left_parens == 0 {
                            tokens.insert(j + 1, Token::RightParen);
                            break;
                        }
                    } else if tokens[j] == Token::LeftParen {
                        left_parens += 1;
                    }
                }
            }
        }
        i += 1;
    }
}
