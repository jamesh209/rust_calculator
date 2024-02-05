#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Integer(i32),
    Float(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenContext,
    CloseContext,
    Err
}

pub fn parse_all_tokens(input : Vec<String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for item in input {
        tokens.push(parse_token(item));
    }

    tokens
}

pub fn parse_token(input: String, ) -> Token {
    let mut _out_token : Token = Token::Err;
    
    match input.trim() {
        "+" => _out_token = Token::Plus,
        "-" => _out_token = Token::Minus,
        "*" => _out_token = Token::Multiply,
        "/" => _out_token = Token::Divide,
        "(" | "[" | "{" => _out_token = Token::OpenContext,
        ")" | "]" | "}" => _out_token = Token::CloseContext,
        num =>{
            if let Ok(value) = num.parse::<f64>() {
                if value.fract() == 0.0 {
                    _out_token = Token::Integer(value as i32);
                } else {
                    _out_token = Token::Float(value);
                }
            } else {
                _out_token = Token::Err;
            }
        }
    }

    _out_token
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_parse_token() {
        assert_eq!(parse_token("+".to_string()), Token::Plus);
        assert_eq!(parse_token("-".to_string()), Token::Minus);
        assert_eq!(parse_token("*".to_string()), Token::Multiply);
        assert_eq!(parse_token("/".to_string()), Token::Divide);
        assert_eq!(parse_token("1".to_string()), Token::Integer(1));
        assert_eq!(parse_token("1.05".to_string()), Token::Float(1.05));
        assert_eq!(parse_token("-10".to_string()), Token::Integer(-10));
        assert_eq!(parse_token("-10.3".to_string()), Token::Float(-10.3));
        assert_eq!(parse_token("(".to_string()), Token::OpenContext);
        assert_eq!(parse_token("[".to_string()), Token::OpenContext);
        assert_eq!(parse_token("{".to_string()), Token::OpenContext);
        assert_eq!(parse_token(")".to_string()), Token::CloseContext);
        assert_eq!(parse_token("]".to_string()), Token::CloseContext);
        assert_eq!(parse_token("}".to_string()), Token::CloseContext);
    }

    #[test]
    fn test_parse_all_tokens(){
        assert_eq!(parse_all_tokens(vec![String::from("1"), String::from("+"), String::from("1")]), vec![Token::Integer(1), Token::Plus, Token::Integer(1)]);
        assert_eq!(parse_all_tokens(vec![String::from("failure"), String::from("1"), String::from("+"), String::from("1")]), vec![Token::Err, Token::Integer(1), Token::Plus, Token::Integer(1)]);
    }
}