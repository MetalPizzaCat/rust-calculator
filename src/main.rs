use regex::Error;
use std::{collections::HashMap, fmt::Display, hash};

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Eq, Hash)]
#[repr(u8)]
enum OperandType {
    Number,
    Add,
    Sub,
    Mul,
    Div,
    Open,
    Close,
}

struct Token {
    operand_type: OperandType,
    value: f32,
}

impl Token {
    pub fn new(op_type: OperandType, value: f32) -> Self {
        Self {
            operand_type: op_type,
            value,
        }
    }
}

fn calculate(tokens: &Vec<Token>) -> Option<f32> {
    let mut numbers: Vec<f32> = Vec::new();
    let mut counter: usize = 0;
    let mut result: f32 = 0.0;
    for token in tokens {
        match token.operand_type {
            OperandType::Add => {
                let a: f32 = numbers.pop().unwrap();
                let b: f32 = numbers.pop().unwrap();
                numbers.push(b + a);
            }
            OperandType::Div => {
                let a: f32 = numbers.pop().unwrap();
                let b: f32 = numbers.pop().unwrap();
                numbers.push(b / a);
            }
            OperandType::Mul => {
                let a: f32 = numbers.pop().unwrap();
                let b: f32 = numbers.pop().unwrap();
                numbers.push(b * a);
            }
            OperandType::Sub => {
                let a: f32 = numbers.pop().unwrap();
                let b: f32 = numbers.pop().unwrap();
                numbers.push(b - a);
            }
            OperandType::Number => {
                numbers.push(token.value);
            }
            _ => {}
        }
    }
    numbers.pop()
}

macro_rules! map {
    ($(($k:expr , $v:expr)),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
}

fn parse(line: String) -> Result<Vec<Token>, Error> {
    //simple list for converting symbols to operations
    let operations: HashMap<&str, OperandType> = map! {
        ("+" ,  OperandType::Add    ),
        ("-",   OperandType::Sub    ),
        ("*",   OperandType::Mul    ),
        ("/",   OperandType::Div    ),
        ("(",   OperandType::Open   ),
        (")",   OperandType::Close  ),
    };

    //operation priority list
    let priorities: HashMap<OperandType, u8> = map! {
        (OperandType::Add,  0),
        (OperandType::Sub,  0),
        (OperandType::Mul,  1),
        (OperandType::Div,  1),
        (OperandType::Open,  2),
        (OperandType::Close,2),
    };

    use regex::Regex;
    let mut stack: Vec<OperandType> = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();
    //This regex matches either any floating point number
    //(([0-9])+(\.[0-9]+)?)
    //(\.[0-9]+)?) this is for optional usage of dot
    //or any of the operation symbols
    //(\+|\-|\*|/|\(|\)))
    let reg_ex = Regex::new(r"((([0-9])+(\.[0-9]+)?)|(\+|\-|\*|/|\(|\)))")?;

    let matches = reg_ex.find_iter(line.as_str());
    'march: for token in matches {
        let val = token.as_str();
        if let Ok(num) = token.as_str().parse::<f32>() {
            tokens.push(Token::new(OperandType::Number, num));
            continue 'march;
        }
        match token.as_str() {
            ")" => {
                while let Some(operation) = stack.pop() {
                    if matches!(operation, OperandType::Open) {
                        let i = 0;
                        break;
                    }
                    tokens.push(Token::new(operation, 0.0));
                }
            }
            "(" => stack.push(OperandType::Open),
            _ => {
                let priority: u8 = priorities[&operations[token.as_str()]];
                while let Some(operation) = stack.pop() {
                    if matches!(operation, OperandType::Open) || priorities[&operation] < priority {
                        stack.push(operation);
                        break;
                    }
                    tokens.push(Token::new(operation, 0.0));
                }
                stack.push(operations[token.as_str()]);
            }
        }
        let u = 0;
    }
    Ok(tokens)
}
fn main() {
    match parse("(4 *(6 - 3) + ( 8 - 6)/2)".to_owned()) {
        Ok(compute_tree) => {
            if let Some(result) = calculate(&compute_tree) {
                println!("Result : {}", result);
            }
        }
        Err(error) => {
            println!("{}", error.to_string())
        }
    }
}
