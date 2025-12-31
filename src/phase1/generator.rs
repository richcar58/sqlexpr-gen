use super::templates::*;
use crate::common::output::{TestExpression, RuntimeValue};
use std::collections::HashMap;

/// Generate a test expression from a template
pub fn generate_from_template(template: &Template) -> TestExpression {
    match template {
        Template::Comparison(t) => generate_comparison(t),
        Template::Like(t) => generate_like(t),
        Template::Between(t) => generate_between(t),
        Template::In(t) => generate_in(t),
        Template::IsNull(t) => generate_is_null(t),
        Template::Boolean(t) => generate_boolean(t),
        Template::ArithmeticComparison(t) => generate_arithmetic_comparison(t),
    }
}

/// Generate comparison expression
fn generate_comparison(template: &ComparisonTemplate) -> TestExpression {
    let expr = format!("{} {} {}",
        template.left.to_string(),
        template.operator,
        template.right.to_string());

    let (true_list, false_list) = match (&template.left, &template.right) {
        (Pattern::Variable(var), Pattern::Literal(lit)) => {
            generate_values_var_op_lit(var, template.operator, lit)
        }
        (Pattern::Literal(lit), Pattern::Variable(var)) => {
            generate_values_lit_op_var(lit, template.operator, var)
        }
        (Pattern::Variable(var1), Pattern::Variable(var2)) => {
            generate_values_var_op_var(var1, var2, template.operator, &template.value_type)
        }
        _ => panic!("Unexpected pattern combination in comparison template"),
    };

    TestExpression::new(expr, true_list, false_list)
}

/// Generate values for variable OP literal (e.g., i1 > 7)
fn generate_values_var_op_lit(var: &str, op: &str, lit: &LiteralValue) -> (Vec<HashMap<String, RuntimeValue>>, Vec<HashMap<String, RuntimeValue>>) {
    match lit {
        LiteralValue::Integer(n) => {
            let true_vals = match op {
                "=" => vec![*n],  // Only one value makes equality true
                "<>" | "!=" => vec![n - 1, n - 2, n + 1, n + 2, n - 10],
                ">" => vec![n + 1, n + 2, n + 5, n + 10, n + 100],
                ">=" => vec![*n, n + 1, n + 2, n + 5, n + 10],
                "<" => vec![n - 1, n - 2, n - 5, n - 10, n - 100],
                "<=" => vec![*n, n - 1, n - 2, n - 5, n - 10],
                _ => panic!("Unknown operator: {}", op),
            };
            let false_vals = match op {
                "=" => vec![n - 1, n - 2, n + 1, n + 2, n - 10],
                "<>" | "!=" => vec![*n],  // Only one value makes inequality false
                ">" => vec![*n, n - 1, n - 2, n - 5, n - 10],
                ">=" => vec![n - 1, n - 2, n - 5, n - 10, n - 100],
                "<" => vec![*n, n + 1, n + 2, n + 5, n + 10],
                "<=" => vec![n + 1, n + 2, n + 5, n + 10, n + 100],
                _ => panic!("Unknown operator: {}", op),
            };
            (
                true_vals.into_iter().map(|v| {
                    let mut map = HashMap::new();
                    map.insert(var.to_string(), RuntimeValue::Integer(v));
                    map
                }).collect(),
                false_vals.into_iter().map(|v| {
                    let mut map = HashMap::new();
                    map.insert(var.to_string(), RuntimeValue::Integer(v));
                    map
                }).collect(),
            )
        }
        LiteralValue::Float(n) => {
            let true_vals = match op {
                "=" => vec![*n],  // Only one value makes equality true
                "<>" | "!=" => vec![n - 1.0, n - 2.0, n + 1.0, n + 2.0, n - 10.0],
                ">" => vec![n + 1.0, n + 2.0, n + 5.0, n + 10.0, n + 100.0],
                ">=" => vec![*n, n + 1.0, n + 2.0, n + 5.0, n + 10.0],
                "<" => vec![n - 1.0, n - 2.0, n - 5.0, n - 10.0, n - 100.0],
                "<=" => vec![*n, n - 1.0, n - 2.0, n - 5.0, n - 10.0],
                _ => panic!("Unknown operator: {}", op),
            };
            let false_vals = match op {
                "=" => vec![n - 1.0, n - 2.0, n + 1.0, n + 2.0, n - 10.0],
                "<>" | "!=" => vec![*n],  // Only one value makes inequality false
                ">" => vec![*n, n - 1.0, n - 2.0, n - 5.0, n - 10.0],
                ">=" => vec![n - 1.0, n - 2.0, n - 5.0, n - 10.0, n - 100.0],
                "<" => vec![*n, n + 1.0, n + 2.0, n + 5.0, n + 10.0],
                "<=" => vec![n + 1.0, n + 2.0, n + 5.0, n + 10.0, n + 100.0],
                _ => panic!("Unknown operator: {}", op),
            };
            (
                true_vals.into_iter().map(|v| {
                    let mut map = HashMap::new();
                    map.insert(var.to_string(), RuntimeValue::Float(v));
                    map
                }).collect(),
                false_vals.into_iter().map(|v| {
                    let mut map = HashMap::new();
                    map.insert(var.to_string(), RuntimeValue::Float(v));
                    map
                }).collect(),
            )
        }
        LiteralValue::String(s) => {
            // Generate strings dynamically based on comparison with s
            let greater_strings = generate_greater_strings(s, 5);
            let lesser_strings = generate_lesser_strings(s, 5);
            let different_strings = generate_different_strings(s, 5);

            let true_vals = match op {
                "=" => vec![s.clone()],  // Only one value makes equality true
                "<>" | "!=" => different_strings.clone(),
                ">" => greater_strings.clone(),
                ">=" => {
                    let mut vals = vec![s.clone()];
                    vals.extend(greater_strings.clone().into_iter().take(4));
                    vals
                }
                "<" => lesser_strings.clone(),
                "<=" => {
                    let mut vals = vec![s.clone()];
                    vals.extend(lesser_strings.clone().into_iter().take(4));
                    vals
                }
                _ => panic!("Unknown operator: {}", op),
            };
            let false_vals = match op {
                "=" => different_strings,
                "<>" | "!=" => vec![s.clone()],  // Only one value makes inequality false
                ">" => {
                    let mut vals = vec![s.clone()];
                    vals.extend(lesser_strings.into_iter().take(4));
                    vals
                }
                ">=" => lesser_strings,
                "<" => {
                    let mut vals = vec![s.clone()];
                    vals.extend(greater_strings.into_iter().take(4));
                    vals
                }
                "<=" => greater_strings,
                _ => panic!("Unknown operator: {}", op),
            };
            (
                true_vals.into_iter().map(|v| {
                    let mut map = HashMap::new();
                    map.insert(var.to_string(), RuntimeValue::String(v));
                    map
                }).collect(),
                false_vals.into_iter().map(|v| {
                    let mut map = HashMap::new();
                    map.insert(var.to_string(), RuntimeValue::String(v));
                    map
                }).collect(),
            )
        }
    }
}

