use crate::tokenizer::{OperationType, Token, TokenType};

pub fn process_token_list(token_list: &[Token]) -> Option<i32> {
    let size = token_list.len();

    if size == 0 {
        return Some(0);
    }

    let mut index = 0;
    let mut result = 0;
    let mut token_index = 0;
    let mut value_offset = 0;
    let mut first_token = true;

    while index < size {
        let token = &token_list[index];

        if token.token_type == TokenType::Operator {
            if first_token {
                // this is the first found "operator" token
                value_offset = 0;
                token_index = index;
            }

            if value_offset + 1 < token_index {
                let opt_value = if first_token {
                    // the first token requires two values from the "token_list"
                    process_operation(
                        &token.operation,
                        token_list[value_offset].value,
                        token_list[value_offset + 1].value,
                    )
                } else {
                    // every other token after the first one only needs one value from the "token_list"
                    process_operation(&token.operation, result, token_list[value_offset + 1].value)
                };

                if let Some(value) = opt_value {
                    result = value;
                    value_offset += 1;
                } else {
                    // most likely a i32 overflow
                    return None;
                }
            } else {
                // invalid expression
                return None;
            }

            // all tokens after the current aren't the first
            first_token = false;
        }

        index += 1;
    }

    Some(result)
}

fn process_operation(operator: &OperationType, value_a: i32, value_b: i32) -> Option<i32> {
    match operator {
        OperationType::Addition => value_a.checked_add(value_b),
        OperationType::Subtraction => value_a.checked_sub(value_b),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_add_tokens() {
        assert_eq!(
            process_token_list(&vec![
                Token {
                    value: 1,
                    token_type: TokenType::Value,
                    operation: OperationType::None,
                },
                Token {
                    value: 1,
                    token_type: TokenType::Value,
                    operation: OperationType::None,
                },
                Token {
                    value: 0,
                    token_type: TokenType::Operator,
                    operation: OperationType::Addition,
                },
            ]),
            Some(2)
        )
    }

    #[test]
    fn test_valid_sub_tokens() {
        assert_eq!(
            process_token_list(&vec![
                Token {
                    value: 1,
                    token_type: TokenType::Value,
                    operation: OperationType::None,
                },
                Token {
                    value: 1,
                    token_type: TokenType::Value,
                    operation: OperationType::None,
                },
                Token {
                    value: 0,
                    token_type: TokenType::Operator,
                    operation: OperationType::Subtraction,
                },
            ]),
            Some(0)
        )
    }

    #[test]
    fn test_valid_add_sub_tokens() {
        assert_eq!(
            process_token_list(&vec![
                Token {
                    value: 1,
                    token_type: TokenType::Value,
                    operation: OperationType::None,
                },
                Token {
                    value: 1,
                    token_type: TokenType::Value,
                    operation: OperationType::None,
                },
                Token {
                    value: 2,
                    token_type: TokenType::Value,
                    operation: OperationType::None,
                },
                Token {
                    value: 0,
                    token_type: TokenType::Operator,
                    operation: OperationType::Addition,
                },
                Token {
                    value: 0,
                    token_type: TokenType::Operator,
                    operation: OperationType::Subtraction,
                },
            ]),
            Some(0)
        )
    }

    #[test]
    fn test_valid_sub_add_tokens() {
        assert_eq!(
            process_token_list(&vec![
                Token {
                    value: 1,
                    token_type: TokenType::Value,
                    operation: OperationType::None,
                },
                Token {
                    value: 1,
                    token_type: TokenType::Value,
                    operation: OperationType::None,
                },
                Token {
                    value: 2,
                    token_type: TokenType::Value,
                    operation: OperationType::None,
                },
                Token {
                    value: 0,
                    token_type: TokenType::Operator,
                    operation: OperationType::Subtraction,
                },
                Token {
                    value: 0,
                    token_type: TokenType::Operator,
                    operation: OperationType::Addition,
                },
            ]),
            Some(2)
        )
    }

    #[test]
    fn test_invalid_add_tokens() {
        assert_eq!(
            process_token_list(&vec![
                Token {
                    value: 1,
                    token_type: TokenType::Value,
                    operation: OperationType::None,
                },
                Token {
                    value: 0,
                    token_type: TokenType::Operator,
                    operation: OperationType::Addition,
                },
            ]),
            None
        )
    }

    #[test]
    fn test_invalid_sub_tokens() {
        assert_eq!(
            process_token_list(&vec![
                Token {
                    value: 1,
                    token_type: TokenType::Value,
                    operation: OperationType::None,
                },
                Token {
                    value: 0,
                    token_type: TokenType::Operator,
                    operation: OperationType::Subtraction,
                },
            ]),
            None
        )
    }

    #[test]
    fn test_valid_add_operator() {
        assert_eq!(process_operation(&OperationType::Addition, 0, 1), Some(1))
    }

    #[test]
    fn test_valid_sub_operator() {
        assert_eq!(
            process_operation(&OperationType::Subtraction, 1, 1),
            Some(0)
        )
    }

    #[test]
    fn test_invalid_add_operator() {
        assert_eq!(
            process_operation(&OperationType::Addition, i32::MAX, 1),
            None
        )
    }

    #[test]
    fn test_invalid_sub_operator() {
        assert_eq!(
            process_operation(&OperationType::Subtraction, -i32::MAX, i32::MAX),
            None
        )
    }
}
