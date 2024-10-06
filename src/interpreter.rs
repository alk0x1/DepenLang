use core::fmt;
use std::{collections::HashMap, sync::Arc};

use crate::ast::Term;

#[derive(Clone)]
pub enum Value {
  Var(String),
  Closure(Arc<dyn Fn(Value) -> Value>),
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Value::Var(x), Value::Var(y)) => x == y,
      (Value::Closure(_), Value::Closure(_)) => false,
      _ => false,
    }
  }
}

impl fmt::Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Var(v) => write!(f, "Var({:?})", v),
      Value::Closure(_) => write!(f, "Closure(<function>)"),
    }
  }
}

pub type Env = HashMap<String, Value>;

pub fn eval(term: Term, env: &Env) -> Value {
  match term {
    Term::Var(x) => env.get(&x).cloned().unwrap_or(Value::Var(x)),
    Term::Abs(x, body) => {
      let env = env.clone();
      Value::Closure(Arc::new(move |arg: Value| {
        let mut new_env = env.clone();
        new_env.insert(x.clone(), arg);
        eval(*body.clone(), &new_env)
      }))
    }
    Term::App(t1, t2) => {
      let func = eval(*t1, env);
      let arg = eval(*t2, env);
      match func {
        Value::Closure(f) => f(arg),
        _ => panic!("Trying to apply a non-function"),
      }
    }
  }
}

pub fn reify(val: Value) -> Term {
  match val {
    Value::Var(x) => Term::Var(x),
      
    Value::Closure(f) => {
      let dummy_var = Value::Var("x".to_string());
      
      let result = f(dummy_var.clone());
      Term::Abs("x".to_string(), Box::new(reify(result)))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_variable_evaluation() {
    let mut env = Env::new();
    env.insert("x".to_string(), Value::Var("x_value".to_string()));

    let term = Term::Var("x".to_string());
    let result = eval(term, &env);

    assert_eq!(result, Value::Var("x_value".to_string()));
  }

  #[test]
  fn test_identity_function() {
    let env = Env::new();
    let identity = Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string())));  // λx. x
    
    let app = Term::App(Box::new(identity), Box::new(Term::Var("y".to_string())));  // Apply λx.x to "y"

    let result = eval(app, &env);

    assert_eq!(result, Value::Var("y".to_string()));
  }

  #[test]
  fn test_function_application() {
    let env = Env::new();

    // λx. λy. x (constant function)
    let constant_func = Term::Abs( 
      "x".to_string(),
      Box::new(Term::Abs(
        "y".to_string(),
        Box::new(Term::Var("x".to_string())),
      )),
    );

    let app1 = Term::App(Box::new(constant_func), Box::new(Term::Var("a".to_string())));  // Apply (λx. λy. x) to "a"
    let app2 = Term::App(Box::new(app1), Box::new(Term::Var("b".to_string())));  // Apply the result to "b"

    let result = eval(app2, &env);
    assert_eq!(result, Value::Var("a".to_string()));
  }

  #[test]
  fn test_environment_closure() {
    let mut env = Env::new();
    env.insert("z".to_string(), Value::Var("z_value".to_string()));
    
    let closure_with_env = Term::Abs("x".to_string(), Box::new(Term::Var("z".to_string()))); // λx. z (z captured from environment)

    // Apply closure to "anything"
    let app = Term::App(Box::new(closure_with_env), Box::new(Term::Var("ignored".to_string())));
    let result = eval(app, &env);
    assert_eq!(result, Value::Var("z_value".to_string()));
  }

  #[test]
  fn test_reify_variable() {
    // Testing reifying a variable
    let var_value = Value::Var("x".to_string());
    let reified = reify(var_value);
    assert_eq!(reified, Term::Var("x".to_string()));
  }

  #[test]
  fn test_reify_identity_function() {
    // Testing reifying the identity function
    let identity_closure = Value::Closure(Arc::new(|arg: Value| arg));
    let reified = reify(identity_closure);
    assert_eq!(
      reified,
      Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string()))) // λx.x
    );
  }

  #[test]
  fn test_reify_constant_function() {
    // Reifying a function that ignores its argument and always returns "a"
    let constant_closure = Value::Closure(Arc::new(|_arg: Value| Value::Var("a".to_string())));
    let reified = reify(constant_closure);
    assert_eq!(
      reified,
      Term::Abs("x".to_string(), Box::new(Term::Var("a".to_string()))) // λx. a
    );
  }

  #[test]
  fn test_reify_nested_function() {
    // Reifying a nested function λx. λy. x (ignores y and returns x)
    let nested_closure = Value::Closure(Arc::new(|arg: Value| {
      Value::Closure(Arc::new(move |_arg2: Value| arg.clone()))
    }));
    
    let reified = reify(nested_closure);
    assert_eq!(
      reified,
      Term::Abs(
        "x".to_string(),
        Box::new(Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string())))) // λx. λx. x
      )
    );
  }

  #[test]
  fn test_reify_environment_closure() {
    // Reifying a closure that captures the environment (z) and returns it
    let mut env = Env::new();
    env.insert("z".to_string(), Value::Var("z_value".to_string()));
    
    let closure_with_env = Value::Closure(Arc::new(|_arg: Value| {
      // Return the captured "z" from the environment
      Value::Var("z_value".to_string())
    }));
    
    let reified = reify(closure_with_env);
    assert_eq!(
      reified,
      Term::Abs("x".to_string(), Box::new(Term::Var("z_value".to_string()))) // λx. z_value
    );
  }
}