/// Generate values for literal OP variable (e.g., -3 > i2)
fn generate_values_lit_op_var(lit: &LiteralValue, op: &str, var: &str) -> (Vec<HashMap<String, RuntimeValue>>, Vec<HashMap<String, RuntimeValue>>) {
    // Flip the operator and reuse var_op_lit logic
    let flipped_op = match op {
        "=" => "=",
        "<>" | "!=" => "<>",
        ">" => "<",
        ">=" => "<=",
        "<" => ">",
        "<=" => ">=",
        _ => panic!("Unknown operator: {}", op),
    };
    generate_values_var_op_lit(var, flipped_op, lit)
}

/// Generate values for variable OP variable (e.g., i3 > i4)
fn generate_values_var_op_var(var1: &str, var2: &str, op: &str, value_type: &ValueType) -> (Vec<HashMap<String, RuntimeValue>>, Vec<HashMap<String, RuntimeValue>>) {
    match value_type {
        ValueType::Integer => {
            let true_pairs = match op {
                "=" => vec![(5, 5), (10, 10), (0, 0), (-5, -5), (100, 100)],
                "<>" | "!=" => vec![(5, 10), (10, 5), (0, 1), (-5, 5), (100, 200)],
                ">" => vec![(10, 5), (5, 0), (0, -5), (100, 50), (1, -1)],
                ">=" => vec![(10, 5), (5, 5), (0, 0), (100, 50), (1, -1)],
                "<" => vec![(5, 10), (0, 5), (-5, 0), (50, 100), (-1, 1)],
                "<=" => vec![(5, 10), (5, 5), (0, 0), (50, 100), (-1, 1)],
                _ => panic!("Unknown operator: {}", op),
            };
            let false_pairs = match op {
                "=" => vec![(5, 10), (10, 5), (0, 1), (-5, 5), (100, 200)],
                "<>" | "!=" => vec![(5, 5), (10, 10), (0, 0), (-5, -5), (100, 100)],
                ">" => vec![(5, 10), (5, 5), (0, 5), (50, 100), (-1, 1)],
                ">=" => vec![(5, 10), (0, 5), (-5, 0), (50, 100), (-1, 1)],
                "<" => vec![(10, 5), (5, 5), (5, 0), (100, 50), (1, -1)],
                "<=" => vec![(10, 5), (5, 0), (0, -5), (100, 50), (1, -1)],
                _ => panic!("Unknown operator: {}", op),
            };
            (
                true_pairs.into_iter().map(|(v1, v2)| {
                    let mut map = HashMap::new();
                    map.insert(var1.to_string(), RuntimeValue::Integer(v1));
                    map.insert(var2.to_string(), RuntimeValue::Integer(v2));
                    map
                }).collect(),
                false_pairs.into_iter().map(|(v1, v2)| {
                    let mut map = HashMap::new();
                    map.insert(var1.to_string(), RuntimeValue::Integer(v1));
                    map.insert(var2.to_string(), RuntimeValue::Integer(v2));
                    map
                }).collect(),
            )
        }
        ValueType::Float => {
            let true_pairs = match op {
                "=" => vec![(5.0, 5.0), (10.0, 10.0), (0.0, 0.0), (-5.5, -5.5), (100.1, 100.1)],
                "<>" | "!=" => vec![(5.0, 10.0), (10.0, 5.0), (0.0, 1.0), (-5.5, 5.5), (100.1, 200.2)],
                ">" => vec![(10.0, 5.0), (5.5, 0.0), (0.0, -5.0), (100.0, 50.0), (1.1, -1.1)],
                ">=" => vec![(10.0, 5.0), (5.0, 5.0), (0.0, 0.0), (100.0, 50.0), (1.1, -1.1)],
                "<" => vec![(5.0, 10.0), (0.0, 5.5), (-5.0, 0.0), (50.0, 100.0), (-1.1, 1.1)],
                "<=" => vec![(5.0, 10.0), (5.0, 5.0), (0.0, 0.0), (50.0, 100.0), (-1.1, 1.1)],
                _ => panic!("Unknown operator: {}", op),
            };
            let false_pairs = match op {
                "=" => vec![(5.0, 10.0), (10.0, 5.0), (0.0, 1.0), (-5.5, 5.5), (100.1, 200.2)],
                "<>" | "!=" => vec![(5.0, 5.0), (10.0, 10.0), (0.0, 0.0), (-5.5, -5.5), (100.1, 100.1)],
                ">" => vec![(5.0, 10.0), (5.0, 5.0), (0.0, 5.0), (50.0, 100.0), (-1.1, 1.1)],
                ">=" => vec![(5.0, 10.0), (0.0, 5.5), (-5.0, 0.0), (50.0, 100.0), (-1.1, 1.1)],
                "<" => vec![(10.0, 5.0), (5.0, 5.0), (5.0, 0.0), (100.0, 50.0), (1.1, -1.1)],
                "<=" => vec![(10.0, 5.0), (5.5, 0.0), (0.0, -5.0), (100.0, 50.0), (1.1, -1.1)],
                _ => panic!("Unknown operator: {}", op),
            };
            (
                true_pairs.into_iter().map(|(v1, v2)| {
                    let mut map = HashMap::new();
                    map.insert(var1.to_string(), RuntimeValue::Float(v1));
                    map.insert(var2.to_string(), RuntimeValue::Float(v2));
                    map
                }).collect(),
                false_pairs.into_iter().map(|(v1, v2)| {
                    let mut map = HashMap::new();
                    map.insert(var1.to_string(), RuntimeValue::Float(v1));
                    map.insert(var2.to_string(), RuntimeValue::Float(v2));
                    map
                }).collect(),
            )
        }
        ValueType::String => {
            let true_pairs = match op {
                "=" => vec![("apple", "apple"), ("banana", "banana"), ("cat", "cat"), ("dog", "dog"), ("egg", "egg")],
                "<>" | "!=" => vec![("apple", "banana"), ("cat", "dog"), ("egg", "fox"), ("grape", "honey"), ("ice", "juice")],
                ">" => vec![("banana", "apple"), ("dog", "cat"), ("fox", "egg"), ("honey", "grape"), ("juice", "ice")],
                ">=" => vec![("banana", "apple"), ("dog", "cat"), ("dog", "dog"), ("honey", "grape"), ("juice", "ice")],
                "<" => vec![("apple", "banana"), ("cat", "dog"), ("egg", "fox"), ("grape", "honey"), ("ice", "juice")],
                "<=" => vec![("apple", "banana"), ("cat", "dog"), ("dog", "dog"), ("grape", "honey"), ("ice", "juice")],
                _ => panic!("Unknown operator: {}", op),
            };
            let false_pairs = match op {
                "=" => vec![("apple", "banana"), ("cat", "dog"), ("egg", "fox"), ("grape", "honey"), ("ice", "juice")],
                "<>" | "!=" => vec![("apple", "apple"), ("banana", "banana"), ("cat", "cat"), ("dog", "dog"), ("egg", "egg")],
                ">" => vec![("apple", "banana"), ("cat", "dog"), ("dog", "dog"), ("grape", "honey"), ("ice", "juice")],
                ">=" => vec![("apple", "banana"), ("cat", "dog"), ("egg", "fox"), ("grape", "honey"), ("ice", "juice")],
                "<" => vec![("banana", "apple"), ("dog", "cat"), ("dog", "dog"), ("honey", "grape"), ("juice", "ice")],
                "<=" => vec![("banana", "apple"), ("dog", "cat"), ("fox", "egg"), ("honey", "grape"), ("juice", "ice")],
                _ => panic!("Unknown operator: {}", op),
            };
            (
                true_pairs.into_iter().map(|(v1, v2)| {
                    let mut map = HashMap::new();
                    map.insert(var1.to_string(), RuntimeValue::String(v1.to_string()));
                    map.insert(var2.to_string(), RuntimeValue::String(v2.to_string()));
                    map
                }).collect(),
                false_pairs.into_iter().map(|(v1, v2)| {
                    let mut map = HashMap::new();
                    map.insert(var1.to_string(), RuntimeValue::String(v1.to_string()));
                    map.insert(var2.to_string(), RuntimeValue::String(v2.to_string()));
                    map
                }).collect(),
            )
        }
        ValueType::Boolean => {
            panic!("Boolean comparisons not supported in this phase");
        }
    }
}

