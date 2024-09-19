#[derive(Clone, PartialEq, Debug)]
enum Term {
	Var(String),                // x
	Abs(String, Box<Term>),     // λx. M
	App(Box<Term>, Box<Term>),  // M N
}

fn subst(var: &str, replacement: &Term, term: &Term) -> Term {
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
		Term::App(t1, t2) => {
			Term::App(Box::new(subst(var, replacement, t1)), Box::new(subst(var, replacement, t2)))
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substitution() {
			let var_x = Term::Var("x".to_string());
			let var_y = Term::Var("y".to_string());
			let abs = Term::Abs("x".to_string(), Box::new(var_x.clone()));
			let app = Term::App(Box::new(var_y.clone()), Box::new(var_x.clone()));

			assert_eq!(subst("x", &var_y, &var_x), var_y);
			assert_eq!(subst("x", &var_y, &abs), abs);
			assert_eq!(subst("x", &var_y, &app), Term::App(Box::new(var_y.clone()), Box::new(var_y)));
    }
}
