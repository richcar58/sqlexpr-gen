mod templates;
mod generator;
mod validator;
mod output;

use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating Phase I test expressions...");

    // 1. Load all templates
    let all_templates = templates::get_all_templates();
    println!("Loaded {} templates", all_templates.len());

    // 2. Generate expressions from templates
    let mut all_tests = Vec::new();
    for template in &all_templates {
        let test = generator::generate_from_template(template);
        all_tests.push(test);
    }

    println!("Generated {} test expressions", all_tests.len());

    // 3. Validate all expressions
    println!("Validating expressions...");
    let mut validation_errors = Vec::new();
    let mut validated_count = 0;

    for (i, test) in all_tests.iter().enumerate() {
        if let Err(errors) = validator::validate_expression(test) {
            eprintln!("\nValidation failed for expression #{}: {}", i + 1, test.expr);
            for error in &errors {
                eprintln!("  {}", error);
            }
            validation_errors.extend(errors);
        } else {
            validated_count += 1;
        }
    }

    println!("Validated {}/{} expressions successfully", validated_count, all_tests.len());

    if !validation_errors.is_empty() {
        eprintln!("\n❌ Validation failed with {} total errors", validation_errors.len());
        return Err("Validation failed".into());
    }

    println!("✓ All expressions validated successfully!");

    // 4. Serialize to JSON
    let json = serde_json::to_string_pretty(&all_tests)?;

    // 5. Write to resources/simple_expressions.json
    let output_path = Path::new("resources/simple_expressions.json");
    fs::create_dir_all("resources")?;
    fs::write(output_path, json)?;

    println!("✓ Wrote {} tests to {}", all_tests.len(), output_path.display());

    Ok(())
}