/// Generate LIKE expression
fn generate_like(template: &LikeTemplate) -> TestExpression {
    let expr = if template.not {
        format!("{} NOT LIKE '{}'", template.variable, template.pattern)
    } else {
        format!("{} LIKE '{}'", template.variable, template.pattern)
    };

    let (match_vals, no_match_vals) = generate_like_values(&template.pattern);

    let (true_list, false_list) = if template.not {
        (no_match_vals, match_vals)
    } else {
        (match_vals, no_match_vals)
    };

    let true_list = true_list.into_iter().map(|v| {
        let mut map = HashMap::new();
        map.insert(template.variable.clone(), RuntimeValue::String(v));
        map
    }).collect();

    let false_list = false_list.into_iter().map(|v| {
        let mut map = HashMap::new();
        map.insert(template.variable.clone(), RuntimeValue::String(v));
        map
    }).collect();

    TestExpression::new(expr, true_list, false_list)
}

/// Generate values that match and don't match a LIKE pattern
fn generate_like_values(pattern: &str) -> (Vec<String>, Vec<String>) {
    let match_vals = match pattern {
        "%sometext" => vec!["sometext", "xsometext", "yysometext", "123sometext", "___sometext"],
        "_sometext" => vec!["asometext", "bsometext", "1sometext", "xsometext", "Zsometext"],
        "sometext%" => vec!["sometext", "sometextx", "sometextyy", "sometext123", "sometext___"],
        "sometext_" => vec!["sometexta", "sometextb", "sometext1", "sometextx", "sometextZ"],
        "some%text" => vec!["sometext", "somextext", "someyytext", "some123text", "some___text"],
        "some_text" => vec!["someAtext", "someBtext", "some1text", "somextext", "someZtext"],
        "some_text%" => vec!["someAtextx", "someBtext", "some1text___", "somextextabc", "someZtext123"],
        "some%text_" => vec!["sometexta", "someXtextb", "some123text1", "someyytextx", "some__textZ"],
        _ => vec!["match1", "match2", "match3", "match4", "match5"],
    };

    let no_match_vals = vec!["nomatch1", "nomatch2", "nomatch3", "nomatch4", "nomatch5"];

    (
        match_vals.into_iter().map(|s| s.to_string()).collect(),
        no_match_vals.into_iter().map(|s| s.to_string()).collect(),
    )
}

