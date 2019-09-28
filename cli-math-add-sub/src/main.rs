use std::env;

mod evaluate;
mod tokenizer;

fn main() {
    let input = env::args().last();

    if input.is_some() {
        let expression = input.unwrap();
        let token_list = tokenizer::tokenize_expression(&expression);
        let result = evaluate::process_token_list(&token_list);

        if result.is_some() {
            println!("result: {}", result.unwrap());
        } else {
            println!("could not process given expression");
        }
    }
}
