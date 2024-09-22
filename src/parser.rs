use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, space0, space1},
    combinator::map,
    error::{context, VerboseError},
    multi::many0,
    sequence::{delimited, pair, preceded},
    Err, IResult,
};
use crate::lexer::Term;

type ParseResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

#[derive(PartialEq)]
pub enum ParseErrorKind {
    UnexpectedChar(char),
    UnexpectedEOF,
    InvalidVariable,
    InvalidAbstraction,
    InvalidApplication,
    Other(String),
}

// Helper function to create context-aware errors
fn custom_error<'a>(input: &'a str, kind: ParseErrorKind) -> Err<VerboseError<&'a str>> {
  let error_msg = match kind {
    ParseErrorKind::UnexpectedChar(c) => format!("Unexpected character: {}", c),
    ParseErrorKind::UnexpectedEOF => "Unexpected end of input".to_string(),
    ParseErrorKind::InvalidVariable => "Invalid variable name".to_string(),
    ParseErrorKind::InvalidAbstraction => "Invalid lambda abstraction".to_string(),
    ParseErrorKind::InvalidApplication => "Invalid function application".to_string(),
    ParseErrorKind::Other(msg) => msg,
  };
  
  Err::Failure(VerboseError { 
    errors: vec![
      (input, nom::error::VerboseErrorKind::Context("error context")),
      ("", nom::error::VerboseErrorKind::Context(Box::leak(error_msg.into_boxed_str())))
    ]
  })
}


// Parse a variable
fn parse_var(input: &str) -> ParseResult<Term> {
  context(
    "variable",
    map(
      take_while1(|c: char| c.is_alphabetic()),
      |var: &str| Term::Var(var.to_string()),
    ),
  )(input)
  .map_err(|_: nom::Err<VerboseError<&str>>| custom_error(input, ParseErrorKind::InvalidVariable))
}

// Parse a lambda abstraction
fn parse_abs(input: &str) -> ParseResult<Term> {
  context(
    "abstraction",
    map(
      pair(
        preceded(char('\\'), take_while1(|c: char| c.is_alphanumeric())),
        preceded(char('.'), parse_term),
      ),
      |(param, body)| Term::Abs(param.to_string(), Box::new(body)),
    ),
  )(input)
  .map_err(|_: nom::Err<VerboseError<&str>>| custom_error(input, ParseErrorKind::InvalidAbstraction))
}

// Parse parentheses
fn parse_parens(input: &str) -> ParseResult<Term> {
  context(
    "parentheses",
    delimited(char('('), parse_term, char(')')),
  )(input)
}

// Parse a single term (variable, abstraction, or parenthesized expression)
fn parse_single_term(input: &str) -> ParseResult<Term> {
  alt((parse_var, parse_abs, parse_parens))(input)
}

// Parse function application
fn parse_application(input: &str) -> ParseResult<Term> {
  context(
    "application",
    map(
      pair(parse_single_term, many0(preceded(space1, parse_single_term))),
      |(first, rest)| {
        rest.into_iter().fold(first, |acc, term| {
          Term::App(Box::new(acc), Box::new(term))
        })
      },
    ),
  )(input)
  .map_err(|_: nom::Err<VerboseError<&str>>| custom_error(input, ParseErrorKind::InvalidApplication))
}

// Main parsing function
pub fn parse_term(input: &str) -> ParseResult<Term> {
  delimited(space0, parse_application, space0)(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_var() {
    assert_eq!(parse_var("x"), Ok(("", Term::Var("x".to_string()))));
  }

  #[test]
  fn test_parse_abs() {
    assert_eq!(
      parse_abs("\\x.x"),
      Ok((
        "",
        Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string())))
      ))
    );
  }

  #[test]
  fn test_parse_app() {
    assert_eq!(
      parse_term("x y"),
      Ok((
        "",
        Term::App(
          Box::new(Term::Var("x".to_string())),
          Box::new(Term::Var("y".to_string()))
        )
      ))
    );
  }

  #[test]
  fn test_parse_complex_term() {
    assert_eq!(
      parse_term("(\\x. x y) (\\z. z)"),
      Ok((
        "",
        Term::App(
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
        )
      ))
    );
  }
}