/// Generate BETWEEN expression
fn generate_between(template: &BetweenTemplate) -> TestExpression {
    let lower_str = template.lower.to_string();
    let upper_str = template.upper.to_string();

    let expr = if template.not {
        format!("{} NOT BETWEEN {} AND {}", template.variable, lower_str, upper_str)
    } else {
        format!("{} BETWEEN {} AND {}", template.variable, lower_str, upper_str)
    };

    let (inside_vals, outside_vals) = match (&template.lower, &template.upper) {
        (LiteralValue::Integer(low), LiteralValue::Integer(high)) => {
            let inside = vec![*low, *high, (low + high) / 2, low + 1, high - 1];
            let outside = vec![low - 1, low - 10, high + 1, high + 10, low - 100];
            (
                inside.into_iter().map(RuntimeValue::Integer).collect::<Vec<RuntimeValue>>(),
                outside.into_iter().map(RuntimeValue::Integer).collect::<Vec<RuntimeValue>>(),
            )
        }
        (LiteralValue::Float(low), LiteralValue::Float(high)) => {
            let inside = vec![*low, *high, (low + high) / 2.0, low + 1.0, high - 1.0];
            let outside = vec![low - 1.0, low - 10.0, high + 1.0, high + 10.0, low - 100.0];
            (
                inside.into_iter().map(RuntimeValue::Float).collect::<Vec<RuntimeValue>>(),
                outside.into_iter().map(RuntimeValue::Float).collect::<Vec<RuntimeValue>>(),
            )
        }
        (LiteralValue::String(low), LiteralValue::String(high)) => {
            // Generate values inside the range [low, high]
            let mut inside = vec![low.clone(), high.clone()];

            // Generate middle values
            if low.len() == high.len() && low.len() == 2 {
                // For two-char strings, generate some in-between values
                let low_bytes = low.as_bytes();
                let high_bytes = high.as_bytes();
                if low_bytes[0] == high_bytes[0] {
                    // Same first character, vary second
                    for i in (low_bytes[1]+1..high_bytes[1]).take(3) {
                        inside.push(format!("{}{}", low_bytes[0] as char, i as char));
                    }
                } else {
                    // Different first character
                    for i in (low_bytes[0]+1..high_bytes[0]).take(3) {
                        inside.push(format!("{}a", i as char));
                    }
                }
            }

            // Remove duplicates and truncate
            inside.sort();
            inside.dedup();
            inside.truncate(5);

            // Generate values outside the range [low, high]
            let before_low = generate_lesser_strings(low, 3);
            let after_high = generate_greater_strings(high, 3);
            let mut outside = before_low;
            outside.extend(after_high);

            // Remove duplicates
            outside.sort();
            outside.dedup();
            outside.truncate(5);

            (
                inside.into_iter().map(RuntimeValue::String).collect::<Vec<RuntimeValue>>(),
                outside.into_iter().map(RuntimeValue::String).collect::<Vec<RuntimeValue>>(),
            )
        }
        _ => panic!("Mismatched types in BETWEEN bounds"),
    };

    let (true_list, false_list) = if template.not {
        (outside_vals, inside_vals)
    } else {
        (inside_vals, outside_vals)
    };

    let true_list = true_list.into_iter().map(|v| {
        let mut map = HashMap::new();
        map.insert(template.variable.clone(), v);
        map
    }).collect();

    let false_list = false_list.into_iter().map(|v| {
        let mut map = HashMap::new();
        map.insert(template.variable.clone(), v);
        map
    }).collect();

    TestExpression::new(expr, true_list, false_list)
}

