/// SQL Expression Test Generator Library
///
/// This library provides functionality for generating test expressions
/// for the sqlexpr-rust parser and evaluator.
///
/// ## Phases
///
/// - **Phase I**: Generate simple single-operator relational expressions
/// - **Phase II**: Combine simple expressions into complex expressions for load testing
///
/// ## Usage
///
/// The library can be used through the provided binaries:
/// - `phase1`: Generate simple expressions → resources/simple_expressions.json
/// - `phase2`: Generate complex expressions → resources/complex_expressions.json
/// - `evaluator_test`: Test complex expressions and measure performance

pub mod common;
pub mod phase1;
pub mod phase2;

// Re-export commonly used items for convenience
pub use common::{RuntimeValue, TestExpression};
