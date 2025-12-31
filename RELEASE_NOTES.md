# Release Notes

## Version 1.1.0 (2025-12-31)

Adds multi-language support and arithmetic expression templates to enable testing of SQL expression parsers with different language restrictions.

### New Features

#### Multi-Language Support
- Added `--lang` command-line argument for selecting expression language level
  - **max**: Full unrestricted language (default) - all operators and type combinations
  - **limited**: Restricted subset for implementations with specific constraints
- Language-specific output files:
  - `resources/simple_expressions-{lang}.json`
  - `resources/complex_expressions-{lang}.json`
  - `evaluator_test-{lang}.out` and `evaluator_test-{lang}.failed`
- All three binaries (phase1, phase2, evaluator_test) support `--lang` argument
- `command_prompts_2.md` contains a history of the Claude prompts used to implement this feature.

#### Limited Language Restrictions
The limited language excludes expressions not supported by certain implementations:
- String operands in comparison operators: `>`, `>=`, `<`, `<=`
- String operands in `BETWEEN` clauses
- Numeric operands (integer/float) in `IN` clauses

#### Arithmetic Expression Templates
- Added 12 new arithmetic comparison templates to Phase I generation
  - 6 integer templates: addition (+), subtraction (-), multiplication (*)
  - 6 float templates: addition (+), subtraction (-)
  - All comparison operators covered: `=`, `!=`, `>`, `>=`, `<`, `<=`
- Expression format: `{variable} {arith_op} {literal} {cmp_op} {literal}`
- Examples: `i109 + 10 = 20`, `f116 + 2.5 > 10`, `i113 * 2 < 100`

### Improvements

#### Enhanced Template Counts
- **Max language**: 90 → 102 templates (+12 arithmetic templates)
- **Limited language**: 72 → 84 templates (+12 arithmetic templates)
- Limited language now supports complexity class 75 (requires 76 templates, provides 84)

#### CLI Error Handling
- Improved error messages for `--lang` argument parsing
- Handles both `--lang=value` and `--lang value` formats
- Clear error messages for invalid or missing language values

### Technical Details

#### Modified Files
- `src/common/language.rs` - New Language enum and path helper methods
- `src/common/cli.rs` - Command-line argument parsing
- `src/phase1/templates.rs` - ArithmeticComparisonTemplate struct and generation
- `src/phase1/generator.rs` - Arithmetic expression value generation logic
- `src/bin/phase1.rs`, `phase2.rs`, `evaluator_test.rs` - Language argument support
- `src/phase2/loader.rs` - Language-specific file loading

#### Value Generation Strategy
Arithmetic templates solve for variable values algebraically:
- For `var + a = target`: generates values where `var = target - a` (true) and `var ≠ target - a` (false)
- For `var - a = target`: generates values where `var = target + a` (true) and `var ≠ target + a` (false)
- For `var * a = target`: generates values where `var = target / a` (true) and `var ≠ target / a` (false)

---

## Version 1.0.0 (2025-12-17)

Initial release of `sqlexpr-gen`, a comprehensive test data generator for the sqlexpr-rust SQL expression parser and evaluator.

### Features

#### Phase I: Simple Expression Generation
- Generates ~150 prototypical single-operator relational expressions
- Covers all operator/operand type combinations supported by sqlexpr-rust
- Each expression includes 5 true-evaluating and 5 false-evaluating value sets
- Supports operators: `=`, `<>`, `>`, `>=`, `<`, `<=`, `LIKE`, `NOT LIKE`, `BETWEEN`, `NOT BETWEEN`, `IN`, `NOT IN`, `IS NULL`, `IS NOT NULL`
- Output: `resources/simple_expressions.json`

#### Phase II: Complex Expression Generation
- Generates 10,000 complex boolean expressions for load testing
- 20 complexity classes: 1-10, 15, 20, 25, 30, 35, 40, 45, 50, 60, 75
- 500 unique expressions per complexity class
- Sophisticated generation features:
  - Weighted operator selection (60/40 probability bias)
  - Automatic OR sequence parenthesization (2+ consecutive ORs)
  - Complete short-circuit avoidance for maximum evaluator workload
  - Non-overlapping variable names within each expression
  - Alphabetically sorted value maps in JSON output
- Output: `resources/complex_expressions.json`

#### Short-Circuit Avoidance
A key innovation ensuring all expression clauses are evaluated during testing:
- AND clauses: All operands evaluate to true (prevents early termination)
- OR sequences: First N-1 operands false, last operand true (forces full evaluation)
- Standalone OR: First operand false, second operand true (ensures both evaluated)
- Results in 3x increase in evaluation time compared to short-circuiting code

#### Evaluator Test Harness
- Validates all 10,000 generated complex expressions
- Measures evaluation performance and success rates
- Reports detailed metrics: throughput, average time per expression, failures
- Outputs: `evaluator_test.out` (summary), `evaluator_test.failed` (failures if any)

---

For detailed usage instructions, see [README.md](README.md).

For development guidance, see [CLAUDE.md](CLAUDE.md).

For license, see [LICENSE](LICENSE).