/// Generate IN expression
fn generate_in(template: &InTemplate) -> TestExpression {
    let values_str = template.values.iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let expr = if template.not {
        format!("{} NOT IN ({})", template.variable, values_str)
    } else {
        format!("{} IN ({})", template.variable, values_str)
    };

    // Use the actual IN list values (no padding to avoid duplicates)
    let in_vals: Vec<RuntimeValue> = template.values.iter().map(|lit| {
        match lit {
            LiteralValue::Integer(i) => RuntimeValue::Integer(*i),
            LiteralValue::Float(f) => RuntimeValue::Float(*f),
            LiteralValue::String(s) => RuntimeValue::String(s.clone()),
        }
    }).collect();

    let not_in_vals: Vec<RuntimeValue> = match template.value_type {
        ValueType::Integer => vec![
            RuntimeValue::Integer(999),
            RuntimeValue::Integer(-999),
            RuntimeValue::Integer(0),
            RuntimeValue::Integer(500),
            RuntimeValue::Integer(-500),
        ],
        ValueType::Float => vec![
            RuntimeValue::Float(999.9),
            RuntimeValue::Float(-999.9),
            RuntimeValue::Float(0.5),
            RuntimeValue::Float(500.5),
            RuntimeValue::Float(-500.5),
        ],
        ValueType::String => vec![
            RuntimeValue::String("notinlist1".to_string()),
            RuntimeValue::String("notinlist2".to_string()),
            RuntimeValue::String("xxx".to_string()),
            RuntimeValue::String("yyy".to_string()),
            RuntimeValue::String("zzz".to_string()),
        ],
        ValueType::Boolean => panic!("Boolean IN not supported"),
    };

    let (true_list, false_list) = if template.not {
        (not_in_vals, in_vals)
    } else {
        (in_vals, not_in_vals)
    };

    let true_list = true_list.into_iter().map(|v| {
        let mut map = HashMap::new();
        map.insert(template.variable.clone(), v);
        map
    }).collect();

    let false_list = false_list.into_iter().map(|v| {
        let mut map = HashMap::new();
        map.insert(template.variable.clone(), v);
        map
    }).collect();

    TestExpression::new(expr, true_list, false_list)
}

