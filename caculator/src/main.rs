use std::io::{self, Write};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

fn parse_number(chars: &mut Peekable<Chars>) -> f64 {
    let mut number = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_digit(10) {
            number.push(c);
            chars.next(); // consume
        } else {
            break;
        }
    }
    number.parse().unwrap()
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    println!("Input strings: {:?}", input);
    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => tokens.push(Token::Number(parse_number(&mut chars))),
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Multiply);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Divide);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LeftParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RightParen);
            }
            ' ' => {
                chars.next();
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }
    tokens
}

fn precedence(token: &Token) -> u8 {
    // 在 `match` 表达式中，对引用进行模式匹配是很常见的做法，Rust 会自动解引用。
    match token {
        Token::Plus | Token::Minus => 1,
        Token::Multiply | Token::Divide => 2,
        _ => 0,
    }
}

fn apply_operator(numbers: &mut Vec<f64>, op: &Token) {
    let b = numbers.pop().unwrap();
    let a = numbers.pop().unwrap();
    let result = match op {
        Token::Plus => a + b,
        Token::Minus => a - b,
        Token::Divide => a / b,
        Token::Multiply => a * b,
        _ => panic!("Unexpected operator"),
    };
    numbers.push(result);
}

fn evaluate(tokens: &[Token]) -> f64 {
    let mut numbers = Vec::new();
    let mut operators: Vec<&Token> = Vec::new();
    // let mut operators = Vec::new();

    for token in tokens {
        match token {
            Token::Number(num) => numbers.push(*num),
            Token::Plus | Token::Minus | Token::Divide | Token::Multiply => {
                while let Some(op) = operators.last() {
                    if precedence(op) >= precedence(token) {
                        apply_operator(&mut numbers, operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(token);
            }
            Token::LeftParen => operators.push(token),
            Token::RightParen => {
                while let Some(op) = operators.pop() {
                    if let Token::LeftParen = op {
                        break;
                    }
                    apply_operator(&mut numbers, op);
                }
            }
        }
    }
    while let Some(op) = operators.pop() {
        apply_operator(&mut numbers, op);
    }
    numbers.pop().unwrap()
}

fn main() {
    println!("Welcome to simple caculator.");
    // let input = "3 + 4 * 2 / (6 - 5) * 2 + 3";
    let input = "11 * 11";
    let tokens = tokenize(input);
    println!("Tokens: {:?}", tokens);
    let result = evaluate(&tokens);
    println!("Result: {}", result);

    // loop {
    //     println!("Please enter your expression: ");
    //     let mut chars = String::new();
    //     io::stdin()
    //         .read_line(&mut chars)
    //         .expect("Failed to read input.");
    //     // `read_line()` 方法会保留输入中的换行符，这就是为什么我们经常需要使用 `trim()`。
    //     let tokens = tokenize(chars.trim());
    //     println!("Input Tokens: {:?}", tokens);
    //     let result = evaluate(&tokens);
    //     println!("Result: {}", result);
    // }
}
