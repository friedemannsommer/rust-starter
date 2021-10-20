use std::cmp::Ordering;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum OperationType {
    Addition,
    Subtraction,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Value(i64),
    Operator(OperationType),
}

pub fn tokenize_expression(expression: &str) -> Vec<Token> {
    let mut matches: Vec<Token> = Vec::new();
    let mut iterator = expression.chars().peekable();

    while let Some(current_char) = iterator.next() {
        match current_char {
            '+' => {
                matches.push(Token::Operator(OperationType::Addition));
            }
            '-' => {
                matches.push(Token::Operator(OperationType::Subtraction));
            }
            '0'..='9' => {
                matches.push(parse_int_token(&current_char, &mut iterator).expect("Couldn't parse given digit(s)"));
            }
            _ => {}
        }
    }

    sort_token_list(matches)
}

fn sort_token_list(mut token_list: Vec<Token>) -> Vec<Token> {
    token_list.sort_by(|a, b| {
        match a {
            Token::Value(_) => {
                match b {
                    Token::Operator(_) => Ordering::Less,
                    _ => Ordering::Equal
                }
            }
            Token::Operator(_) => {
                match b {
                    Token::Value(_) => Ordering::Greater,
                    _ => Ordering::Equal
                }
            }
        }
    });

    token_list
}

fn parse_int_token<CharIter: Iterator<Item=char>>(current: &char, iterator: &mut Peekable<CharIter>) -> Option<Token> {
    let mut num = current.to_digit(10)?;

    while let Some(Some(digit)) = iterator.peek().map(|char| char.to_digit(10)) {
        num = num.checked_mul(10)?.checked_add(digit)?;
        iterator.next();
    }

    Some(Token::Value(num as i64))
}

impl Token {
    pub fn get_opt_value(&self) -> Option<i64> {
        match self {
            Token::Value(value) => Some(*value),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_token_composition_1() {
        assert_eq!(
            tokenize_expression("1+2"),
            vec![
                Token::Value(1),
                Token::Value(2),
                Token::Operator(OperationType::Addition),
            ]
        );
    }

    #[test]
    fn test_valid_token_composition_2() {
        assert_eq!(
            tokenize_expression("100+200000"),
            vec![
                Token::Value(100),
                Token::Value(200000),
                Token::Operator(OperationType::Addition),
            ]
        );
    }

    #[test]
    fn test_valid_token_composition_3() {
        assert_eq!(
            tokenize_expression("123456789+0"),
            vec![
                Token::Value(123456789),
                Token::Value(0),
                Token::Operator(OperationType::Addition),
            ]
        );
    }

    #[test]
    fn test_invalid_token_composition_1() {
        assert_eq!(
            tokenize_expression("-100+200000"),
            vec![
                Token::Value(100),
                Token::Value(200000),
                Token::Operator(OperationType::Subtraction),
                Token::Operator(OperationType::Addition),
            ]
        );
    }

    #[test]
    fn test_invalid_token_composition_2() {
        assert_eq!(
            tokenize_expression("-1+0+-"),
            vec![
                Token::Value(1),
                Token::Value(0),
                Token::Operator(OperationType::Subtraction),
                Token::Operator(OperationType::Addition),
                Token::Operator(OperationType::Addition),
                Token::Operator(OperationType::Subtraction),
            ]
        );
    }
}
