#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Lambda,
    Dot,
    LeftParen,
    RightParen,
    Identifier(String),
}
pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            match ch {
                '\\' => {
                    self.advance();
                    tokens.push(Token::Lambda);
                }
                '.' => {
                    self.advance();
                    tokens.push(Token::Dot);
                }
                '(' => {
                    self.advance();
                    tokens.push(Token::LeftParen);
                }
                ')' => {
                    self.advance();
                    tokens.push(Token::RightParen);
                }
                c if c.is_whitespace() => {
                    self.advance();
                }
                c if c.is_alphabetic() => {
                    let identifier = self.read_identifier();
                    tokens.push(Token::Identifier(identifier));
                }
                _ => return Err(format!("Unexpected character: {}", ch)),
            }
        }

        Ok(tokens)
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).cloned()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while let Some(ch) = self.peek() {
            if ch.is_alphabetic() {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "(\\x. x y) (\\z. z)";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::LeftParen,
                Token::Lambda,
                Token::Identifier("x".to_string()),
                Token::Dot,
                Token::Identifier("x".to_string()),
                Token::Identifier("y".to_string()),
                Token::RightParen,
                Token::LeftParen,
                Token::Lambda,
                Token::Identifier("z".to_string()),
                Token::Dot,
                Token::Identifier("z".to_string()),
                Token::RightParen,
            ]
        );
    }
}