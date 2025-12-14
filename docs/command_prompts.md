# Load and Performance Testing SqlExpr-Rust

The ultimae goal of this project is to generate 10,000 evaluation tests for the sqlexpr-rust project.  The tests will be used to compare the performance of different implementations of the parser/evaluator  that accept the same expression language.  We build the test data in phases so that we can validate results step by step. 

## Load Background Information

claude
/init
brave_web_search sqlexpr-rust at https://github.com/richcar58/sqlexpr-rust

## Phase 0 - Definitions and Conventions

We start with definitions that apply to the sqlexpr-rust language and implementation:

1. **Expression Complexity** - The complexity of a boolean expression is the number of logical OR (disjuction) and AND (conjunction) operators it contains.
2. **Complexity Class** - A complexity class is the set of expressions with the same expression complexity.
2. **Conjunct** - An operand of a conjunction is a *conjunct*.
3. **Disjunct** - An operand of a disjunction is a *disjunct*.
4. **Relational Expression** - An expression defined as a *RelationalExpr* in ast.rs.  Specifically, these exressions are Equal, NotEqual, GreaterThan, GreaterOrEqual, LessThan, LessOrEqual, Like, Between, In and IsNull.
5. **Negated Relational Expression** - A Like, Between, In or IsNull expression that incorporates the logical NOT operator.  NOT can also precede other relational expressions. 
6. **Relational Operator** - An operator used in any relational expression.  Specifically, these operators are the Equal, NotEqual, GreaterThan, GreaterOrEqual, LessThan, LessOrEqual, Like, Between, In and Is tokens defined in lexer.rs.

The names of variables in expressions follow these rules:

1. All variable names start with a single lowercase character followed by an integer.  The first character must reflect the variable's type as follows:
    1. *i* indicates integer values
    2. *f* indicates float values
    3. *s* indicates string values
    4. *b* indicates boolean values

## Phase I Design - Generating Simple Relational Expressions

The goal of this phase is to plan for the generation of simple, single operator relational expressions and then execute a chosen plan. Here are the requirements:

1. Create a comprehensive list of the prototypical expressions for each operand type to which the operator can be applied.  For example, for the GreaterThan (>), the following simple expression could be generated:

    1. u1 > 7
    2. -3 > u2
    3. f1 > 22.4
    4. 50.1 > f2
    5. s1 > 'banana'
    6. 'united' > s2
    7. u3 > u4
    8. f3 > f4
    9. s3 > s4

2. The 9 prototypical expressions in the list above show all the ways one or two variables can be used in a simple GreaterThan expression.  The same number of prototypes can be generated for Equal, NotEqual, GreaterOrEqual, LessThan, and LessOrEqual.
3. Here are the prototypical expressions for the *Like* operator:
    1. s1 LIKE '%sometext'
    2. s2 LIKE '_sometext'
    3. s3 LIKE 'sometext%'
    4. s4 LIKE 'sometext_'
    5. s5 LIKE 'some%text'
    6. s6 LIKE 'some%text'
    7. s3 LIKE 'some_text%'
    8. s4 LIKE 'some%text_'
4. Here are the prototypical expressions for the *BETWEEN* operator:
    1. u1 BETWEEN 1 AND 10
    1. f1 BETWEEN 1.0 AND 10.0
    1. s1 BETWEEN 'aa' AND 'bb'
5. Here are the prototypical expressions for the *IN* operator:
    1. u1 IN (1, 2. 3)
    2. f1 IN (1.0, 2.0, 3.0)
    3. s1 IN ('apple', 'banana', 'cherry')
6. Here are the prototypical expressions for the *IS* operator:
    1. u1 IS NULL
    2. f1 IS NULL
    3. s1 IS NULL
7. For each expression with Like, Between, In or Is operators, create another expression with *NOT* properly inserted. 
8. The following restriction apply on certain operations:
    1. The second operand on LIKE operations must be a string.
    2. The second and third operands on BETWEEN operations are the lower and upper bounds (inclusive) and can only be numeric or string literals.
    3. The second operand on IN operations can only be a homegeneous list of integer, float or string.
9. Here are the prototypical expressions for simple boolean expressions:
    1. b1
    2. NOT b2
10. Do not generate relational expressions that contain only numeric or string or boolean constants.
11. Use different constants in each generated expression.
12. For each generated expression, produce two lists of values such that:
    1. A *true list* of 5 values that each return **true** processed by Evaluate::evaluate(input: &str, map: &HashMap<String, RuntimeValue>).  See the *evaluator_tests.rs* test code in sqlexpr-rust for usage examples.
    2. A *false list* of 5 values that each return **false** processed by Evaluate::evaluate(input: &str, map: &HashMap<String, RuntimeValue>).
    3. Creating the true and false lists also serves to validate that all expressions are well-formed and have properly typed values.
    4. No invocation of evaluate() should ever fail.  If a failure occurs, the root cause must be determined and fixed.
13. Create a JSON file *resources/simple_expressions.json*, and populated it with the generated expressions and their validated value lists from previous steps.  The content should be formatted as an array of JSON objects, each with the following fields:
    1. *expr*, which is assigned an expression string
    2. *true_list*, which is assigned the list of values that cause the expression to evaluate to true
    3. *false_list*, which is assigned the list of values that cause the expression to evaluate to false
14, Populate *resources/simple_expressions.json* with the generated expressions and their validated value lists from previous steps.

Please generate possible approaches for implementing the above requirements.      

### Tweaks to Generated Expressions and Value Assignments

The *resources/simple_expressions.json* looks good.  Let's tighten things up by removing duplicate entries in all *true_list* and *false_list* fields.  Also, change the generation code to not insert duplicates into those lists.

