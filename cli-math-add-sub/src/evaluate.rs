use crate::tokenizer::{OperationType, Token, TokenType};

pub fn process_token_list(token_list: &Vec<Token>) -> Option<i32> {
    let size = token_list.len();

    if size == 0 {
        return Some(0);
    }

    let mut index = 0;
    let mut result = 0;
    let mut value_offset = 0;
    let mut first_token = true;

    while index < size {
        let token = token_list.get(index).unwrap();

        if token.token_type == TokenType::Operator {
            if first_token {
                // this is the first found "operator" token
                value_offset = index - 1;
            }

            if value_offset > 0 {
                let opt_value = if first_token {
                    // the first token requires two values from the "token_list"
                    process_operation(
                        &token.operation,
                        &token_list.get(value_offset).unwrap().value,
                        &token_list.get(value_offset - 1).unwrap().value,
                    )
                } else {
                    // every other token after the first one only needs one value from the "token_list"
                    process_operation(
                        &token.operation,
                        &result,
                        &token_list.get(value_offset - 1).unwrap().value,
                    )
                };

                if opt_value.is_some() {
                    result = opt_value.unwrap();
                    value_offset -= 1;
                } else {
                    return None;
                }
            } else {
                return None;
            }

            // all tokens after the current aren't the first
            first_token = false;
        }

        index += 1;
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
