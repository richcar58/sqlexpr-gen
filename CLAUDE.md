# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`sqlexpr-gen` is a test generation tool for the `sqlexpr-rust` project. The goal is to generate 10,000 evaluation tests for validating different implementations of a SQL expression parser/evaluator that share the same expression language.

## Commands

### Build and Run
```bash
cargo build          # Build the project
cargo run            # Run the main binary
cargo test           # Run tests
cargo check          # Fast compile check without producing binary
```

## Project Architecture

### Core Concepts

The project generates test data for `sqlexpr-rust`, which uses the following conventions:

**Variable Naming Convention:**
- All variables: single lowercase character + integer (e.g., `i1`, `f2`, `s3`, `b4`)
- First character indicates type:
  - `i` = integer values
  - `f` = float values
  - `s` = string values
  - `b` = boolean values

**Expression Complexity:**
- Complexity = number of logical OR and AND operators in expression
- Complexity Class = set of expressions with same complexity

**Operators and Expressions:**
- Relational operators: Equal, NotEqual, GreaterThan, GreaterOrEqual, LessThan, LessOrEqual, Like, Between, In, IsNull
- Conjunct = operand of a conjunction (AND)
- Disjunct = operand of a disjunction (OR)
- Negated relational expressions use NOT with Like, Between, In, or IsNull

### Test Generation Strategy

Tests are generated in phases to enable step-by-step validation:

**Phase I - Simple Relational Expressions:**
Generate single-operator relational expressions with:
- Prototypical expressions for each operator/operand type combination
- Two validation lists per expression:
  - `true_list`: 5 value sets that evaluate to true
  - `false_list`: 5 value sets that evaluate to false
- Output to `resources/simple_expressions.json` as array of objects with `expr`, `true_list`, and `false_list` fields

**Operator-Specific Constraints:**
- LIKE: second operand must be string with wildcards (%, _)
- BETWEEN: second and third operands are inclusive bounds (numeric or string literals)
- IN: second operand must be homogeneous list (all integers, floats, or strings)
- No expressions with only constants
- Each expression uses different constants
- All expressions must validate successfully with `Evaluate::evaluate(input: &str, map: &HashMap<String, RuntimeValue>)`

### Directory Structure

- `src/` - Rust source code (currently minimal skeleton)
- `docs/` - Project documentation and requirements
- `resources/` - Generated test data (JSON files)
- `target/` - Build artifacts (ignored)

## Integration with sqlexpr-rust

This project generates test inputs for `sqlexpr-rust` (https://github.com/richcar58/sqlexpr-rust), a SQL-like boolean expression parser and evaluator with grammar-enforced type safety.

### Expression Language Features

**AST Hierarchy (three-tier structure):**
- `BooleanExpression`: AND, OR, NOT, boolean literals/variables, relational expressions
- `RelationalExpression`: Comparisons, LIKE, BETWEEN, IN, IS NULL operations
- `ValueExpression`: Arithmetic operations, literals, variables

**Supported Operators:**
- Logical: AND, OR, NOT (with short-circuit evaluation)
- Comparison: `>`, `>=`, `<`, `<=`, `=`, `<>`, `!=`
- Pattern: LIKE/NOT LIKE (wildcards: `%` multi-char, `_` single-char, plus ESCAPE clause)
- Range: BETWEEN/NOT BETWEEN (inclusive)
- Membership: IN/NOT IN
- Null testing: IS NULL, IS NOT NULL
- Arithmetic: `+`, `-`, `*`, `/`, `%` (modulo)
- Unary: `+`, `-`

**Type System:**
- Runtime values: Integer (i64), Float (f64), String, Boolean, Null
- Type coercion in arithmetic:
  - Int + Int → Int
  - Float + Float → Float
  - Mixed arithmetic (Int + Float) → Float (auto-promotion)
  - Division always returns Float
- NULL handling: NULL in arithmetic/comparisons raises errors; must use IS NULL instead

### Critical Constraints for Test Generation

- All top-level expressions must evaluate to boolean
- NULL values cannot participate in arithmetic or comparison operations
- Mixed-type arithmetic automatically promotes integers to floats
- LIKE patterns: `%` matches any sequence, `_` matches single character
- BETWEEN ranges are inclusive on both ends
- IN lists must be homogeneous (all same type)
- The evaluator uses HashMap for variable binding resolution
