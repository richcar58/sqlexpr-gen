# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`sqlexpr-gen` is a test generation tool for the `sqlexpr-rust` project. The goal is to generate 10,000 evaluation tests for validating different implementations of a SQL expression parser/evaluator that share the same expression language.

**Version**: 1.1.0 - Fully implemented and tested

## Commands

### Build and Run
```bash
cargo build                      # Build debug version (all binaries)
cargo build --release            # Build release version (optimized)
cargo test                       # Run unit tests
cargo check                      # Fast compile check

# Phase I: Generate simple expressions (default: max language)
cargo run --bin phase1
cargo run --release --bin phase1

# Phase I: Generate for specific language
cargo run --release --bin phase1 -- --lang max
cargo run --release --bin phase1 -- --lang limited

# Phase II: Generate complex expressions (default: max language)
cargo run --bin phase2
cargo run --release --bin phase2

# Phase II: Generate for specific language
cargo run --release --bin phase2 -- --lang max
cargo run --release --bin phase2 -- --lang limited

# Run evaluator tests (default: max language)
cargo run --bin evaluator_test
cargo run --release --bin evaluator_test

# Run evaluator tests for specific language
cargo run --release --bin evaluator_test -- --lang max
cargo run --release --bin evaluator_test -- --lang limited
```

## Project Architecture

### Core Concepts

The project generates test data for `sqlexpr-rust`, which uses the following conventions:

**Language Levels:**
- `max`: Full unrestricted language (all operators, all type combinations) - ~90 templates
- `limited`: Restricted subset excluding:
  - String operands in: `>`, `>=`, `<`, `<=`, `BETWEEN`
  - Numeric operands in: `IN` clauses (only strings allowed)
  - Results in ~72 templates
- Default language if `--lang` not specified: `max`

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
- Prototypical expressions for each operator/operand type combination (~150 expressions)
- Two validation lists per expression:
  - `true_list`: 5 value sets that evaluate to true
  - `false_list`: 5 value sets that evaluate to false
- Output to `resources/simple_expressions-{lang}.json` as array of objects with `expr`, `true_list`, and `false_list` fields (where `{lang}` is `max` or `limited`)

**Phase I Operator-Specific Constraints:**
- LIKE: second operand must be string with wildcards (%, _)
- BETWEEN: second and third operands are inclusive bounds (numeric or string literals)
- IN: second operand must be homogeneous list (all integers, floats, or strings)
- No expressions with only constants
- Each expression uses different constants
- All expressions must validate successfully with `Evaluate::evaluate(input: &str, map: &HashMap<String, RuntimeValue>)`

**Phase II - Complex Relational Expressions:**
Combine simple expressions into complex boolean expressions for load testing:
- Generate 500 unique expressions per complexity class (10,000 total)
- Complexity classes: 1-10, 15, 20, 25, 30, 35, 40, 45, 50, 60, 75
- Output to `resources/complex_expressions-{lang}.json` with `expr` and `value_map` fields (where `{lang}` is `max` or `limited`)

**Phase II Generation Rules:**
1. Simple expressions randomly selected from Phase I output
2. Each simple expression used only once per complex expression
3. All selected simple expressions enclosed in parentheses
4. Operators joined by AND/OR with weighted selection:
   - First operator: randomly chosen
   - Subsequent operators: 60% same as previous, 40% different
5. OR sequences (2+ consecutive ORs) are parenthesized
6. Short-circuit avoidance:
   - AND clauses: assign values from `true_list` (all evaluate to true)
   - OR sequences: first (N-1) from `false_list`, last from `true_list`
   - Standalone OR: first from `false_list`, second from `true_list`
7. Non-overlapping variable names within each expression
8. Value maps alphabetically sorted by variable name

**Evaluator Test:**
- Reads `resources/complex_expressions-{lang}.json` (where `{lang}` is `max` or `limited`)
- Evaluates all expressions using `evaluate()` from sqlexpr-rust
- Outputs success/failure counts and timing to `evaluator_test-{lang}.out`
- Logs failures (if any) to `evaluator_test-{lang}.failed`

### Directory Structure

```
sqlexpr-gen/
├── src/
│   ├── bin/
│   │   ├── phase1.rs          # Simple expression generator
│   │   ├── phase2.rs          # Complex expression generator
│   │   └── evaluator_test.rs  # Test harness
│   ├── phase1/
│   │   ├── generator.rs       # Phase I generation logic
│   │   ├── templates.rs       # Expression templates
│   │   ├── validator.rs       # Expression validation
│   │   └── mod.rs
│   ├── phase2/
│   │   ├── combiner.rs        # Expression combination with short-circuit avoidance
│   │   ├── loader.rs          # Phase I output loader
│   │   ├── types.rs           # Data structures and complexity classes
│   │   └── mod.rs
│   ├── common/
│   │   ├── cli.rs             # Command-line argument parsing
│   │   ├── language.rs        # Language enum and path helpers
│   │   ├── output.rs          # Shared RuntimeValue types
│   │   └── mod.rs
│   └── lib.rs
├── resources/
│   ├── simple_expressions-max.json      # Phase I output for max language (~90 expressions)
│   ├── simple_expressions-limited.json  # Phase I output for limited language (~72 expressions)
│   ├── complex_expressions-max.json     # Phase II output for max language (10,000 expressions)
│   └── complex_expressions-limited.json # Phase II output for limited language (10,000 expressions)
├── docs/
│   └── command_prompts.md        # Detailed specifications
├── Cargo.toml
├── LICENSE                       # MIT License
├── README.md                     # User documentation
├── RELEASE_NOTES.md              # Version history
└── CLAUDE.md                     # This file
```

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

## Implementation Status (v1.0.0)

**Completed Features:**
- ✓ Phase I simple expression generation (~150 expressions)
- ✓ Phase II complex expression generation (10,000 expressions across 20 complexity classes)
- ✓ Short-circuit avoidance logic (maximizes evaluator workload)
- ✓ OR sequence parenthesization (2+ consecutive ORs)
- ✓ Weighted operator selection (60/40 bias)
- ✓ Non-overlapping variable names per expression
- ✓ Alphabetically sorted value maps in JSON output
- ✓ Comprehensive evaluator test harness
- ✓ 100% success rate on all 10,000 generated expressions
- ✓ Performance: ~11,300 expressions/second (release build)

**Test Results:**
- All 10,000 complex expressions evaluate successfully
- Short-circuit avoidance verified (3x increase in evaluation time vs. short-circuiting)
- Release build: 0.885 seconds total, 88 microseconds per expression average
