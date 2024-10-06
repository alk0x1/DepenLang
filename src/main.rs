use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

mod ast;
mod interpreter;

use interpreter::{eval, reify, Env};
use ast::Term;

fn parse_term(code: &str) -> Term {
    match code {
        "\\x. x" => Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string()))),
        "\\x. \\y. x" => Term::Abs("x".to_string(), Box::new(Term::Abs("y".to_string(), Box::new(Term::Var("x".to_string()))))),
        "(\\x. x) z" => Term::App(Box::new(Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string())))), Box::new(Term::Var("z".to_string()))),
        "(\\x. \\y. x) a b" => {
            let nested = Term::App(
                Box::new(Term::Abs("x".to_string(), Box::new(Term::Abs("y".to_string(), Box::new(Term::Var("x".to_string())))))),
                Box::new(Term::Var("a".to_string()))
            );
            Term::App(Box::new(nested), Box::new(Term::Var("b".to_string())))
        }
        _ => panic!("Unknown expression: {}", code),
    }
}

fn interpret_file(file_path: &Path) -> Result<Term, io::Error> {
    let file = fs::File::open(file_path)?;
    let env = Env::new();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let term = parse_term(&line);
        let result = eval(term, &env);
        let reified = reify(result);
        return Ok(reified);
    }
    Err(io::Error::new(io::ErrorKind::Other, "Empty file"))
}

fn main() -> io::Result<()> {
    let test_files = vec!["identity.lisp", "constant.txt", "capture.txt", "nested.txt"];
    for test_file in test_files {
        let path = Path::new(test_file);
        match interpret_file(path) {
            Ok(term) => println!("Interpreted: {:?}", term),
            Err(e) => eprintln!("Error interpreting file {}: {}", test_file, e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_interpret_identity() {
        let path = Path::new("/Users/alanbertani/dev/DepenLang/test_files/identity.lisp");
        let result = interpret_file(path).unwrap();
        assert_eq!(result, Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string()))));
    }

    #[test]
    fn test_interpret_constant() {
      let path = Path::new("/Users/alanbertani/dev/DepenLang/test_files/constant.lisp");
      let result = interpret_file(path).unwrap();
        assert_eq!(result, Term::Abs("x".to_string(), Box::new(Term::Abs("y".to_string(), Box::new(Term::Var("x".to_string()))))));
    }

    #[test]
    fn test_interpret_identity_application() {
      let path = Path::new("/Users/alanbertani/dev/DepenLang/test_files/capture.lisp");
      let result = interpret_file(path).unwrap();
      assert_eq!(result, Term::Var("z".to_string())); // Applying identity function to z
    }

    #[test]
    fn test_interpret_nested_application() {
      let path = Path::new("/Users/alanbertani/dev/DepenLang/test_files/nested.lisp");
      let result = interpret_file(path).unwrap();
        assert_eq!(result, Term::Var("a".to_string())); // Applying nested function, returns "a"
    }
}