/// Generate IS NULL expression
fn generate_is_null(template: &IsNullTemplate) -> TestExpression {
    let expr = if template.not {
        format!("{} IS NOT NULL", template.variable)
    } else {
        format!("{} IS NULL", template.variable)
    };

    let null_vals = vec![RuntimeValue::Null];  // Only one NULL value needed

    let not_null_vals: Vec<RuntimeValue> = match template.value_type {
        ValueType::Integer => vec![
            RuntimeValue::Integer(0),
            RuntimeValue::Integer(1),
            RuntimeValue::Integer(-1),
            RuntimeValue::Integer(100),
            RuntimeValue::Integer(-100),
        ],
        ValueType::Float => vec![
            RuntimeValue::Float(0.0),
            RuntimeValue::Float(1.0),
            RuntimeValue::Float(-1.0),
            RuntimeValue::Float(100.5),
            RuntimeValue::Float(-100.5),
        ],
        ValueType::String => vec![
            RuntimeValue::String("".to_string()),
            RuntimeValue::String("a".to_string()),
            RuntimeValue::String("test".to_string()),
            RuntimeValue::String("hello".to_string()),
            RuntimeValue::String("world".to_string()),
        ],
        ValueType::Boolean => vec![
            RuntimeValue::Boolean(true),
            RuntimeValue::Boolean(false),
            RuntimeValue::Boolean(true),
            RuntimeValue::Boolean(false),
            RuntimeValue::Boolean(true),
        ],
    };

    let (true_list, false_list) = if template.not {
        (not_null_vals, null_vals)
    } else {
        (null_vals, not_null_vals)
    };

    let true_list = true_list.into_iter().map(|v| {
        let mut map = HashMap::new();
        map.insert(template.variable.clone(), v);
        map
    }).collect();

    let false_list = false_list.into_iter().map(|v| {
        let mut map = HashMap::new();
        map.insert(template.variable.clone(), v);
        map
    }).collect();

    TestExpression::new(expr, true_list, false_list)
}

/// Generate boolean expression
fn generate_boolean(template: &BooleanTemplate) -> TestExpression {
    let expr = if template.not {
        format!("NOT {}", template.variable)
    } else {
        template.variable.clone()
    };

    let true_vals = vec![RuntimeValue::Boolean(true)];  // Only one true value needed
    let false_vals = vec![RuntimeValue::Boolean(false)];  // Only one false value needed

    let (true_list, false_list) = if template.not {
        (false_vals, true_vals)
    } else {
        (true_vals, false_vals)
    };

    let true_list = true_list.into_iter().map(|v| {
        let mut map = HashMap::new();
        map.insert(template.variable.clone(), v);
        map
    }).collect();

    let false_list = false_list.into_iter().map(|v| {
        let mut map = HashMap::new();
        map.insert(template.variable.clone(), v);
        map
    }).collect();

    TestExpression::new(expr, true_list, false_list)
}

// Helper functions for string comparison value generation

/// Generate strings that are lexicographically greater than the given string
fn generate_greater_strings(s: &str, count: usize) -> Vec<String> {
    let mut result = Vec::new();

    // Strategy: append characters or increment last character
    result.push(format!("{}z", s));  // Append 'z'
    result.push(format!("{}zz", s)); // Append 'zz'

    // Increment last character if possible
    if let Some(last_char) = s.chars().last() {
        if last_char < 'z' {
            let mut chars: Vec<char> = s.chars().collect();
            chars.pop();
            chars.push((last_char as u8 + 1) as char);
            result.push(chars.into_iter().collect());
        }
    }

    // Add some high values
    result.push("zzzzz".to_string());
    result.push("~~~~~".to_string());

    result.truncate(count);
    while result.len() < count {
        result.push(format!("{}_{}", s, result.len()));
    }
    result
}

/// Generate strings that are lexicographically less than the given string
fn generate_lesser_strings(s: &str, count: usize) -> Vec<String> {
    let mut result = Vec::new();

    // Decrement last character if possible
    if let Some(last_char) = s.chars().last() {
        if last_char > 'a' {
            let mut chars: Vec<char> = s.chars().collect();
            chars.pop();
            chars.push((last_char as u8 - 1) as char);
            result.push(chars.into_iter().collect());
        }
    }

    // Truncate string
    if s.len() > 1 {
        result.push(s[0..s.len()-1].to_string());
    }
    if s.len() > 2 {
        result.push(s[0..s.len()-2].to_string());
    }

    // Add some low values
    result.push("a".to_string());
    result.push("aa".to_string());
    result.push("aaa".to_string());
    result.push("".to_string());

    // Filter to only keep values actually less than s
    result.retain(|x| x.as_str() < s);

    result.truncate(count);
    while result.len() < count {
        result.push(format!("a{}", result.len()));
    }
    result
}

/// Generate strings that are different from the given string
fn generate_different_strings(s: &str, count: usize) -> Vec<String> {
    let candidates = vec![
        "different1", "different2", "other1", "other2", "other3",
        "notequal1", "notequal2", "xyz", "abc", "test",
        "hello", "world", "foo", "bar", "baz",
    ];

    candidates.iter()
        .filter(|x| **x != s)
        .take(count)
        .map(|x| x.to_string())
        .collect()
}

