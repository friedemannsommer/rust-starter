use crate::tokenizer::{OperationType, Token, TokenType};

pub fn process_token_list(token_list: &Vec<Token>) -> Option<i32> {
    let size = token_list.len();

    if size == 0 {
        return Some(0);
    }

    let mut index = 1;
    let mut result;
    let first_token = token_list.get(0).unwrap();

    if first_token.token_type == TokenType::Value {
        result = first_token.value;
    } else {
        return None
    }

    while index < size {
        let token = token_list.get(index).unwrap();

        if token.token_type == TokenType::Operator {
            if index + 1 < size {
                let opt_value = process_operation(
                    &token.operation,
                    &result,
                    &token_list.get(index + 1).unwrap().value,
                );

                if opt_value.is_some() {
                    result = opt_value.unwrap();
                    index += 2;
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            index += 1;
        }
    }

    Some(result)
}

fn process_operation(operator: &OperationType, value_a: &i32, value_b: &i32) -> Option<i32> {
    match operator {
        OperationType::Addition => Some(value_a + value_b),
        OperationType::Subtraction => Some(value_a - value_b),
        _ => None,
    }
}
