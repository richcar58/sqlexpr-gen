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
    2. Complexity classes 15, 20, 25, 30, 35, 40, 45, 50.
    3. Complexity classes 75, 100.
2. The simple expressions that are incorporated into each complex expression are randomly chosen from the set of all expresions in *resources/simple_expressions.json*.
3. Simple expressions are joined together by AND or OR operators to incrementally build a complex expression.
4. All selected simple expression are enclosed in parentheses.  This is to improve readability and avoid unexpected capture.
5. A random sampling of simple expressions are arithmetically transformed before being added into a complex expression.
    1. To be a candidate for transformation, a simple expression must meet these qualifications:
        1. Only simple expressions of numeric type can be transformed.  Specifically, expression that contain integer or float variables.
        2. Only expressions that contain an Equal, NotEqual, GreaterThan, GreaterOrEqual, LessThan or LessOrEqual relational operator can be transformed.  
    2. Approximately 20% of candidate expressions get transformed.  The candidates transformed are randomly chosen for each complex expression; different complex expressions will have different simple expressions transformed.  
    3. An arimethic transformation is applied using these rules:
        1. An integer between 2 and 100 is randomly chosen.
        2. An arithmetic operator is randomly chosen.  Possible choices are *\*, + and -* (multiply, add and subtract). 
        3. The chosen arithmetic operation and integer are concatenated to the text to the left and to the right of the simple expression's relational operator.  For example, if *33* and *+** were chosen to transform the expression, *i2 > 6*, then the transformed expression would be *i2 + 33 > 6 + 33*.
    4. Note that the specified arithmetic transformations have the following characteristics:
        1. Since the same operation is applied to both the left and right side of the relational expression, the *true_list* and *false_list* values continue to have their desired effect.
        2. Because of type coersion, newly inserted integer values will be automatically converted to float during evaluation.
6. Sequences of two or more OR clauses are always parenthesized.  This implies that before and after such a sequence there is either (1) no logical operator, or (2) an AND operator, or (3) a NOT operator.
7. Short circuiting terminates conjuctive and disjunctive expression evaluation early.  This defeats the goal of maximizing test load, so we defeat short circuiting by following these rules:
    1. The variables in each AND clause are assigned values such that each clause evaluates to true. 
    2. In parenthesized sequence of OR clauses, only the last clause is assigned values that to evaluate to true, all other clauses evaluate to false.
    3. Standalone OR clauses, i.e., those that are not part of a parenthesized sequence of OR clauses, can take on any value.
8. The value mapping chosen for each complex expression covers all variables in the expression.  The elements of the mapping are outputted in alphabetic order by variable name.

### Details of the Generation Process

Complex expression strings are constructed using simple expressions and their associated value lists from *resources/simple_expressions.json*.  

2. 
3. To increase the load of complex expression evaluation, simple expressions may be transformed before they are inserted into a complex expression.  The possible transformations are defined below in the **Simple Expression Transformation Rules** section.  
4.  
5. The first logical operator (AND or OR) to appear in a complex expression is randomly selected.
    1. Every subsequent operator is selected using a weighed 60/40 probability.  60% of the time the next operator selected is the same as last operator selected; 40% of the time it differs from the last operator.  This bias slightly favors creating moderate length sequences of either AND or OR operators.
6. Using the short circuiting rule specified above, either the *true_list* or the *false_list* of a given simple expression is selected as its value source.  The actual value chosen from the selected list is random.



#### Simple Expression Transformation Rules

Given a JSON object from the top-level array in *resources/simple_expressions.json* and either an AND or OR operator, perform these transformations before incorporating it into a complex expression.

1. Parenthesize the expression string in the *expr* field.
2. Choose the value list depending on the operation.
    1. If an AND operator was