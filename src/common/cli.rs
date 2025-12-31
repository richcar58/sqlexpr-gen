use crate::common::Language;

/// Parse command-line arguments for language selection
/// Returns Language (defaults to Max if not specified)
pub fn parse_language_arg() -> Result<Language, String> {
    let args: Vec<String> = std::env::args().collect();

    // Look for --lang argument
    for i in 0..args.len() {
        if args[i] == "--lang" {
            // Space-separated format: --lang <value>
            if i + 1 < args.len() {
                let value = &args[i + 1];
                if value.is_empty() {
                    return Err("--lang requires a value. Valid options: max, limited".to_string());
                }
                return Language::from_str(value)
                    .map_err(|e| format!("Invalid --lang argument: {}", e));
            } else {
                return Err("--lang requires a value. Valid options: max, limited".to_string());
            }
        } else if args[i].starts_with("--lang=") {
            // Equals format: --lang=<value>
            match args[i].strip_prefix("--lang=") {
                Some(value) if !value.is_empty() => {
                    return Language::from_str(value)
                        .map_err(|e| format!("Invalid --lang argument: {}", e));
                }
                _ => {
                    return Err("--lang= requires a value after '='. Valid options: max, limited\nUsage: --lang=max or --lang max".to_string());
                }
            }
        }
    }

    // Default to Max if --lang not specified
    Ok(Language::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests can't easily test std::env::args() without complex setup
    // The parsing logic is simple enough to verify through integration tests
}
