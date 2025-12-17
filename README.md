# sqlexpr-gen

A comprehensive test data generator for the [sqlexpr-rust](https://github.com/richcar58/sqlexpr-rust) SQL expression parser and evaluator.

## Overview

`sqlexpr-gen` generates thousands of validated test expressions with corresponding value mappings for performance testing and validation of SQL-like boolean expression evaluators. The tool produces two types of test data:

1. **Simple Expressions** - Single relational operator expressions with true/false value lists
2. **Complex Expressions** - Multi-operator boolean expressions optimized for load testing

The generated expressions are specifically designed to avoid short-circuiting, ensuring maximum evaluator workload during performance testing.

### Related Projects

- **GitHub Repository**: [https://github.com/richcar58/sqlexpr-gen](https://github.com/richcar58/sqlexpr-gen)
- **sqlexpr-rust GitHub**: [https://github.com/richcar58/sqlexpr-rust](https://github.com/richcar58/sqlexpr-rust)
- **sqlexpr-rust on crates.io**: [https://crates.io/crates/sqlexpr-rust](https://crates.io/crates/sqlexpr-rust)

## Features

### Phase I: Simple Expression Generation

Generates single-operator relational expressions covering all operator/operand combinations:

- **Operators Supported**: `=`, `<>`, `>`, `>=`, `<`, `<=`, `LIKE`, `NOT LIKE`, `BETWEEN`, `NOT BETWEEN`, `IN`, `NOT IN`, `IS NULL`, `IS NOT NULL`
- **Data Types**: Integer, Float, String, Boolean, Null
- **Variable Naming**: Type-prefixed globally unique names (e.g., `i1`, `f2`, `s3`, `b4`)
- **Value Lists**: Each expression includes 5 true-evaluating and 5 false-evaluating value sets
- **Output**: `resources/simple_expressions.json` with ~150 prototypical expressions

### Phase II: Complex Expression Generation

Combines simple expressions into complex boolean expressions for load testing:

- **Complexity Classes**: 20 classes spanning complexity 1-75 (where complexity = number of logical operators)
  - Classes 1-10: Individual complexity levels
  - Classes 15, 20, 25, 30, 35, 40, 45, 50, 60, 75: Higher complexity targets
- **Generation Volume**: 500 unique expressions per complexity class (10,000 total)
- **Operator Selection**: Weighted 60/40 probability (60% same as previous, 40% different) for natural expression variety
- **OR Sequence Handling**: Sequences of 2+ consecutive OR operators are automatically parenthesized
- **Short-Circuit Avoidance**: Strategic value assignment ensures all clauses are evaluated:
  - AND clauses: All operands evaluate to true
  - OR sequences: First N-1 operands false, last operand true
  - Standalone OR: First operand false, second operand true
- **Variable Management**: Non-overlapping variable names within each expression
- **Output**: `resources/complex_expressions.json` with alphabetically-sorted value maps

### Evaluator Testing

Built-in test harness validates all generated expressions:

- Loads complex expressions and evaluates them against sqlexpr-rust
- Tracks success/failure rates and execution timing
- Reports performance metrics (expressions/second, average time per expression)
- Logs failed expressions with error details for debugging
- Output files: `evaluator_test.out` (summary), `evaluator_test.failed` (failures if any)

## Key Concepts

### Complexity Class

The **complexity** of an expression is defined as the number of logical operators (AND/OR) it contains. For example:
- `A AND B` has complexity 1 (1 operator)
- `A AND B OR C` has complexity 2 (2 operators)
- `A AND B AND C AND D` has complexity 3 (3 operators)

Expressions with the same complexity form a **complexity class**.

### Short-Circuit Avoidance

Boolean expression evaluators typically use short-circuit evaluation:
- `false AND X` - X is never evaluated
- `true OR Y` - Y is never evaluated

For load testing, we want to maximize evaluator work by ensuring ALL clauses are evaluated. This is achieved by:
- Setting AND operands to true (forces evaluation of subsequent operands)
- Setting first N-1 OR operands to false (forces evaluation through to last operand)
- Parenthesizing OR sequences to group them as evaluation units

### Variable Naming Convention

All variables follow a strict naming pattern: `{type}{number}`

- `i` prefix = Integer values (e.g., `i1`, `i42`)
- `f` prefix = Float values (e.g., `f5`, `f91`)
- `s` prefix = String values (e.g., `s9`, `s100`)
- `b` prefix = Boolean values (e.g., `b107`, `b108`)

Numbers are globally unique across the entire test suite, ensuring no accidental variable collisions.

## Installation and Compilation

### Prerequisites

- Rust 1.89 or later
- Cargo (included with Rust)

### Building the Project

```bash
# Clone the repository
git clone https://github.com/richcar58/sqlexpr-gen.git
cd sqlexpr-gen

# Build debug version (fast compilation)
cargo build

# Build release version (optimized for performance)
cargo build --release
```

## User Guide

### 1. Generate Simple Expressions (Phase I)

Generate the foundational set of simple relational expressions:

```bash
# Debug build
cargo run --bin phase1

# Release build (faster)
cargo run --release --bin phase1
```

**Output**: `resources/simple_expressions.json`

This creates approximately 150 simple expressions covering all operator/operand type combinations. Each expression includes:
- The expression string (e.g., `"i1 = 7"`)
- A `true_list` with 5 value sets that make the expression evaluate to true
- A `false_list` with 5 value sets that make the expression evaluate to false

### 2. Generate Complex Expressions (Phase II)

Combine simple expressions into complex test expressions:

```bash
# Debug build
cargo run --bin phase2

# Release build (recommended for speed)
cargo run --release --bin phase2
```

**Output**: `resources/complex_expressions.json`

This generates 10,000 complex expressions (500 per complexity class) by randomly combining simple expressions. Each complex expression includes:
- The expression string with proper parenthesization
- A `value_map` with variable assignments that avoid short-circuiting

**Note**: Phase II requires Phase I output (`resources/simple_expressions.json`) to exist.

### 3. Run Evaluator Test

Validate all generated complex expressions:

```bash
# Debug build
cargo run --bin evaluator_test

# Release build (recommended for accurate performance metrics)
cargo run --release --bin evaluator_test
```

**Outputs**:
- `evaluator_test.out` - Test summary with success rate and performance metrics
- `evaluator_test.failed` - Failed expressions with error messages (only created if failures occur)

**Example output**:
```
Total expressions evaluated: 10000
Successful evaluations: 10000
Failed evaluations: 0
Success rate: 100.00%

Total execution time: 0.885 seconds
Average time per expression: 0.000088 seconds
Expressions per second: 11300.45
```

## Performance Characteristics

### Generation Performance

- Phase I: Generates ~150 expressions in < 1 second
- Phase II: Generates 10,000 expressions in ~10-15 seconds (debug) / ~5-8 seconds (release)

### Evaluation Performance (Release Build)

- 10,000 expressions evaluated in ~0.9 seconds
- Throughput: ~11,300 expressions/second
- Average: ~88 microseconds per expression
- 100% success rate with properly generated expressions

### Short-Circuit Impact

The short-circuit avoidance feature significantly increases evaluation time, which is the desired behavior:
- With short-circuiting: ~5 seconds (many clauses skipped)
- Without short-circuiting: ~15 seconds (all clauses evaluated) - 3x more work!

This demonstrates that the test data successfully maximizes evaluator workload.

## Project Structure

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
│   │   └── validator.rs       # Expression validation
│   ├── phase2/
│   │   ├── combiner.rs        # Expression combination logic
│   │   ├── loader.rs          # Phase I output loader
│   │   └── types.rs           # Data structures
│   ├── common/
│   │   ├── output.rs          # Shared output types
│   │   └── mod.rs
│   └── lib.rs
├── resources/
│   ├── simple_expressions.json   # Phase I output
│   └── complex_expressions.json  # Phase II output
├── docs/
│   └── command_prompts.md     # Detailed specifications
├── Cargo.toml
├── LICENSE                    # MIT License
└── README.md
```

## Testing Methodology

The generated test data follows these validation principles:

1. **Type Safety**: All expressions respect sqlexpr-rust's type system (Integer, Float, String, Boolean, Null)
2. **Null Handling**: NULL values only appear in IS NULL/IS NOT NULL contexts (no arithmetic/comparison)
3. **Operator Coverage**: All supported operators have representative test cases
4. **Edge Cases**: Includes boundary values, empty strings, special characters, etc.
5. **Uniqueness**: No duplicate expressions within a complexity class
6. **Non-Overlapping Variables**: Each complex expression uses distinct variable names
7. **Realistic Patterns**: Operator selection weighted to favor natural expression structures

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Version History

See [RELEASE_NOTES.md](RELEASE_NOTES.md) for version history and changes.

## Acknowledgments

Anthopic's Claude Sonnet 4.5 was used to generate most of the code and documentation in this project.  See [docs/command_prompts.md](docs/command_prompts.md) for prompt history.

