use crate::phase2::types::{ComplexExpression, ExpressionNode, LogicalOperator};
use crate::common::output::RuntimeValue;
use std::collections::{HashMap, BTreeMap};
use rand::Rng;
use rand::seq::SliceRandom;

/// Builds complex expressions from simple expression nodes
pub struct ExpressionCombiner {
    nodes: Vec<ExpressionNode>,
    complexity: usize,
}

impl ExpressionCombiner {
    /// Create a new combiner with the given expression nodes
    /// complexity = number of AND/OR operators = nodes.len() - 1
    pub fn new(nodes: Vec<ExpressionNode>) -> Self {
        let complexity = if nodes.len() > 0 { nodes.len() - 1 } else { 0 };
        Self { nodes, complexity }
    }

    /// Generate a complex expression following Phase II rules
    pub fn generate(&self) -> Result<ComplexExpression, String> {
        if self.nodes.is_empty() {
            return Err("No expression nodes to combine".to_string());
        }

        if self.nodes.len() == 1 {
            // Single node - just use it with a value from true_list
            return self.generate_single_node();
        }

        let mut rng = rand::thread_rng();

        // Step 1: Generate operator sequence following 60/40 probability rule
        let operators = self.generate_operator_sequence(&mut rng);

        // Step 2: Identify OR sequences that need parenthesization
        let or_sequences = self.identify_or_sequences(&operators);

        // Step 3: Build expression string with proper parenthesization
        let expr = self.build_expression_string(&operators, &or_sequences);

        // Step 4: Build value mapping that avoids short-circuiting
        let value_map = self.build_value_mapping(&operators, &or_sequences, &mut rng)?;

        Ok(ComplexExpression { expr, value_map })
    }

    /// Generate single-node expression (complexity 0)
    fn generate_single_node(&self) -> Result<ComplexExpression, String> {
        let node = &self.nodes[0];
        let mut rng = rand::thread_rng();

        // Select random value from true_list
        let values = node.true_list.choose(&mut rng)
            .ok_or("No true values available for single node")?;

        // Convert HashMap to BTreeMap for sorted keys
        let value_map: BTreeMap<String, RuntimeValue> = values.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        Ok(ComplexExpression {
            expr: node.expr.clone(),
            value_map,
        })
    }

    /// Generate operator sequence following the 60/40 probability rule
    /// First operator is random; subsequent operators favor previous (60%) vs opposite (40%)
    fn generate_operator_sequence<R: Rng>(&self, rng: &mut R) -> Vec<LogicalOperator> {
        let mut operators = Vec::new();

        if self.nodes.len() < 2 {
            return operators;
        }

        // First operator is random
        let first_op = if rng.gen_bool(0.5) {
            LogicalOperator::And
        } else {
            LogicalOperator::Or
        };
        operators.push(first_op);

        // Subsequent operators: 60% same as previous, 40% opposite
        for _ in 1..self.complexity {
            let prev_op = *operators.last().unwrap();
            let next_op = if rng.gen_bool(0.6) {
                prev_op  // 60% - same as previous
            } else {
                prev_op.opposite()  // 40% - opposite
            };
            operators.push(next_op);
        }

        operators
    }

    /// Identify sequences of 2+ consecutive OR operators
    /// Returns Vec<(start_index, length)> of OR sequences
    fn identify_or_sequences(&self, operators: &[LogicalOperator]) -> Vec<(usize, usize)> {
        let mut sequences = Vec::new();
        let mut current_start: Option<usize> = None;
        let mut current_length = 0;

        for (i, &op) in operators.iter().enumerate() {
            if op == LogicalOperator::Or {
                if current_start.is_none() {
                    current_start = Some(i);
                    current_length = 1;
                } else {
                    current_length += 1;
                }
            } else {
                // AND found - close any ongoing OR sequence
                if let Some(start) = current_start {
                    if current_length >= 2 {
                        sequences.push((start, current_length));
                    }
                    current_start = None;
                    current_length = 0;
                }
            }
        }

        // Check for OR sequence at end
        if let Some(start) = current_start {
            if current_length >= 2 {
                sequences.push((start, current_length));
            }
        }

        sequences
    }

    /// Build the expression string with proper parenthesization
    fn build_expression_string(
        &self,
        operators: &[LogicalOperator],
        or_sequences: &[(usize, usize)],
    ) -> String {
        if self.nodes.is_empty() {
            return String::new();
        }

        if self.nodes.len() == 1 {
            return self.nodes[0].expr.clone();
        }

        let mut result = String::new();
        let mut next_node = 0;
        let mut op_idx = 0;

        while op_idx < operators.len() {
            // Check if this operator starts an OR sequence
            let or_seq = or_sequences.iter()
                .find(|&&(start, _)| start == op_idx)
                .copied();

            if let Some((_, seq_len)) = or_seq {
                // Process OR sequence: (node OR node OR ... OR node)
                result.push_str("(");
                result.push_str(&self.nodes[next_node].expr);
                next_node += 1;

                // Add seq_len OR operators (connecting seq_len+1 total nodes)
                for _ in 0..seq_len {
                    result.push_str(" OR ");
                    result.push_str(&self.nodes[next_node].expr);
                    next_node += 1;
                }
                result.push_str(")");

                // Skip past the operators in this sequence
                op_idx += seq_len;

                // Add the operator after the sequence (if any)
                if op_idx < operators.len() {
                    result.push_str(" ");
                    result.push_str(operators[op_idx].to_str());
                    result.push_str(" ");
                    op_idx += 1;
                }
            } else {
                // Regular operator (not start of OR sequence)
                result.push_str(&self.nodes[next_node].expr);
                next_node += 1;
                result.push_str(" ");
                result.push_str(operators[op_idx].to_str());
                result.push_str(" ");
                op_idx += 1;
            }
        }

        // Add final node
        if next_node < self.nodes.len() {
            result.push_str(&self.nodes[next_node].expr);
        }

        result
    }

