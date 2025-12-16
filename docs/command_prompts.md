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

**UNDER CONSTRUCTION**

## Phase II Design - Generating Complex Relational Expressions

In this phase, we combine simple expressions generated in Phase I to yield complex expresses that will be used in performance load testing.  Complex expression strings along with their value mappings are used as input into Evaluate::evaluate(input: &str, map: &HashMap<String, RuntimeValue>) during load testing.  The defining characteristics of a complex expression generation are:

1. The simple expressions added to a complex expression are randomly chosen from among the top-level objects in *resources/simple_expressions.json*.
2. Each selected simple expression may be transformed.
3. Each selected simple expression is always parenthesized to improve readability and avoid unexpected capture.
4. Sequences of two or more OR clauses are always parenthisized.  This implies that before and after such a sequence there is either (1) no logical operator, or (2) an AND operator, or (3) a NOT operator.
5. Short circuiting that provides for early termination of conjuctive and disjunctive expressions must be avoided by adhering to these rules:
    1. The variables in each AND clause will be assigned values such that the clause to evaluate to true. 
    2. Only the last clause in a disjuction of two or more OR operators will be assigned values that cause that clause to evaluate to true, all other clauses will evaluate to false.
6. The value mapping chosen for each complex expression covers all variables in the expression.  The elements of the mapping are listed in alphabetic order by variable name.

### The Generation Process

Complex expression strings are constructed using the simple expressions and their associated value lists from *resources/simple_expressions.json*.  The defining characteristics of a complex expression are:

1. The process is driven by the goal of generating 500 unique combinations of expressions and value maps for each complexity class listed here:
    1. Complexity classes for each integer 1 to 10, inclusive.
    2. Complexity classes 15, 20, 25, 30, 35, 40, 45, 50.
    3. Complexity classes 75, 100.
2. 

2. For each complexity class, generation of a complex expression begins with these actions:
    1. Select at random a JSON object from the top-level array in *resources/simple_expressions.json*.
    2. Randomly select either the AND or OR operator.
    3. Use the selected operator and rules in the Simple Expression Transformation Rules section below to transform the JSON object.



#### Simple Expression Transformation Rules

Given a JSON object from the top-level array in *resources/simple_expressions.json* and either an AND or OR operator, perform these transformations before incorporating it into a complex expression.

1. Parenthesize the expression string in the *expr* field.
2. Choose the value list depending on the operation.
    1. If an AND operator was