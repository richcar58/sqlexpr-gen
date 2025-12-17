/// Common utilities shared between Phase I and Phase II
///
/// This module contains types and utilities used across different phases
/// of test generation.

pub mod output;

// Re-export commonly used items
pub use output::{RuntimeValue, TestExpression};