/// Generate arithmetic comparison expression
fn generate_arithmetic_comparison(template: &ArithmeticComparisonTemplate) -> TestExpression {
    let expr = format!("{} {} {} {} {}",
        template.left_var,
        template.arith_op,
        template.arith_operand.to_string(),
        template.cmp_op,
        template.right.to_string());

    let (true_list, false_list) = generate_arithmetic_values(
        &template.left_var,
        template.arith_op,
        &template.arith_operand,
        template.cmp_op,
        &template.right,
        &template.value_type
    );

    TestExpression::new(expr, true_list, false_list)
}

/// Generate values for arithmetic comparison expressions
/// For expression: var {arith_op} arith_operand {cmp_op} right
/// We need to solve for var values that make the expression true/false
fn generate_arithmetic_values(
    var: &str,
    arith_op: &str,
    arith_operand: &LiteralValue,
    cmp_op: &str,
    right: &LiteralValue,
    value_type: &ValueType,
) -> (Vec<HashMap<String, RuntimeValue>>, Vec<HashMap<String, RuntimeValue>>) {
    match value_type {
        ValueType::Integer => {
            let arith_val = match arith_operand {
                LiteralValue::Integer(v) => *v,
                _ => panic!("Mismatched type in arithmetic operand"),
            };
            let target = match right {
                LiteralValue::Integer(v) => *v,
                _ => panic!("Mismatched type in comparison target"),
            };

            // Solve for var: var {arith_op} arith_val {cmp_op} target
            // For each comparison operator, find var values that satisfy
            let base_var = match arith_op {
                "+" => target - arith_val,  // var + a = t => var = t - a
                "-" => target + arith_val,  // var - a = t => var = t + a
                "*" => {
                    if arith_val != 0 {
                        target / arith_val  // var * a = t => var = t / a (integer division)
                    } else {
                        panic!("Division by zero in arithmetic template");
                    }
                }
                _ => panic!("Unknown arithmetic operator: {}", arith_op),
            };

            // Generate true values based on comparison operator
            let true_vals = match cmp_op {
                "=" => vec![base_var],
                "!=" | "<>" => {
                    // Values that make result != target
                    match arith_op {
                        "+" => vec![base_var - 1, base_var + 1, base_var - 5, base_var + 5, base_var - 10],
                        "-" => vec![base_var - 1, base_var + 1, base_var - 5, base_var + 5, base_var + 10],
                        "*" => vec![base_var - 1, base_var + 1, base_var - 2, base_var + 2, base_var + 5],
                        _ => unreachable!(),
                    }
                }
                ">" => {
                    // var {arith_op} arith_val > target
                    // Need var such that result > target
                    match arith_op {
                        "+" => vec![base_var + 1, base_var + 2, base_var + 5, base_var + 10, base_var + 20],
                        "-" => vec![base_var + 1, base_var + 2, base_var + 5, base_var + 10, base_var + 20],
                        "*" => vec![base_var + 1, base_var + 2, base_var + 3, base_var + 5, base_var + 10],
                        _ => unreachable!(),
                    }
                }
                ">=" => {
                    match arith_op {
                        "+" => vec![base_var, base_var + 1, base_var + 2, base_var + 5, base_var + 10],
                        "-" => vec![base_var, base_var + 1, base_var + 2, base_var + 5, base_var + 10],
                        "*" => vec![base_var, base_var + 1, base_var + 2, base_var + 3, base_var + 5],
                        _ => unreachable!(),
                    }
                }
                "<" => {
                    match arith_op {
                        "+" => vec![base_var - 1, base_var - 2, base_var - 5, base_var - 10, base_var - 20],
                        "-" => vec![base_var - 1, base_var - 2, base_var - 5, base_var - 10, base_var - 20],
                        "*" => vec![base_var - 1, base_var - 2, base_var - 3, base_var - 5, base_var - 10],
                        _ => unreachable!(),
                    }
                }
                "<=" => {
                    match arith_op {
                        "+" => vec![base_var, base_var - 1, base_var - 2, base_var - 5, base_var - 10],
                        "-" => vec![base_var, base_var - 1, base_var - 2, base_var - 5, base_var - 10],
                        "*" => vec![base_var, base_var - 1, base_var - 2, base_var - 3, base_var - 5],
                        _ => unreachable!(),
                    }
                }
                _ => panic!("Unknown comparison operator: {}", cmp_op),
            };

            // Generate false values (opposite of true values)
            let false_vals = match cmp_op {
                "=" => vec![base_var - 1, base_var + 1, base_var - 5, base_var + 5, base_var + 10],
                "!=" | "<>" => vec![base_var],
                ">" => {
                    match arith_op {
                        "+" => vec![base_var, base_var - 1, base_var - 2, base_var - 5, base_var - 10],
                        "-" => vec![base_var, base_var - 1, base_var - 2, base_var - 5, base_var - 10],
                        "*" => vec![base_var, base_var - 1, base_var - 2, base_var - 3, base_var - 5],
                        _ => unreachable!(),
                    }
                }
                ">=" => {
                    match arith_op {
                        "+" => vec![base_var - 1, base_var - 2, base_var - 5, base_var - 10, base_var - 20],
                        "-" => vec![base_var - 1, base_var - 2, base_var - 5, base_var - 10, base_var - 20],
                        "*" => vec![base_var - 1, base_var - 2, base_var - 3, base_var - 5, base_var - 10],
                        _ => unreachable!(),
                    }
                }
                "<" => {
                    match arith_op {
                        "+" => vec![base_var, base_var + 1, base_var + 2, base_var + 5, base_var + 10],
                        "-" => vec![base_var, base_var + 1, base_var + 2, base_var + 5, base_var + 10],
                        "*" => vec![base_var, base_var + 1, base_var + 2, base_var + 3, base_var + 5],
                        _ => unreachable!(),
                    }
                }
                "<=" => {
                    match arith_op {
                        "+" => vec![base_var + 1, base_var + 2, base_var + 5, base_var + 10, base_var + 20],
                        "-" => vec![base_var + 1, base_var + 2, base_var + 5, base_var + 10, base_var + 20],
                        "*" => vec![base_var + 1, base_var + 2, base_var + 3, base_var + 5, base_var + 10],
                        _ => unreachable!(),
                    }
                }
                _ => panic!("Unknown comparison operator: {}", cmp_op),
            };

            (
                true_vals.into_iter().map(|v| {
                    let mut map = HashMap::new();
                    map.insert(var.to_string(), RuntimeValue::Integer(v));
                    map
                }).collect(),
                false_vals.into_iter().map(|v| {
                    let mut map = HashMap::new();
                    map.insert(var.to_string(), RuntimeValue::Integer(v));
                    map
                }).collect(),
            )
        }
        ValueType::Float => {
            let arith_val = match arith_operand {
                LiteralValue::Float(v) => *v,
                _ => panic!("Mismatched type in arithmetic operand"),
            };
            let target = match right {
                LiteralValue::Float(v) => *v,
                _ => panic!("Mismatched type in comparison target"),
            };

            let base_var = match arith_op {
                "+" => target - arith_val,
                "-" => target + arith_val,
                "*" => {
                    if arith_val != 0.0 {
                        target / arith_val
                    } else {
                        panic!("Division by zero in arithmetic template");
                    }
                }
                _ => panic!("Unknown arithmetic operator: {}", arith_op),
            };

            let true_vals = match cmp_op {
                "=" => vec![base_var],
                "!=" | "<>" => vec![base_var - 1.0, base_var + 1.0, base_var - 5.0, base_var + 5.0, base_var - 10.0],
                ">" => vec![base_var + 1.0, base_var + 2.0, base_var + 5.0, base_var + 10.0, base_var + 20.0],
                ">=" => vec![base_var, base_var + 1.0, base_var + 2.0, base_var + 5.0, base_var + 10.0],
                "<" => vec![base_var - 1.0, base_var - 2.0, base_var - 5.0, base_var - 10.0, base_var - 20.0],
                "<=" => vec![base_var, base_var - 1.0, base_var - 2.0, base_var - 5.0, base_var - 10.0],
                _ => panic!("Unknown comparison operator: {}", cmp_op),
            };

            let false_vals = match cmp_op {
                "=" => vec![base_var - 1.0, base_var + 1.0, base_var - 5.0, base_var + 5.0, base_var + 10.0],
                "!=" | "<>" => vec![base_var],
                ">" => vec![base_var, base_var - 1.0, base_var - 2.0, base_var - 5.0, base_var - 10.0],
                ">=" => vec![base_var - 1.0, base_var - 2.0, base_var - 5.0, base_var - 10.0, base_var - 20.0],
                "<" => vec![base_var, base_var + 1.0, base_var + 2.0, base_var + 5.0, base_var + 10.0],
                "<=" => vec![base_var + 1.0, base_var + 2.0, base_var + 5.0, base_var + 10.0, base_var + 20.0],
                _ => panic!("Unknown comparison operator: {}", cmp_op),
            };

            (
                true_vals.into_iter().map(|v| {
                    let mut map = HashMap::new();
                    map.insert(var.to_string(), RuntimeValue::Float(v));
                    map
                }).collect(),
                false_vals.into_iter().map(|v| {
                    let mut map = HashMap::new();
                    map.insert(var.to_string(), RuntimeValue::Float(v));
                    map
                }).collect(),
            )
        }
        _ => panic!("Unsupported value type for arithmetic comparison"),
    }
}
