/// Supported expression language levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// Full unrestricted language (all operators, all type combinations)
    Max,
    /// Restricted subset excluding:
    /// - String operands in: >, >=, <, <=, BETWEEN
    /// - Numeric operands in: IN clauses
    Limited,
}

impl Language {
    /// Parse from string (for CLI parsing)
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "max" => Ok(Language::Max),
            "limited" => Ok(Language::Limited),
            _ => Err(format!("Unknown language '{}'. Valid: max, limited", s)),
        }
    }

    /// Get file suffix for this language
    pub fn file_suffix(&self) -> &'static str {
        match self {
            Language::Max => "max",
            Language::Limited => "limited",
        }
    }

    /// Get resource file path for simple expressions
    pub fn simple_expressions_path(&self) -> String {
        format!("resources/simple_expressions-{}.json", self.file_suffix())
    }

    /// Get resource file path for complex expressions
    pub fn complex_expressions_path(&self) -> String {
        format!("resources/complex_expressions-{}.json", self.file_suffix())
    }

    /// Get evaluator test output path
    pub fn evaluator_output_path(&self) -> String {
        format!("evaluator_test-{}.out", self.file_suffix())
    }

    /// Get evaluator test failure path
    pub fn evaluator_failure_path(&self) -> String {
        format!("evaluator_test-{}.failed", self.file_suffix())
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::Max  // Default if --lang not specified
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_str() {
        assert_eq!(Language::from_str("max").unwrap(), Language::Max);
        assert_eq!(Language::from_str("MAX").unwrap(), Language::Max);
        assert_eq!(Language::from_str("limited").unwrap(), Language::Limited);
        assert_eq!(Language::from_str("LIMITED").unwrap(), Language::Limited);
        assert!(Language::from_str("invalid").is_err());
    }

    #[test]
    fn test_file_paths() {
        let max = Language::Max;
        assert_eq!(max.simple_expressions_path(), "resources/simple_expressions-max.json");
        assert_eq!(max.complex_expressions_path(), "resources/complex_expressions-max.json");
        assert_eq!(max.evaluator_output_path(), "evaluator_test-max.out");
        assert_eq!(max.evaluator_failure_path(), "evaluator_test-max.failed");

        let limited = Language::Limited;
        assert_eq!(limited.simple_expressions_path(), "resources/simple_expressions-limited.json");
    }

    #[test]
    fn test_default() {
        assert_eq!(Language::default(), Language::Max);
    }
}
