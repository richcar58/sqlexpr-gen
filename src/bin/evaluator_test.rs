use sqlexpr_gen::phase2::types::ComplexExpression;
use sqlexpr_gen::common::output::RuntimeValue;
use sqlexpr_rust::evaluate;
use std::fs;
use std::path::Path;
use std::time::Instant;
use std::collections::{HashMap, BTreeMap};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct FailureRecord {
    expr: String,
    value_map: BTreeMap<String, RuntimeValue>,
    error: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Evaluator Test: Testing complex expressions...");
    println!("Loading complex_expressions.json...");

    // Load complex expressions
    let input_path = Path::new("resources/complex_expressions.json");
    let json = fs::read_to_string(input_path)?;
    let expressions: Vec<ComplexExpression> = serde_json::from_str(&json)?;

    println!("Loaded {} expressions", expressions.len());
    println!("Starting evaluation...\n");

    // Track results
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut failures: Vec<FailureRecord> = Vec::new();

    // Start timing
    let start_time = Instant::now();

    // Evaluate each expression
    for (idx, expr_obj) in expressions.iter().enumerate() {
        // Convert RuntimeValue HashMap to sqlexpr_rust::RuntimeValue HashMap
        let mut bindings = HashMap::new();
        for (var, val) in &expr_obj.value_map {
            let runtime_val = convert_runtime_value(val);
            bindings.insert(var.clone(), runtime_val);
        }

        // Evaluate the expression
        match evaluate(&expr_obj.expr, &bindings) {
            Ok(_result) => {
                success_count += 1;
            }
            Err(e) => {
                failure_count += 1;
                failures.push(FailureRecord {
                    expr: expr_obj.expr.clone(),
                    value_map: expr_obj.value_map.clone(),
                    error: e.to_string(),
                });

                eprintln!("FAILURE #{} (expression #{}): {}", failure_count, idx + 1, e);
                eprintln!("  Expression: {}", &expr_obj.expr[..expr_obj.expr.len().min(100)]);
            }
        }

        // Progress indicator every 1000 expressions
        if (idx + 1) % 1000 == 0 {
            println!("Progress: {}/{} evaluated ({} success, {} failures)",
                     idx + 1, expressions.len(), success_count, failure_count);
        }
    }

    // End timing
    let elapsed = start_time.elapsed();

    // Write results summary
    let results_output = format!(
        "Evaluator Test Results\n\
         ======================\n\
         \n\
         Total expressions evaluated: {}\n\
         Successful evaluations: {}\n\
         Failed evaluations: {}\n\
         Success rate: {:.2}%\n\
         \n\
         Total execution time: {:.3} seconds\n\
         Average time per expression: {:.6} seconds\n\
         Expressions per second: {:.2}\n",
        expressions.len(),
        success_count,
        failure_count,
        (success_count as f64 / expressions.len() as f64) * 100.0,
        elapsed.as_secs_f64(),
        elapsed.as_secs_f64() / expressions.len() as f64,
        expressions.len() as f64 / elapsed.as_secs_f64()
    );

    let output_path = Path::new("evaluator_test.out");
    fs::write(output_path, &results_output)?;
    println!("\n{}", results_output);
    println!("Results written to: evaluator_test.out");

    // Write failures if any
    if !failures.is_empty() {
        let failures_json = serde_json::to_string_pretty(&failures)?;
        let failures_path = Path::new("evaluator_test.failed");
        fs::write(failures_path, failures_json)?;
        println!("Failure details written to: evaluator_test.failed");
    } else {
        println!("No failures - all expressions evaluated successfully!");
    }

    Ok(())
}

/// Convert our RuntimeValue to sqlexpr_rust::RuntimeValue
fn convert_runtime_value(val: &RuntimeValue) -> sqlexpr_rust::RuntimeValue {
    match val {
        RuntimeValue::Integer(i) => sqlexpr_rust::RuntimeValue::Integer(*i),
        RuntimeValue::Float(f) => sqlexpr_rust::RuntimeValue::Float(*f),
        RuntimeValue::String(s) => sqlexpr_rust::RuntimeValue::String(s.clone()),
        RuntimeValue::Boolean(b) => sqlexpr_rust::RuntimeValue::Boolean(*b),
        RuntimeValue::Null => sqlexpr_rust::RuntimeValue::Null,
    }
}
