use serde::{Serialize, Deserialize};
use crate::common::output::RuntimeValue;
use std::collections::{HashMap, BTreeMap};

/// Complex expression output format for Phase II
/// Different from Phase I's TestExpression which has true_list/false_list
/// Note: value_map uses BTreeMap to maintain alphabetical key ordering in JSON output
#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexExpression {
    pub expr: String,
    pub value_map: BTreeMap<String, RuntimeValue>,
}

/// Internal representation of a simple expression during construction
#[derive(Debug, Clone)]
pub struct ExpressionNode {
    pub expr: String,                                       // Expression with parens: "(i1 = 7)"
    pub variables: Vec<String>,                             // Variables in expression: ["i1"]
    pub true_list: Vec<HashMap<String, RuntimeValue>>,     // Values that make expr true
    pub false_list: Vec<HashMap<String, RuntimeValue>>,    // Values that make expr false
}

/// Logical operator for combining expressions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperator {
    And,
    Or,
}

impl LogicalOperator {
    pub fn to_str(&self) -> &'static str {
        match self {
            LogicalOperator::And => "AND",
            LogicalOperator::Or => "OR",
        }
    }

    pub fn opposite(&self) -> LogicalOperator {
        match self {
            LogicalOperator::And => LogicalOperator::Or,
            LogicalOperator::Or => LogicalOperator::And,
        }
    }
}

/// Complexity class specification
pub const COMPLEXITY_CLASSES: &[usize] = &[
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    15, 20, 25, 30, 35, 40, 45, 50, 60, 75
];

pub const TARGET_COUNT_PER_CLASS: usize = 500;
