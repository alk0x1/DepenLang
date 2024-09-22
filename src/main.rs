use std::{fs, io::Read};

use interpreter::{eval, Env, Value};

mod lexer;
mod parser;
use lexer::Term;
use parser::parse_term;
mod interpreter;
mod typechecker;

fn main() {
  run_file("../test_files/identity.lisp").expect("error on running file");
}

fn run_file(filename: &str) -> Result<Value, String> {
  let mut file = fs::File::open(filename).map_err(|e| format!("Error opening file: {}", e))?;
  let mut contents = String::new();
  file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;

  // Step 2: Parse the content
  let (remaining_input, parsed_term) = parse_term(&contents).map_err(|e| format!("Error parsing file: {}", e))?;

  // Ensure no remaining input that wasn't parsed
  if !remaining_input.is_empty() {
    return Err(format!("Unparsed input remaining: {:?}", remaining_input));
  }

  // Step 3: Create an empty environment
  let env = Env::new();

  // Step 4: Evaluate the term
  let result = eval(parsed_term, &env);

  // Step 5: Return or print the result
  Ok(result)
}

#[cfg(test)]
mod file_tests {
  use std::sync::Arc;

use super::*;

  #[test]
  fn test_run_file_identity_function() {
    let file_contents = include_str!("../test_files/identity.lisp");
    let (remaining_input, parsed_term) = parse_term(file_contents).expect("error parsing the term");
    println!("remaing: {}", remaining_input);
    
    let env = Env::new();
    let result = eval(parsed_term, &env);

    // Test the identity function by applying it to a value
    if let Value::Closure(closure) = result {
      let input_value = Value::Var("x".to_string());
      let applied_result = closure(input_value.clone());

      // Ensure the closure behaves as an identity function
      assert_eq!(applied_result, input_value);
    } else {
      panic!("Expected a closure, but got {:?}", result);
    }
  }

  #[test]
  fn test_run_file_constant_function() {
    let file_contents = include_str!("../test_files/constant.lisp");
    
    // Parse all terms in the file, ensuring there's no unparsed input left
    let (remaining_input, parsed_term) = parse_term(file_contents).expect("error parsing term");
    
    // Ensure there's no remaining unparsed input
    assert!(remaining_input.trim().is_empty(), "Unparsed input remaining: {:?}", remaining_input);
    
    // Create a fresh environment and evaluate the parsed term
    let env = Env::new();
    let result = eval(parsed_term, &env);

    // Expected result for the constant function (λx. λy. x) applied to 'a' and 'b' should return 'a'
    let expected = Value::Var("a".to_string());
    assert_eq!(result, expected);
  }

  #[test]
  fn test_run_file_environment_capture() {
      let mut env = Env::new();
      env.insert("z".to_string(), Value::Var("z_value".to_string()));

      // Read and parse the contents of the 'capture.lisp' file
      let file_contents = include_str!("../test_files/capture.lisp");
      let (remaining_input, parsed_term) = parse_term(file_contents).expect("error parsing term");

      // Ensure there's no remaining unparsed input
      assert!(remaining_input.trim().is_empty(), "Unparsed input remaining: {:?}", remaining_input);

      // Evaluate the parsed term in the pre-populated environment
      let result = eval(parsed_term, &env);

      // The result should be a closure
      if let Value::Closure(closure) = result {
          // Apply the closure to any argument (it should be ignored)
          let applied_result = closure(Value::Var("dummy".to_string()));
          
          // Check that the result correctly captures the value of 'z' from the environment
          assert_eq!(applied_result, Value::Var("z_value".to_string()));
      } else {
          panic!("Expected a closure, but got {:?}", result);
      }
  }

//   #[test]
// fn test_boolean_logic() {
//     let file_contents = include_str!("../test_files/boolean_logic.lisp");
//     let (remaining_input, parsed_term) = parse_term(file_contents).expect("error parsing term");
    
//     // Ensure there's no remaining unparsed input
//     assert!(remaining_input.trim().is_empty(), "Unparsed input remaining: {:?}", remaining_input);
    
//     let env = Env::new();
//     let result = eval(parsed_term, &env);

//     // The result should be a closure (our NOT function)
//     if let Value::Closure(not_function) = result {
//         // Create true value: \x. \y. x
//         let true_value = Value::Closure(Arc::new(|x: Value| {
//             Value::Closure(Arc::new(move |_: Value| x.clone()))
//         }));

//         // Apply NOT to true (which should return false)
//         let not_true = not_function(true_value);

//         // Apply the result to two arbitrary values. If it's false, it should return the second value.
//         if let Value::Closure(false_result) = not_true {
//             let arbitrary_value1 = Value::Var("arbitrary1".to_string());
//             let arbitrary_value2 = Value::Var("arbitrary2".to_string());
            
//             // We need to apply false_result twice because our booleans take two arguments
//             let final_result = false_result(arbitrary_value1.clone());
//             if let Value::Closure(final_closure) = final_result {
//                 let actual_final_result = final_closure(arbitrary_value2.clone());

//                 // The final result should be arbitrary_value2, indicating that NOT(true) correctly returned false
//                 assert_eq!(actual_final_result, arbitrary_value2);
//             } else {
//                 panic!("Expected a closure after first application, but got {:?}", final_result);
//             }
//         } else {
//             panic!("Expected a closure representing false, but got {:?}", not_true);
//         }
//     } else {
//         panic!("Expected a closure, but got {:?}", result);
//     }
// }
}
