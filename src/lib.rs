

/* the Tokenizer should parse a raw string into a list of tokens. 
A token is a type value pair implemented as struct.
The tokenizer follows rules expressed in a regex format to consume and
label the first word of the source until it either fails to match the word,
or it has consumed the full source.
*/

// use lazy_static::lazy_static; not in use atm
use regex::Regex;


pub struct Tokenizer {
    tokens: Vec<Token>,
}
impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            tokens: Vec::<Token>::new(),
        }
    }
    pub fn tokenize(&mut self, src: &str) -> Result<(), String> {
        let tokens_initial_length = &self.tokens.len();
        let mut src_ = src.trim_start_matches(|c| c==' ').to_string();
        for tokentype in Tokenizer::get_type_stack() {
            match Tokenizer::parse_token(&tokentype, &src_) {
                Some(parsed) => {
                    src_ = parsed.1;
                    self.push_token(parsed.0);
                    break;
                }
                None => (),
            };
        };
        if tokens_initial_length == &self.tokens.len() {
            return Err(format!("no match in {}", src_.lines().nth(0).expect("no nth0 line")));
        } else if self.tokens.last().unwrap().tokentype != TokenType::Eof {
            return self.tokenize(&src_);
        } else {
            Ok(())
        }
    }
    pub fn push_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
    pub fn parse_token(tokentype: &TokenType, src: &str) -> Option<(Token, String)> {
        match Tokenizer::get_rule(&tokentype).captures(src) {
            Some(matched) => {
                let matched_string = matched.get(0)
                                            .expect("failed in Tokenizer::parse_token()")
                                            .as_str();
                //println!("{}", matched_string); // @debug
                match tokentype {
                    &TokenType::Newline|&TokenType::Eof => {
                        let rest = src.trim().to_string();
                        Some((Token::new(tokentype, ""), rest))
                    },
                    _ => {
                        let rest = src.strip_prefix(&matched_string)
                                      .expect(&format!("could not strip: {}", matched_string))
                                      .trim_start_matches(|c|c==' ')
                                      .to_string();
                        Some((Token::new(tokentype, matched_string), rest))
                    }
                }                            
            }
            None => None,
        
        }
    }
    pub fn get_rule(tokentype: &TokenType) -> Regex {
        // defines the rule for parsing the given tokentype.
            match tokentype { 
                TokenType::Name => Regex::new(r"^(?i)[a-z_](?i)[a-z_0-9]+|^(?i)[a-z]")
                    .expect("invalid regex in Tokenizer::name-rule"),

                TokenType::Num => Regex::new(r"^[0-9][0-9_.]+")
                    .expect("invalid regex in Num-rule"),

                TokenType::Op => Regex::new(r"^[+|\-|*|=|\.|,|;|:|!|?|←|→|]")
                    .expect("invalid regex in Op-rule"),

                TokenType::Text => Regex::new(r"^[\u0022](.*?)[\u0022]") // u0022 -> "
                    .expect("invalid regex in Text-rule"),

                TokenType::Newline => Regex::new(r"\u000a")
                    .expect("invalid regex in Newline-rule"),
                TokenType::Eof => Regex::new(r"\z")
                    .expect("invalid regex in eof-rule"),
            }
    }
    pub fn get_type_stack() -> Vec<TokenType> {
        vec![
            TokenType::Name,
            TokenType::Op,
            TokenType::Num,
            TokenType::Text,
            TokenType::Newline,
            TokenType::Eof,
        ]
    }
}




pub struct Token {
    tokentype: TokenType,
    value: String
}
impl Token {
    pub fn new(tokentype: &TokenType, value: &str) -> Token {
        Token {
            tokentype: TokenType::from(tokentype),
            value: String::from(value),
        } 
    }
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.tokentype.as_string(), self.value)
    }
}



#[derive(PartialEq)]
pub enum TokenType {
    Name,
    Op,
    Num,
    Text,   //would prefer TokenType::String, but in order to avoid confusion...
    Newline,
    Eof,
}
impl TokenType {
    pub fn as_string(&self) -> String{
        match self {
            TokenType::Name => "Name".to_string(),
            TokenType::Op => "Op".to_string(),
            TokenType::Num => "Num".to_string(),
            TokenType::Text => "Text".to_string(),
            TokenType::Newline => "Newline".to_string(),
            TokenType::Eof => "EOF".to_string(),
        }
    }
    pub fn from(tokentype: &TokenType) -> TokenType {
        match tokentype {
            &TokenType::Name => TokenType::Name,
            &TokenType::Op => TokenType::Op,
            &TokenType::Num => TokenType::Num,
            &TokenType::Text => TokenType::Text,
            &TokenType::Newline => TokenType::Newline,
            &TokenType::Eof => TokenType::Eof,
        }
    }
}



//#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {

        let test_src = r#"print "hello world", 93
        a+b = c
        "#;
        let mut tokenizer = Tokenizer::new();
        match tokenizer.tokenize(&test_src) {
            Ok(()) => for token in tokenizer.tokens {println!("{}", token)}
            Err(err) => println!("{}",err), 
        };
    }
}
