use std::env;

mod evaluate;
mod tokenizer;

fn main() {
    let input = env::args().last();

    if let Some(expression) = input {
        let token_list = tokenizer::tokenize_expression(&expression);

        if let Some(result) = evaluate::process_token_list(&token_list) {
            println!("{}", result);
        } else {
            eprintln!("could not process given expression");
        }
    } else {
        eprintln!("no expression given");
    }
}
