use crate::calc_parser::Token;

pub trait Evaluate<T> {
    fn evaluate(self, a: T, b: T) -> T;
}

impl Evaluate<f64> for Token {
    fn evaluate(self, a: f64, b: f64) -> f64 {
        match self {
            Token::Plus => a + b,
            Token::Minus => a - b,
            Token::Multiply => a * b,
            Token::Divide => a / b,
            _ => panic!("Invalid operation for floats"),
        }
    }
}

impl Evaluate<i32> for Token {
    fn evaluate(self, a: i32, b: i32) -> i32 {
        match self {
            Token::Plus => a + b,
            Token::Minus => a - b,
            Token::Multiply => a * b,
            Token::Divide => a / b,
            _ => panic!("Invalid operation for floats"),
        }
    }
}

pub fn postfix_evaluation(input_tokens: &Vec<Token>) -> f64{
    let mut stack : Vec<f64> = Vec::new();

    for &token in input_tokens {
        match token {
            Token::Integer(num) => stack.push(num as f64),
            Token::Float(num) => stack.push(num),
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(token.evaluate(a, b));
            }
            _ => println!("Error this token should not appear in the postfix list"),
        }
    }

    return stack.pop().unwrap();
}

fn precedence(token: Token) -> i32 {
    match token {
        Token::Plus | Token::Minus => 1,
        Token::Multiply | Token::Divide => 2,
        _ => 0,
    }
}

pub fn shunting_yard(input_tokens: &Vec<Token>) -> Vec<Token>{
    let mut stack: Vec<Token> = Vec::new();
    let mut output: Vec<Token> = Vec::new();

    for &token in input_tokens {
        match token {
            Token::Integer(_num) => output.push(token),
            Token::Float(_num) => output.push(token),
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                while let Some(&top) = stack.last() {
                    if precedence(top) >= precedence(token) {
                        output.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(token);
            }
            Token::OpenContext => stack.push(token),
            Token::CloseContext => {
                while let Some(&top) = stack.last() {
                    if top == Token::OpenContext {
                        stack.pop();
                        break;
                    } else {
                        output.push(stack.pop().unwrap());
                    }
                }
            }
            _ => println!("Err was discovered and removed")
        }
    }

    while let Some(&_top) = stack.last() {
        output.push(stack.pop().unwrap());
    }

    return output;
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_resolve_expression() {
        // Test with integers
        let tokens_int = vec![
            Token::Integer(2),
            Token::Integer(3),
            Token::Integer(4),
            Token::Integer(1),
            Token::Minus,
            Token::Multiply,
            Token::Integer(5),
            Token::Divide,
            Token::Plus,
        ];

        assert_eq!(postfix_evaluation(&tokens_int), 3.8);
    }

    #[test]
    fn test_shunting_yard() {
        // Test with integers
        
        let infix_tokens = vec![
            Token::Integer(2),
            Token::Plus,
            Token::Integer(3),
            Token::Multiply,
            Token::OpenContext,
            Token::Integer(4),
            Token::Minus,
            Token::Integer(1),
            Token::CloseContext,
            Token::Divide,
            Token::Integer(5),
            ];

        let postfix_tokens = vec![
            Token::Integer(2),
            Token::Integer(3),
            Token::Integer(4),
            Token::Integer(1),
            Token::Minus,
            Token::Multiply,
            Token::Integer(5),
            Token::Divide,
            Token::Plus,
        ];

        assert_eq!(shunting_yard(&infix_tokens), postfix_tokens);
    }
}