use nom::{
    error::Error,
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, space0},
    sequence::tuple,
    IResult,
};
use crate::lexer::Term;

fn parse_parens(input: &str) -> IResult<&str, Term> {
  let (input, _) = char('(')(input)?;
  let (input, term) = parse_term(input)?;
  let (input, _) = char(')')(input)?;
  Ok((input, term))
}

fn parse_abs(input: &str) -> IResult<&str, Term> {
  let (input, _) = char('\\')(input)?;
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
  let (input, func) = parse_var(input)?; // Parse the first term

  // Keep applying arguments to the function as long as there are more terms
  let mut current_term = func;
  let mut input = input;

  while let Ok((new_input, (_, next_term))) = tuple((space0, parse_var))(input) {
    input = new_input;
    current_term = Term::App(Box::new(current_term), Box::new(next_term)); // Left-associative application
  }

  Ok((input, current_term))
}


pub fn parse_term(input: &str) -> IResult<&str, Term, Error<&str>> {
  let input = input.trim_start();

  let (mut input, mut term) = alt((parse_parens, parse_abs, parse_var))(input)?;

  while let Ok((new_input, (_, next_term))) = tuple((space0, alt((parse_parens, parse_abs, parse_var))))(input) {
    input = new_input;
    term = Term::App(Box::new(term), Box::new(next_term)); // Left-associative application
  }

  Ok((input, term))
}

fn continue_parsing_applications(input: &str, initial_term: Term) -> IResult<&str, Term> {
  let mut term = initial_term;
  let mut input = input;

  // Try to continue parsing applications as long as there are valid terms to apply
  while let Ok((new_input, next_term)) = parse_term(input.trim_start()) {
    input = new_input;
    term = Term::App(Box::new(term), Box::new(next_term));
  }

  Ok((input, term))
}



#[cfg(test)]
mod tests {
  use super::*;
  use crate::lexer::Term;

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
      parse_app("x y"),
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
  fn test_parse_term() {
    assert_eq!(
      parse_term("\\x.x"),
      Ok((
        "",
        Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string())))
      ))
    );

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
  fn test_parse_parens() {
    assert_eq!(
      parse_parens("(x)"),
      Ok(("", Term::Var("x".to_string())))
    );

    assert_eq!(
      parse_parens("(\\x.x)"),
      Ok((
        "",
        Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string())))
      ))
    );
  }

#[test]
  fn test_full_application() {
    let input = "(\\x. \\y. x) a b";
    let expected = Ok((
      "",
      Term::App(
        Box::new(Term::App(
          Box::new(Term::Abs(
            "x".to_string(), 
            Box::new(Term::Abs(
              "y".to_string(), 
              Box::new(Term::Var("x".to_string()))
            ))
          )),
          Box::new(Term::Var("a".to_string()))
        )),
        Box::new(Term::Var("b".to_string()))
      )
    ));
    assert_eq!(parse_term(input), expected);
  }
}
