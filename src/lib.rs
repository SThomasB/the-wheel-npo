

/* the Tokenizer should parse a raw string into a list of tokens. 
A token is a type value pair implemented as struct.
The tokenizer follows rules expressed in a regex format to consume and
label the first word of the source until it either fails to match the word,
or it has consumed the full source.
*/

// use lazy_static::lazy_static; not in use atm
use regex::Regex;


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
    pub fn tokenize(&mut self, src: &str) -> Option<()> {
        todo!()
        /*For tokentype in parse_stack {
            parse_token
        } */
    }
    pub fn parse_token(&mut self, tokentype: TokenType, text: &str) -> Token {
        match Tokenizer::get_rule(&tokentype).captures(text) {
            Some(matched) => Token::new(tokentype, matched.get(0).unwrap().as_str().to_string()),
            None => todo!(),
        }

    }
    pub fn get_rule(tokentype: &TokenType) -> Regex {
        // defines the rule for parsing the given tokentype.
            match tokentype { 
                TokenType::Name => Regex::new(r"^(?i)[a-z_](?i)[a-z_0-9]+")
                    .expect("invalid regex in Tokenizer::name-rule"),

                TokenType::Num => Regex::new(r"^[0-9][0-9_.]+")
                    .expect("invalid regex in Num-rule"),

                TokenType::Op => Regex::new(r"^[+|\-|*|.|,|;|:|!|?|←|→|]")
                    .expect("invalid regex in Op-rule"),

                TokenType::Text => Regex::new(r"^[\u0022](.*?)[\u0022]") // u0022 -> "
                    .expect("invalid regex in Text-rule"),

                TokenType::Newline => Regex::new(r"^[\n]")
                    .expect("invalid regex in Newline-rule"),
            }
    }
    pub fn get_parse_stack<'a>(&'a self) -> &'a Vec<TokenType> {
        &self.parse_stack
    }
}



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







//#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {

        let mut tokenizer = Tokenizer::new();
        let mut src = r#"print hello, a+b"#;  
        let token = tokenizer.parse_token(TokenType::Name, src);
        println!("{}", token);
        src = src.strip_prefix(&token.value).unwrap().trim();
        println!("{}",src);
        println!("{}",tokenizer.parse_token(TokenType::Name, src));
        let op_token = tokenizer.parse_token(TokenType::Op, r"→:+9?()[]");
        println!("{}", op_token);
        let text_token = tokenizer.parse_token(TokenType::Text,r#""hello""#);
        println!("{}", text_token);
        
    }
}
