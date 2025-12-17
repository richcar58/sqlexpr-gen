use sqlexpr_gen::phase2::loader::SimpleExpressionLoader;
use sqlexpr_gen::phase2::combiner::ExpressionCombiner;
use sqlexpr_gen::phase2::types::{ComplexExpression, COMPLEXITY_CLASSES, TARGET_COUNT_PER_CLASS};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Phase II: Generating complex test expressions...");
    println!("Loading simple expressions from Phase I...");

    // Load simple expressions
    let loader = SimpleExpressionLoader::load()?;
    println!("Loaded simple expressions successfully");

    let mut all_complex_expressions = Vec::new();

    // Generate for each complexity class
    for &complexity in COMPLEXITY_CLASSES {
        println!("\nGenerating {} expressions for complexity class {}...",
                 TARGET_COUNT_PER_CLASS, complexity);

        let count = generate_complexity_class(&loader, complexity, TARGET_COUNT_PER_CLASS)?;
        println!("  Generated {} expressions", count.len());

        all_complex_expressions.extend(count);
    }

    // Write output
    println!("\nWriting {} complex expressions to resources/complex_expressions.json...",
             all_complex_expressions.len());

    let output_path = Path::new("resources/complex_expressions.json");
    let json = serde_json::to_string_pretty(&all_complex_expressions)?;
    fs::write(output_path, json)?;

    println!("Phase II complete!");
    println!("Total expressions generated: {}", all_complex_expressions.len());
    println!("Expected: {} complexity classes × {} expressions = {}",
             COMPLEXITY_CLASSES.len(),
             TARGET_COUNT_PER_CLASS,
             COMPLEXITY_CLASSES.len() * TARGET_COUNT_PER_CLASS);

    Ok(())
}

/// Generate expressions for a specific complexity class
/// complexity = number of AND/OR operators = number of simple expressions - 1
fn generate_complexity_class(
    loader: &SimpleExpressionLoader,
    complexity: usize,
    target_count: usize,
) -> Result<Vec<ComplexExpression>, Box<dyn std::error::Error>> {
    let mut expressions = Vec::new();
    let num_nodes = complexity + 1;  // complexity 1 needs 2 nodes, etc.

    let mut attempts = 0;
    let max_attempts = target_count * 10;  // Allow retries for uniqueness

    while expressions.len() < target_count && attempts < max_attempts {
        attempts += 1;

        // Select non-overlapping simple expressions
        let nodes = match loader.select_non_overlapping(num_nodes) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("  Warning: Failed to select nodes (attempt {}): {}", attempts, e);
                continue;
            }
        };

        // Combine into complex expression
        let combiner = ExpressionCombiner::new(nodes);
        match combiner.generate() {
            Ok(expr) => {
                // Check for duplicate expression strings
                if !expressions.iter().any(|e: &ComplexExpression| e.expr == expr.expr) {
                    expressions.push(expr);
                }
            }
            Err(e) => {
                eprintln!("  Warning: Failed to generate expression (attempt {}): {}", attempts, e);
                continue;
            }
        }

        // Progress indicator every 100 expressions
        if expressions.len() % 100 == 0 && expressions.len() > 0 {
            println!("  Progress: {}/{}", expressions.len(), target_count);
        }
    }

    if expressions.len() < target_count {
        eprintln!("  Warning: Only generated {} expressions (target was {})",
                  expressions.len(), target_count);
    }

    Ok(expressions)
}
