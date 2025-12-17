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

To make it easier to combine expression in the next phase of test generation, please modify the code so that all variable names are generated using a single monotonically increasing counter.  The counter is intialized at 1 and is incremented after its value is read during name generation.  The same counter is used to generate the names of variables of all types so that after all variable names have been generated, the counter's value will be one more than the number of variables.  

Let's improve upon the JSON format of the *resources/simple_expressions.json* file.  The *true_list* and *false_list* are each arrays of objects.  This should stay the same, but each object should be 1 or more key/value pairs where the key is the variable name and the value is the variable value.  For example, *i2: 88* assigns integer value 88 to the variable named i2.  The first letter of each variable name indicates its type, so there's no need to separately store type information.  Please modify the code to output the new format for *simple_expression.json* and test that no regression has taken place.   

## Phase II Design - Generating Complex Relational Expressions

In this phase, we combine simple expressions generated in Phase I to yield the complex expressions appropriate for performance load testing.  Complex expression strings along with their value mappings are used as input into Evaluate::evaluate(input: &str, map: &HashMap<String, RuntimeValue>) during load testing.  Complex expressions are generated as follows:

1. The goal of the generation process is to generate 500 unique combinations of expressions and value maps for each complexity class listed here:
    1. Complexity classes for each integer 1 to 10, inclusive.
    2. Complexity classes 15, 20, 25, 30, 35, 40, 45, 50, 60, 75.
2. The simple expressions that are incorporated into each complex expression are randomly chosen from the set of all expresions in *resources/simple_expressions.json*.
    1. A simple expression can only be used once per complex expression.
3. All selected simple expressions are enclosed in parentheses.  This is to improve readability and avoid unexpected capture.
4. Simple expressions are joined together by AND or OR operators to incrementally build a complex expression.
    1. The first logical operator (AND or OR) to appear in a complex expression is randomly selected.
    2. Every subsequent operator is selected using a weighed 60/40 probability.  60% of the time the next operator selected is the same as last operator selected; 40% of the time it differs from the last operator.  This bias slightly favors creating moderate length sequences of either AND or OR operators.
5. Sequences of two or more OR clauses are always parenthesized.  This implies that before and after such a sequence there is either (1) no logical operator, or (2) an AND operator, or (3) a NOT operator.
6. Short circuiting terminates conjuctive and disjunctive expression evaluation early.  This defeats the goal of maximizing test load, so we avoid short circuiting by following these rules:
    1. The variables in each AND clause are assigned values such that each clause evaluates to true. 
    2. In parenthesized sequence of OR clauses, only the last clause is assigned values that to evaluate to true, all other clauses evaluate to false.
    3. Standalone OR clauses, i.e., those that are not part of a parenthesized sequence of OR clauses, can take on any value.
7. Using the short circuiting rule specified above, either the *true_list* or the *false_list* of a given simple expression is selected as its value source.  The actual value object chosen from the selected list is randomly assigned.
8. The value mapping chosen for each complex expression covers all variables in the expression, but should not contain variable assignments that aren't referenced in the complex expression.  
9. The newly constructed complex expressions and their value maps are written to a JSON file, *resources/complex_expressions.json*.
    1. The JSON file contains an array of JSON objects with each object having the following fields:
        1. The *expr* field contains a complex expression string.
        2. The *value_map* field is a JSON object that contains 1 or more key/value pairs.  Each key is a variable name and the value is the variable's value.
        3. The keys in the *value_map* are arranged in alphabetic order.
10. A test program named *evaluator_test.rs* should be created perform the following function.
    1. Read *resources/complex_expressions.json*.
    2. For each JSON object read, call Evaluate::evaluate(input: &str, map: &HashMap<String, RuntimeValue>) with *expr* string and *value_map* used to construct the function's parameters.
    3. The output file, *evaluator_test.out*, should record (1) the number of successful and failed evaluate() calls and (2) total execution time.
    4. If any failures occur, the file *evaluator_test.failed* should contain (1) the JSON objects whose executions failed and (2) the error messages returned by evaluate().

Before planning for the Phase II implementation, we probably need to move or refactor the existing Phase I implementation.  Phase I processing generates simple expressions in *resources/simple_expressions.json*, which are then used by Phase II to generate complex expressions in *resources/complex_expressions.json*.  Currently, the code in the *src* directory only implements Phase I processing.  Please suggest a plan to reorganize the code so that Phase I and Phase II code can cohabitate in the same project.

Before planning for the Phase II implementation, please use version sqlexpr-rust version 1 from crates.io rather than depending on local source code.

### Phase II Tweaks

I notice that in *complex_expressions.json* the content of *value_map* fields are not sorted alphabetically by variable name.  Please make sure all *value_map* fields are sorted when written to file.

I also notice that in *complex_expressions.json* the complexity class 1 and 2 expressions allow for short-circuiting on sequences of OR clauses.  There may be other instances of short-cicuiting in higher complexity class expressions, though in at least some cases short-circuiting is handled properly.  Please modify the code so that short-circuiting never occurs.