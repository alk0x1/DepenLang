# Dependent Type Checker in Rust

This project implements a simple dependent type checker to demonstrate lambda calculus and lambda encoding in Rust. The syntax is inspired by lambda calculus and dependent type theory. This language supports basic type-checking with dependent types, lambda abstraction, and encoded data structures like natural numbers and booleans.

## Syntax Overview

The syntax consists of the following elements:
- **Variables**: Single-letter identifiers such as `x`, `y`, `z`.
- **Lambda Abstraction**: Represented using `\` to define anonymous functions.
- **Function Application**: Function application is written by placing the function and its argument(s) next to each other.
- **Type Declaration**: Declared using `:`, for example `x: Nat` means `x` is of type `Nat`.
- **Dependent Types**: Defined using `forall`, indicating types that depend on values.

The syntax is designed to be simple, ASCII-friendly, and easy to understand, focusing on the core concepts of lambda calculus and dependent types.

## Syntax
- **Variable**: `x`
- **Lambda abstraction**: `\x: T. expr`
  - Defines a function with parameter `x` of type `T` and body `expr`.
- **Function application**: `f x`
  - Applies function `f` to argument `x`.

## Type System
- **Dependent Function Types**: 
  - A function can have a type that depends on the argument:
    ```
    f: (x: A) -> B(x)
    ```
    This means `f` takes an argument `x` of type `A` and returns something of type `B(x)`, where the return type depends on `x`.

## Examples
- **Boolean Logic in Lambda Calculus**:
  ```rust
  true  = \t: Type. \f: Type. t
  false = \t: Type. \f: Type. f
  if_then_else = \b: (t: Type) -> (f: Type) -> Type. \x: T. \y: T. b x y

  // Use it like:
  if_then_else true 42 0  // Evaluates to 42
  if_then_else false 42 0 // Evaluates to 0
	```