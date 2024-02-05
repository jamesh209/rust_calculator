use std::env;

mod calc_parser;
use calc_parser::parse_all_tokens;
use crate::calc_parser::Token;

mod calc_evaluator;
use calc_evaluator::postfix_evaluation;
use calc_evaluator::shunting_yard;

fn main() {
    let args: Vec<String> = env::args().collect();
    let tokens : Vec<Token> = parse_all_tokens(args);
    let postfix_tokens = shunting_yard(&tokens);
    println!("{}", postfix_evaluation(&postfix_tokens));
}