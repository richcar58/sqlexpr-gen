# Release Notes

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
