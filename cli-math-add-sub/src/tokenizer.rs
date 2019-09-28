#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
    Value,
    Operator,
}

#[derive(Debug)]
#[derive(PartialEq)]
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
        if current_char.is_whitespace() {
            continue;
        }

        let is_operator_value = is_operator(current_char);

        if !is_operator_value && !current_char.is_numeric() {
            continue;
        }

        if is_operator_value {
            if current_value.len() != 0 {
                let int_token = parse_int_token(&current_value.iter().collect::<String>());

                current_value.clear();

                if int_token.is_some() {
                    matches.push(int_token.unwrap());
                }
            }

            matches.push(create_operator_token(match current_char {
                '+' => OperationType::Addition,
                '-' => OperationType::Subtraction,
                _ => OperationType::None,
            }))
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

    matches
}

fn is_operator(value: char) -> bool {
    match value {
        '+' | '-' => true,
        _ => false,
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
