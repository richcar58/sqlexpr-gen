use serde::{Serialize, Deserialize};
use sqlexpr_rust::RuntimeValue as SqlExprRuntimeValue;
use std::collections::HashMap;

/// Serializable wrapper for sqlexpr-rust's RuntimeValue
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum RuntimeValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl From<RuntimeValue> for SqlExprRuntimeValue {
    fn from(value: RuntimeValue) -> Self {
        match value {
            RuntimeValue::Integer(i) => SqlExprRuntimeValue::Integer(i),
            RuntimeValue::Float(f) => SqlExprRuntimeValue::Float(f),
            RuntimeValue::String(s) => SqlExprRuntimeValue::String(s),
            RuntimeValue::Boolean(b) => SqlExprRuntimeValue::Boolean(b),
            RuntimeValue::Null => SqlExprRuntimeValue::Null,
        }
    }
}

impl From<SqlExprRuntimeValue> for RuntimeValue {
    fn from(value: SqlExprRuntimeValue) -> Self {
        match value {
            SqlExprRuntimeValue::Integer(i) => RuntimeValue::Integer(i),
            SqlExprRuntimeValue::Float(f) => RuntimeValue::Float(f),
            SqlExprRuntimeValue::String(s) => RuntimeValue::String(s),
            SqlExprRuntimeValue::Boolean(b) => RuntimeValue::Boolean(b),
            SqlExprRuntimeValue::Null => RuntimeValue::Null,
        }
    }
}

/// Test expression with true and false value lists
#[derive(Debug, Serialize, Deserialize)]
pub struct TestExpression {
    pub expr: String,
    pub true_list: Vec<HashMap<String, RuntimeValue>>,
    pub false_list: Vec<HashMap<String, RuntimeValue>>,
}

impl TestExpression {
    pub fn new(expr: String, true_list: Vec<HashMap<String, RuntimeValue>>, false_list: Vec<HashMap<String, RuntimeValue>>) -> Self {
        Self { expr, true_list, false_list }
    }
}
