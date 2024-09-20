use nom::{
  bytes::complete::take_while1, character::complete::{char, space0}, sequence::tuple, IResult
};

use crate::lexer::Term;


fn parse_abs(input: &str) -> IResult<&str, Term> {
  let (input, _) = char('λ')(input)?;
  let (input, param) = take_while1(|c: char| c.is_alphanumeric())(input)?;
  let (input, _) = char('.')(input)?;
  let (input, body) = parse_term(input)?;  // Calling parse_term for the body
  Ok((input, Term::Abs(param.to_string(), Box::new(body))))
}

fn parse_var(input: &str) -> IResult<&str, Term> {
  let (input, var_name) = take_while1(|c: char| c.is_alphabetic())(input)?;
  Ok((input, Term::Var(var_name.to_string())))
}


fn parse_app(input: &str) -> IResult<&str, Term> {
  let (mut input, mut func) = parse_var(input)?;

  while let Ok((new_input, (_, arg))) = tuple((space0, parse_var))(input) {
    input = new_input;
    func = Term::App(Box::new(func), Box::new(arg));
  }

  Ok((input, func))
}

fn parse_term(input: &str) -> IResult<&str, Term> {
  if let Ok(result) = parse_abs(input) {
    return Ok(result);
  }

  if let Ok(result) = parse_app(input) {
    return Ok(result);
  }

  parse_var(input)
}

#[test]
fn test_parse_var() {
  assert_eq!(parse_var("x"), Ok(("", Term::Var("x".to_string()))));
}

#[test]
fn test_parse_abs() {
  assert_eq!(parse_abs("λx.x"), Ok(("", Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string()))))));
}

#[test]
fn test_parse_app() {
  assert_eq!(parse_app("x y"), Ok(("", Term::App(Box::new(Term::Var("x".to_string())), Box::new(Term::Var("y".to_string()))))));
}

#[test]
fn test_parse_term() {
  assert_eq!(parse_term("λx.x"), Ok(("", Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string()))))));
  assert_eq!(parse_term("x y"), Ok(("", Term::App(Box::new(Term::Var("x".to_string())), Box::new(Term::Var("y".to_string()))))));
}
