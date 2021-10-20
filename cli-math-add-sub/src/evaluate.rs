use crate::tokenizer::{OperationType, Token};

pub fn process_token_list(tokens: &[Token]) -> Option<i64> {
    let size = tokens.len();

    if size == 0 {
        return Some(0);
    }

    let mut result = 0;
    let mut token_index = 0;
    let mut value_index = 0;

    for index in 0..size {
        let token = &tokens[index];

        if let Token::Operator(operator_type) = token {
            if token_index == 0 {
                token_index = index;
            }

            if value_index + 1 < token_index {
                if value_index == 0 {
                    result = tokens[value_index].get_opt_value()?;
                }

                result = process_operation(operator_type, result, tokens[value_index + 1].get_opt_value()?)?;
                value_index += 1;
            } else {
                // invalid expression
                return None;
            }
        }
    }

    Some(result)
}

fn process_operation(operator: &OperationType, value_a: i64, value_b: i64) -> Option<i64> {
    match operator {
        OperationType::Addition => value_a.checked_add(value_b),
        OperationType::Subtraction => value_a.checked_sub(value_b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_add_tokens() {
        assert_eq!(
            process_token_list(&[
                Token::Value(1),
                Token::Value(1),
                Token::Operator(OperationType::Addition)
            ]),
            Some(2)
        )
    }

    #[test]
    fn test_valid_sub_tokens() {
        assert_eq!(
            process_token_list(&[
                Token::Value(1),
                Token::Value(1),
                Token::Operator(OperationType::Subtraction)
            ]),
            Some(0)
        )
    }

    #[test]
    fn test_valid_add_sub_tokens() {
        assert_eq!(
            process_token_list(&[
                Token::Value(1),
                Token::Value(1),
                Token::Value(2),
                Token::Operator(OperationType::Addition),
                Token::Operator(OperationType::Subtraction)
            ]),
            Some(0)
        )
    }

    #[test]
    fn test_valid_sub_add_tokens() {
        assert_eq!(
            process_token_list(&[
                Token::Value(1),
                Token::Value(1),
                Token::Value(2),
                Token::Operator(OperationType::Subtraction),
                Token::Operator(OperationType::Addition)
            ]),
            Some(2)
        )
    }

    #[test]
    fn test_invalid_add_tokens() {
        assert_eq!(
            process_token_list(&[
                Token::Value(1),
                Token::Operator(OperationType::Addition)
            ]),
            None
        )
    }

    #[test]
    fn test_invalid_sub_tokens() {
        assert_eq!(
            process_token_list(&[
                Token::Value(1),
                Token::Operator(OperationType::Subtraction)
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
            process_operation(&OperationType::Addition, i64::MAX, 1),
            None
        )
    }

    #[test]
    fn test_invalid_sub_operator() {
        assert_eq!(
            process_operation(&OperationType::Subtraction, i64::MIN, i64::MAX),
            None
        )
    }
}
