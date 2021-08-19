
// Tokenizer:
    // let tokenizer = Tokenizer::new(source_code).tokenize()
    // For token in tokenizer {
    //    token.view()
//     }
// should print out parts of the src labeled with corresponding token type.
use lazy_static::lazy_static;
use regex::Regex;


pub struct Token {
    tokentype: TokenType,
    value: String
}
impl Token {
    pub fn new(tokentype: TokenType, value: String) -> Token {
        Token {
            tokentype: tokentype,
            value: value,
        } 
    }
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.tokentype.as_string(), self.value)
    }
}


pub enum TokenType {
    Name,
    Op,
    Num,
    Text,   //would prefer TokenType::String, but in order to avoid confusion...
    Newline,
}
impl TokenType {
    pub fn as_string(&self) -> String{
        match self {
            TokenType::Name => "Name".to_string(),
            TokenType::Op => "Op".to_string(),
            TokenType::Num => "Num".to_string(),
            TokenType::Text => "Text".to_string(),
            TokenType::Newline => "Newline".to_string(),
        }
    }
}

pub struct Tokenizer {
    parse_stack: Vec<TokenType>,
    tokens: Vec<Token>,
}
impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            parse_stack: vec![
                TokenType::Name,
                TokenType::Op,
                TokenType::Num,
                TokenType::Text,
                TokenType::Newline,
            ],
            tokens: Vec::<Token>::new(),
        }
    }
    pub fn parse_token(&mut self, text: &str, re: Regex) -> Token {
        Token{
            tokentype: TokenType::Name,
            value: re.captures(text).unwrap()
                                    .get(0)
                                    .unwrap()
                                    .as_str()
                                    .to_string()
        }
    }
    pub fn name_rule() -> Regex {
            Regex::new(r"^(?i)[a-z_](?i)[a-z_0-9]+").unwrap()
                
    }
}



//#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let re: Regex = Tokenizer::name_rule();
        let mut tokenizer = Tokenizer::new();
        let parsed = tokenizer.parse_token("ok_93 hello world",re);
        println!("{}", Token{tokentype: TokenType::Name, value: "var1".to_string()});
        println!("{}",parsed)
    }
}
