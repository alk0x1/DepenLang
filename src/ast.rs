use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Term {
  Var(String),
  Abs(String, Box<Term>),
  App(Box<Term>, Box<Term>),
}

impl Term {
  pub fn pretty_print(&self) -> String {
    match self {
      Term::Var(var) => var.clone(),
      Term::Abs(param, body) => format!("\\{}. {}", param, body.pretty_print()),
      Term::App(func, arg) => {
        let func_str = match **func {
          Term::Abs(_, _) => format!("({})", func.pretty_print()),
          _ => func.pretty_print(),
        };
        let arg_str = match **arg {
          Term::App(_, _) | Term::Abs(_, _) => format!("({})", arg.pretty_print()),
          _ => arg.pretty_print(),
        };
        format!("{} {}", func_str, arg_str)
      }
    }
  }

  pub fn ascii_tree(&self) -> String {
    self.ascii_tree_helper("", true)
  }

  fn ascii_tree_helper(&self, indent: &str, is_last: bool) -> String {
    let mut tree = String::new();
    tree.push_str(indent);

    if is_last {
      tree.push_str("└── ");
    } else {
      tree.push_str("├── ");
    }

    match self {
      Term::Var(var) => {
        tree.push_str(&format!("Var ({})\n", var));
      }
      Term::Abs(param, body) => {
        tree.push_str(&format!("Abs ({})\n", param));
        tree.push_str(&body.ascii_tree_helper(&format!("{}  ", indent), true));
      }
      Term::App(func, arg) => {
        tree.push_str("App\n");
        tree.push_str(&func.ascii_tree_helper(&format!("{}│ ", indent), false));
        tree.push_str(&arg.ascii_tree_helper(&format!("{}  ", indent), true));
      }
    }

    tree
  }
}

impl fmt::Display for Term {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.pretty_print())
  }
}

pub fn subst(var: &str, replacement: &Term, term: &Term) -> Term {
  match term {
    Term::Var(x) => {
      if x == var {
        replacement.clone()
      } else {
        Term::Var(x.clone())
      }
    }
    Term::Abs(param, body) => {
      if param == var {
        Term::Abs(param.clone(), body.clone())
      } else {
        Term::Abs(param.clone(), Box::new(subst(var, replacement, body)))
      }
    }
    Term::App(t1, t2) => Term::App(
      Box::new(subst(var, replacement, t1)),
      Box::new(subst(var, replacement, t2)),
    ),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pretty_print() {
    let var = Term::Var("x".to_string());
    assert_eq!(var.pretty_print(), "x");
    let abs = Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string())));
    assert_eq!(abs.pretty_print(), "\\x. x");

    let app = Term::App(
      Box::new(Term::Var("f".to_string())),
      Box::new(Term::Var("x".to_string())),
    );
    assert_eq!(app.pretty_print(), "f x");

    let complex_term = Term::App(
      Box::new(Term::Abs(
        "x".to_string(),
        Box::new(Term::Abs(
          "y".to_string(),
          Box::new(Term::Var("x".to_string())),
        )),
      )),
      Box::new(Term::Var("a".to_string())),
    );
    assert_eq!(complex_term.pretty_print(), "(\\x. \\y. x) a");
  }

  #[test]
  fn test_substitution() {
    let var_x = Term::Var("x".to_string());
    let var_y = Term::Var("y".to_string());
    let abs = Term::Abs("x".to_string(), Box::new(var_x.clone()));
    let app = Term::App(Box::new(var_y.clone()), Box::new(var_x.clone()));

    assert_eq!(subst("x", &var_y, &var_x), var_y);
    assert_eq!(subst("x", &var_y, &abs), abs);
    assert_eq!(
      subst("x", &var_y, &app),
      Term::App(Box::new(var_y.clone()), Box::new(var_y))
    );
  }
}
