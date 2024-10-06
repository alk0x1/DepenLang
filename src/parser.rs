use crate::lexer::{Token, Lexer};
use crate::ast::Term;

#[derive(Debug, PartialEq)]
pub enum ParseError {
  UnexpectedToken(Token),
  UnexpectedEndOfInput,
  InvalidExpression,
}

pub struct Parser {
  tokens: Vec<Token>,
  current: usize,
}

impl Parser {
  pub fn new(input: &str) -> Self {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap_or_else(|e| panic!("Lexer error: {:?}", e));
    Parser {
      tokens,
      current: 0,
    }
  }

  pub fn parse(&mut self) -> Result<Term, ParseError> {
    self.expression()
  }

  fn expression(&mut self) -> Result<Term, ParseError> {
    self.application()
  }

  fn application(&mut self) -> Result<Term, ParseError> {
    let mut expr = self.atom()?;
    while self.peek().is_some() && !matches!(self.peek(), Some(Token::RightParen)) {
      let right = self.atom()?;
      expr = Term::App(Box::new(expr), Box::new(right));
    }
    Ok(expr)
  }

  fn atom(&mut self) -> Result<Term, ParseError> {
    match self.advance() {
      Some(Token::Identifier(name)) => Ok(Term::Var(name)),
      Some(Token::Lambda) => self.abstraction(),
      Some(Token::LeftParen) => {
        let expr = self.expression()?;
        self.consume(Token::RightParen)?;
        Ok(expr)
      }
      Some(token) => Err(ParseError::UnexpectedToken(token)),
      None => Err(ParseError::UnexpectedEndOfInput),
    }
  }

  fn abstraction(&mut self) -> Result<Term, ParseError> {
    let param = match self.advance() {
      Some(Token::Identifier(name)) => name,
      Some(token) => return Err(ParseError::UnexpectedToken(token)),
      None => return Err(ParseError::UnexpectedEndOfInput),
    };
    self.consume(Token::Dot)?;
    let body = self.expression()?;
    Ok(Term::Abs(param, Box::new(body)))
  }

  fn advance(&mut self) -> Option<Token> {
    if self.is_at_end() {
      None
    } else {
      let token = self.tokens[self.current].clone();
      self.current += 1;
      Some(token)
    }
  }

  fn peek(&self) -> Option<&Token> {
    if self.is_at_end() {
      None
    } else {
      Some(&self.tokens[self.current])
    }
  }

  fn consume(&mut self, expected: Token) -> Result<(), ParseError> {
    if self.check(&expected) {
      self.advance();
      Ok(())
    } else {
      Err(ParseError::UnexpectedToken(self.peek().cloned().unwrap_or(Token::Identifier("EOF".to_string()))))
    }
  }

  fn check(&self, token: &Token) -> bool {
    self.peek().map_or(false, |t| t == token)
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.tokens.len()
  }
}

pub fn parse(input: &str) -> Result<Term, ParseError> {
  let mut parser = Parser::new(input);
  parser.parse()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_variable() {
    assert_eq!(parse("x"), Ok(Term::Var("x".to_string())));
  }

  #[test]
  fn test_parse_abstraction() {
    assert_eq!(
      parse("\\x. x"),
      Ok(Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string()))))
    );
  }

  #[test]
  fn test_parse_application() {
    assert_eq!(
      parse("x y"),
      Ok(Term::App(
        Box::new(Term::Var("x".to_string())),
        Box::new(Term::Var("y".to_string()))
      ))
    );
  }

  #[test]
  fn test_parse_complex_term() {
    assert_eq!(
      parse("(\\x. x y) (\\z. z)"),
      Ok(Term::App(
        Box::new(Term::Abs(
          "x".to_string(),
          Box::new(Term::App(
            Box::new(Term::Var("x".to_string())),
            Box::new(Term::Var("y".to_string()))
          ))
        )),
        Box::new(Term::Abs(
          "z".to_string(),
          Box::new(Term::Var("z".to_string()))
        ))
      ))
    );
  }
  
}