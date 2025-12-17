use sqlexpr_rust::{evaluate, RuntimeValue as SqlExprRuntimeValue};
use crate::common::output::{TestExpression, RuntimeValue};
use std::collections::HashMap;

/// Validation error information
#[derive(Debug)]
pub struct ValidationError {
    pub expr: String,
    pub value_map: HashMap<String, RuntimeValue>,
    pub expected: bool,
    pub actual: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Expression: {}\n  Values: {:?}\n  Expected: {}\n  Actual: {}",
            self.expr, self.value_map, self.expected, self.actual
        )
    }
}

/// Validate a test expression against sqlexpr-rust evaluator
pub fn validate_expression(test: &TestExpression) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Validate true_list - all should evaluate to true
    for value_map in &test.true_list {
        let sqlexpr_map = convert_to_sqlexpr_map(value_map);
        match evaluate(&test.expr, &sqlexpr_map) {
            Ok(true) => {
                // Expected result
            }
            Ok(false) => {
                errors.push(ValidationError {
                    expr: test.expr.clone(),
                    value_map: value_map.clone(),
                    expected: true,
                    actual: "false".to_string(),
                });
            }
            Err(e) => {
                errors.push(ValidationError {
                    expr: test.expr.clone(),
                    value_map: value_map.clone(),
                    expected: true,
                    actual: format!("Error: {}", e),
                });
            }
        }
    }

    // Validate false_list - all should evaluate to false
    for value_map in &test.false_list {
        let sqlexpr_map = convert_to_sqlexpr_map(value_map);
        match evaluate(&test.expr, &sqlexpr_map) {
            Ok(false) => {
                // Expected result
            }
            Ok(true) => {
                errors.push(ValidationError {
                    expr: test.expr.clone(),
                    value_map: value_map.clone(),
                    expected: false,
                    actual: "true".to_string(),
                });
            }
            Err(e) => {
                errors.push(ValidationError {
                    expr: test.expr.clone(),
                    value_map: value_map.clone(),
                    expected: false,
                    actual: format!("Error: {}", e),
                });
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Convert our RuntimeValue HashMap to sqlexpr-rust's RuntimeValue HashMap
fn convert_to_sqlexpr_map(map: &HashMap<String, RuntimeValue>) -> HashMap<String, SqlExprRuntimeValue> {
    map.iter()
        .map(|(k, v)| (k.clone(), v.clone().into()))
        .collect()
}
