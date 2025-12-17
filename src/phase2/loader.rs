use crate::common::output::TestExpression;
use crate::phase2::types::ExpressionNode;
use std::fs;
use std::path::Path;
use std::collections::HashSet;
use rand::seq::SliceRandom;
use regex::Regex;

/// Loader for simple expressions from Phase I output
pub struct SimpleExpressionLoader {
    expressions: Vec<TestExpression>,
    var_regex: Regex,
}

impl SimpleExpressionLoader {
    /// Load simple expressions from resources/simple_expressions.json
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new("resources/simple_expressions.json");
        let json = fs::read_to_string(path)?;
        let expressions: Vec<TestExpression> = serde_json::from_str(&json)?;

        let var_regex = Regex::new(r"\b[ifsb]\d+\b")?;

        Ok(Self {
            expressions,
            var_regex,
        })
    }

    /// Select N random simple expressions ensuring NO variable name overlap
    /// Retries up to 1000 times if overlap detected
    pub fn select_non_overlapping(&self, count: usize) -> Result<Vec<ExpressionNode>, String> {
        if count > self.expressions.len() {
            return Err(format!(
                "Cannot select {} expressions from {} available",
                count, self.expressions.len()
            ));
        }

        let max_retries = 1000;
        let mut rng = rand::thread_rng();

        for _ in 0..max_retries {
            // Select random indices
            let mut indices: Vec<usize> = (0..self.expressions.len()).collect();
            indices.shuffle(&mut rng);
            let selected_indices = &indices[..count];

            // Check for variable overlaps
            let mut all_vars = HashSet::new();
            let mut has_overlap = false;

            let candidates: Vec<ExpressionNode> = selected_indices.iter()
                .map(|&i| self.to_expression_node(&self.expressions[i]))
                .collect();

            for node in &candidates {
                for var in &node.variables {
                    if !all_vars.insert(var.clone()) {
                        has_overlap = true;
                        break;
                    }
                }
                if has_overlap {
                    break;
                }
            }

            if !has_overlap {
                return Ok(candidates);
            }
        }

        Err(format!(
            "Could not find {} non-overlapping expressions after {} retries",
            count, max_retries
        ))
    }

    /// Convert TestExpression to ExpressionNode
    fn to_expression_node(&self, test: &TestExpression) -> ExpressionNode {
        let variables = self.extract_variables(&test.expr);

        ExpressionNode {
            expr: format!("({})", test.expr),  // Wrap in parentheses
            variables,
            true_list: test.true_list.clone(),
            false_list: test.false_list.clone(),
        }
    }

    /// Extract all variables from an expression using regex
    fn extract_variables(&self, expr: &str) -> Vec<String> {
        let mut vars: Vec<String> = self.var_regex
            .find_iter(expr)
            .map(|m| m.as_str().to_string())
            .collect();

        vars.sort();
        vars.dedup();
        vars
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_extraction() {
        let loader = SimpleExpressionLoader {
            expressions: vec![],
            var_regex: Regex::new(r"\b[ifsb]\d+\b").unwrap(),
        };

        let vars = loader.extract_variables("i1 = 7 AND s2 LIKE '%test'");
        assert_eq!(vars, vec!["i1", "s2"]);

        let vars2 = loader.extract_variables("i3 > i4");
        assert_eq!(vars2, vec!["i3", "i4"]);
    }
}
