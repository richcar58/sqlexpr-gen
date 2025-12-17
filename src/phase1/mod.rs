/// Phase I: Simple expression generation
///
/// This module contains all code for generating simple single-operator
/// relational expressions that will be used as building blocks for
/// complex expressions in Phase II.

pub mod templates;
pub mod generator;
pub mod validator;

// Re-export commonly used items
pub use templates::{Template, VarCounter, get_all_templates};
pub use generator::generate_from_template;
pub use validator::validate_expression;
