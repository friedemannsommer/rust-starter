use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Value,
    Operator,
}

#[derive(Debug, PartialEq)]
pub enum OperationType {
    None,
    Addition,
    Subtraction,
}

#[derive(Debug)]
pub struct Token {
    pub value: i32,
    pub token_type: TokenType,
    pub operation: OperationType,
}

pub fn tokenize_expression(expression: &str) -> Vec<Token> {
    let mut matches: Vec<Token> = Vec::new();
    let mut current_value: Vec<char> = Vec::new();

    for current_char in expression.chars() {
        let operation_type = get_operator_type(current_char);

        if operation_type == OperationType::None && !current_char.is_numeric() {
            continue;
        }

        if operation_type != OperationType::None {
            if current_value.len() != 0 {
                let int_token = parse_int_token(&current_value.iter().collect::<String>());

                current_value.clear();

                if int_token.is_some() {
                    matches.push(int_token.unwrap());
                }
            }

            matches.push(create_operator_token(operation_type));
        } else {
            current_value.push(current_char);
        }
    }

    if current_value.len() != 0 {
        let int_token = parse_int_token(&current_value.iter().collect::<String>());

        if int_token.is_some() {
            matches.push(int_token.unwrap());
        }
    }

    sort_token_list(&mut matches);

    matches
}

fn sort_token_list(token_list: &mut Vec<Token>) {
    token_list.sort_by(|a, b| {
        let a_is_value = a.token_type == TokenType::Value;
        let b_is_value = b.token_type == TokenType::Value;

        if a_is_value && b_is_value {
            return Ordering::Equal;
        } else if a_is_value && !b_is_value {
            return Ordering::Less;
        }

        Ordering::Greater
    });

    let size = token_list.len();
    let mut index: usize = 0;

    while index < size {
        if token_list.get(index).unwrap().token_type == TokenType::Operator {
            break;
        }

        index += 1;
    }

    token_list.get_mut(0..index).unwrap().reverse();
}

fn get_operator_type(value: char) -> OperationType {
    match value {
        '+' => OperationType::Addition,
        '-' => OperationType::Subtraction,
        _ => OperationType::None,
    }
}

fn parse_int_token(str_value: &String) -> Option<Token> {
    let int_value = str_value.parse::<i32>();

    if int_value.is_ok() {
        return Some(Token {
            value: int_value.unwrap(),
            operation: OperationType::None,
            token_type: TokenType::Value,
        });
    } else {
        return None;
    }
}

fn create_operator_token(operator_type: OperationType) -> Token {
    Token {
        operation: operator_type,
        token_type: TokenType::Operator,
        value: 0,
    }
}