    /// Build value mapping that avoids short-circuiting
    /// Rules:
    /// - AND clauses: select from true_list
    /// - OR sequences: first (len-1) from false_list, last from true_list
    /// - Standalone OR: can use either true or false (randomly chosen)
    fn build_value_mapping<R: Rng>(
        &self,
        operators: &[LogicalOperator],
        or_sequences: &[(usize, usize)],
        rng: &mut R,
    ) -> Result<BTreeMap<String, RuntimeValue>, String> {
        let mut value_map = BTreeMap::new();

        // Build a mapping of which nodes are in OR sequences and their positions
        let mut or_seq_nodes: HashMap<usize, (usize, usize)> = HashMap::new(); // node_idx -> (seq_start, seq_len)
        for &(seq_start_op, seq_len) in or_sequences {
            // OR sequence starting at operator seq_start_op with seq_len operators
            // connects nodes [seq_start_op, seq_start_op+1, ..., seq_start_op+seq_len]
            for i in 0..=seq_len {
                or_seq_nodes.insert(seq_start_op + i, (seq_start_op, seq_len));
            }
        }

        // Process each node
        for (node_idx, node) in self.nodes.iter().enumerate() {
            let use_true = if let Some(&(seq_start, seq_len)) = or_seq_nodes.get(&node_idx) {
                // Node is in an OR sequence
                // Only the last node (seq_start + seq_len) should be true
                node_idx == seq_start + seq_len
            } else if node_idx == self.nodes.len() - 1 {
                // Last node: always true to make overall expression true
                true
            } else {
                // Check the operator AFTER this node (at index node_idx)
                // This determines if this node could cause short-circuiting
                if node_idx < operators.len() {
                    // To avoid short-circuiting:
                    // - If followed by AND: node must be true (so AND doesn't short-circuit)
                    // - If followed by OR: node must be false (so OR doesn't short-circuit)
                    operators[node_idx] == LogicalOperator::And
                } else {
                    // Safety fallback (shouldn't happen)
                    true
                }
            };

            // Select random value from appropriate list
            let values = if use_true {
                node.true_list.choose(rng)
                    .ok_or_else(|| format!("No true values for node {}", node_idx))?
            } else {
                node.false_list.choose(rng)
                    .ok_or_else(|| format!("No false values for node {}", node_idx))?
            };

            // Merge values into map
            for (var, val) in values {
                value_map.insert(var.clone(), val.clone());
            }
        }

        Ok(value_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_node(expr: &str, var: &str) -> ExpressionNode {
        let true_val = HashMap::from([(var.to_string(), RuntimeValue::Integer(1))]);
        let false_val = HashMap::from([(var.to_string(), RuntimeValue::Integer(0))]);

        ExpressionNode {
            expr: format!("({})", expr),
            variables: vec![var.to_string()],
            true_list: vec![true_val],
            false_list: vec![false_val],
        }
    }

    #[test]
    fn test_operator_sequence_generation() {
        let nodes = vec![
            create_test_node("i1 = 1", "i1"),
            create_test_node("i2 = 2", "i2"),
            create_test_node("i3 = 3", "i3"),
        ];

        let combiner = ExpressionCombiner::new(nodes);
        assert_eq!(combiner.complexity, 2);

        // Generate multiple sequences to verify randomness
        for _ in 0..10 {
            let ops = combiner.generate_operator_sequence(&mut rand::thread_rng());
            assert_eq!(ops.len(), 2);
        }
    }

    #[test]
    fn test_or_sequence_identification() {
        let combiner = ExpressionCombiner::new(vec![]);

        // Test sequence: OR, OR, AND, OR
        let ops = vec![
            LogicalOperator::Or,
            LogicalOperator::Or,
            LogicalOperator::And,
            LogicalOperator::Or,
        ];

        let sequences = combiner.identify_or_sequences(&ops);
        assert_eq!(sequences.len(), 1);
        assert_eq!(sequences[0], (0, 2));  // First two are OR sequence
    }

    #[test]
    fn test_single_node_generation() {
        let node = create_test_node("i1 = 1", "i1");
        let combiner = ExpressionCombiner::new(vec![node]);

        let result = combiner.generate().unwrap();
        assert_eq!(result.expr, "(i1 = 1)");
        assert!(result.value_map.contains_key("i1"));
    }
}
