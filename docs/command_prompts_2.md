
**THIS FILE SUPERSCEDES command_prompts.md, WHICH WAS USED FOR RELEASE 1.0.0 AND SHOULD BE IGNORED**

## Load Background Information

claude
/init
brave_web_search sqlexpr-rust at https://github.com/richcar58/sqlexpr-rust

# Multiple Language Expression Generation

## Introduction

The expressions generated in release 1.0.0 for load testing are appropriate for some parser implementations, but are not restrictive enough for others.  Specifically, some implementations prohibit the use of string data types for operands in the these clauses:  GreaterThan, GreaterOrEqual, LessThan, LessOrEqual and BETWEEN.  Additionally, some implementations do not not allow numerical data types to be used in IN clauses.  Our goal is to enhance sqlexpr-gen to generate load tests that target specific implementations of the SQL expression language.  The high level requirements are defined as follows:

1. Add a new command line argument, *--lang*, to indicate the level of expression language targeted.
2. Refactor the code to allow phase 1 generation to support multiple target languages.
3. Enhance the code to allow phase 1 and phase II generation to depend on the *--lang* setting.
4. Rename the generated resource files to indicate the *--lang* they target.

## Summary of Multiple Language Enhancements

1. Support two language levels, *max* and *limited*, specified with the *--lang* command line argument.
    1. *max* language support is essentially equal to the current phase I processing.
    2. *limited* language support is the new, restricted phase I processing whose code is not yet implemented.
    3. The default language is *max* if *--lang* is not specified.
    4. Enhancements to command line parsing code and code that will need to reference the *--lang* value.
2. Two phase I generators will be implemented.
    1. The current *src/phase1* code will be moved to *src/phase1/max* and continue to support the current language.
    2. The new phase I generation code will reside in *src/phase1/limited* and support the new, restricted language.
    3. Adjustments to configuration files and other code will be made as necessary for proper invocation of phase I processing.
3. The phase I output file name will be *resources/simple_expressions-\<lang>.json*, where \<lang> is replaced with either *max* or *limited*.
4. The phase II output file name will be *resources/complex_expressions-\<lang>.json*, where \<lang> is replaced with either *max* or *limited*.
    1. The default language is *max* if *--lang* is not specified.
    2. Phase II processing will abort if the language-specific *resources/simple_expressions-\<lang>.json* input file cannot be loaded.
5. Test programs such as *evaluator_test.rs* will also support the optional *--lang* command line argument.
    1. The default language is *max* if *--lang* is not specified.
    2. For any test program invocation, the language-specific *resources/complex_expressions-\<lang>.json* will be used.
    3. Include the target language in the program's output.
    4. Include the target language name in the program's output file names:
        1. evaluator_test-<lang>.out
        2. evaluator_test-<lang>.failed


The follow sections follow the design document format used to generate the 1.0.0 parser and evaluator, but with modifications to incorporate multiple language support.

## Phase 0 - Definitions and Conventions

We start with definitions that apply to the sqlexpr-rust language and implementation.  The addition of the **Language** definition is the only difference from the release 1.0.0 definitions.

1. **Expression Complexity** - The complexity of a boolean expression is the number of logical OR (disjuction) and AND (conjunction) operators it contains.
2. **Complexity Class** - A complexity class is the set of expressions with the same expression complexity.
2. **Conjunct** - An operand of a conjunction is a *conjunct*.
3. **Disjunct** - An operand of a disjunction is a *disjunct*.
4. **Relational Expression** - An expression defined as a *RelationalExpr* in ast.rs.  Specifically, these exressions are Equal, NotEqual, GreaterThan, GreaterOrEqual, LessThan, LessOrEqual, Like, Between, In and IsNull.
5. **Negated Relational Expression** - A Like, Between, In or IsNull expression that incorporates the logical NOT operator.  NOT can also precede other relational expressions. 
6. **Relational Operator** - An operator used in any relational expression.  Specifically, these operators are the Equal, NotEqual, GreaterThan, GreaterOrEqual, LessThan, LessOrEqual, Like, Between, In and Is tokens defined in lexer.rs.
7. **Language** - The name of the target expression language to be generated.  The currently defined languages are *max* and *limited*.

The names of variables in expressions follow these rules:

1. All variable names start with a single lowercase character followed by an integer.  The first character must reflect the variable's type as follows:
    1. *i* indicates integer values
    2. *f* indicates float values
    3. *s* indicates string values
    4. *b* indicates boolean values

## Phase I Design - Generating Simple Relational Expressions in the limited Language

This section details the requirements for the generation of simple, single operator relational expressions that conform to the *limited* expression language. The *max* language implementation and the integration of both languages into codebase are addressed in the following section, **Phase I Implementation Strategy**. The requirements in this section are similar to those for the release 1.0.0 code, but with some constructs removed to restrict the *limited* language.  How these requirements are used depends on the implementation plan we choose.  Here are the requirements:

1. Create a comprehensive list of the prototypical expressions for each operand type to which the operator can be applied.  For example, for the GreaterThan (>), the following simple expression could be generated:

    1. i1 > 7
    2. -3 > u2
    3. f1 > 22.4
    4. 50.1 > f2
    5. i3 > i4
    6. f3 > f4

2. The 9 prototypical expressions in the list above show all the ways one or two variables can be used in a simple GreaterThan expression.  The same number of prototypes can be generated for Equal, NotEqual, GreaterOrEqual, LessThan, and LessOrEqual.
3. Here are the prototypical expressions for the *Like* operator:
    1. s1 LIKE '%sometext'
    2. s2 LIKE '_sometext'
    3. s3 LIKE 'sometext%'
    4. s4 LIKE 'sometext_'
    5. s5 LIKE 'some%text'
    6. s6 LIKE 'some_ext'
    7. s3 LIKE 'some_text%'
    8. s4 LIKE 'some%text_'
4. Here are the prototypical expressions for the *BETWEEN* operator:
    1. i1 BETWEEN 1 AND 10
    2. f1 BETWEEN 1.0 AND 10.0
5. Here are the prototypical expressions for the *IN* operator:
    1. s1 IN ('apple', 'banana', 'cherry')
6. Here are the prototypical expressions for the *IS* operator:
    1. i1 IS NULL
    2. f1 IS NULL
    3. s1 IS NULL
7. For each expression with Like, Between, In or Is operators, create another expression with *NOT* properly inserted. 
8. The following restriction apply on certain operations:
    1. The second operand on LIKE operations must be a string.
    2. The second and third operands on BETWEEN operations are the lower and upper bounds (inclusive) and can only be numeric literals.
    3. The second operand on IN operations can only be a homegeneous list of strings.
9. Here are the prototypical expressions for simple boolean expressions:
    1. b1
    2. NOT b2
10. Do not generate relational expressions that contain only numeric or string or boolean constants.
11. Use different constants in each generated expression.
12. For each generated expression, produce two lists of values such that:
    1. A *true list* of 5 values that each return **true** processed by Evaluate::evaluate(input: &str, map: &HashMap<String, RuntimeValue>).  See the *evaluator_tests.rs* test code in sqlexpr-rust for usage examples.
    2. A *false list* of 5 values that each return **false** processed by Evaluate::evaluate(input: &str, map: &HashMap<String, RuntimeValue>).
    3. Each *true list* and *false list* is a list of 1 or more key/value pairs where the key is the variable name and the value is the variable value.  For example, *{i2: 88}* assigns integer value 88 to the variable named i2.  The elements of these lists should be sorted in alphabetic order of the variable name.
    4. All variable names are generated using a single monotonically increasing counter.  The counter is intialized at 1 and is incremented after its value is read during name generation.  The same counter is used to generate the names of variables of all types so that after all variable names have been generated, the counter's value will be one more than the number of variables.
    5. Do not allow duplicate entries in any *true_list* and *false_list* fields.  This may limit the size of some lists
    6. Creating the true and false lists also serves to validate that all expressions are well-formed and have properly typed values.
    7. No invocation of evaluate() should ever fail.  If a failure occurs, the root cause must be determined and fixed.
    8. Each *true list* and *false list* is a list of 1 or more key/value pairs where the key is the variable name and the value is the variable value.
13. Create a JSON file *resources/simple_expressions.json*, and populated it with the generated expressions and their validated value lists from previous steps.  The content should be formatted as an array of JSON objects, each with the following fields:
    1. *expr*, which is assigned an expression string
    2. *true_list*, which is assigned the list of values that cause the expression to evaluate to true
    3. *false_list*, which is assigned the list of values that cause the expression to evaluate to false
14, Populate *resources/simple_expressions.json* with the generated expressions and their validated value lists from previous steps.

## Phase I Implementation Strategy

### Refactoring *max* Language Code Generation

The existing release 1.0.0 language is now named *max*.  The current phase I generation code needs to be moved to a new directory and to support for all current functionality.  The necessary changes include:

1. Move the current *src/phase1* code to *src/phase1/max*.
2. Change to all code, including module and configuration files, to reference the new code location.
3. Store of the language name (*max*) in a single place accessible to all generation code.
4. Write output to the language-specific file named *resources/simple_expressions-max.json*.

Other than the above changes, the actual logic of the existing phase I should remain essentially unchanged.

### Implementing *limited* Language Code Generation

The new language name is *llmited* and its code needs to be generated in the *src/phase1/limited* directory.  The *limited* language is a proper subset of the *max* language, so its code can be implemented as follows:

1. Copy the *max* code generator to *src/phase1/limited*.
2. Remove support for constructs that are part of the *max* language but not part of the *limited* language.
    1. Disallow the use of string data types for operands in GreaterThan, GreaterOrEqual, LessThan, LessOrEqual and BETWEEN clauses.
    2. Disallow the use of numeric operands in IN clauses.
3. Substitute references to the *max* with references to *limited*.
    1. Write output to the language-specific file named *resources/simple_expressions-limited.json*.

### Integrating Multiple Language Support

The main points of integration for phase I processing are as follows:

1. The build, run and test instructions in README.md should continue to work as is with support for the *--lang* argument where necessary.
2. The *--lang* command line parameter's value must be accessible to all phase I generation code.
3. The current *resources/simple_expressions.json* JSON format must be used in both language-specific output files.

Please compare the above implementation strategy with other possible strategies for implementing the phase I requirements.      

### Tweaks to Generated Code

1. cli.rs panics when the argument starts with "--lang=".  Please output an informative error message when there's problem with argument parsing.
2. The next issue is a little more complicated.  When using the limited language, only 71 simple expressions are generated in simple_expressions-limited.json.  Phase II processing builds complex
  logical expressions that can contain as many as 75 distinct simple expressions.  Since we don't allow the same simple expression to be used more than once per complex expression, we don't have enough simple
  expressions to populate complex expressions of the highest complexity class.  Please suggest solutions to this problem that in Phase II preserve (1) the uniqueness requirement for simple expressions complex
  expressions and (2) allows for expressions of complexity classes of at least 75.  Recommend plans to implement the suggested solutions without generating any code yet.

  Let's go with a version of Solution 1: Generate Additional Compliant Templates, Option 1B: Add Arithmetic Expression Variations.  Create 2 sets of arithmetic templates with each set containing all of the
  comparison operators =, !=, >, >=, <, and  <=.  One set of templates will use integer variables and the other set float variables.  For the 6 templates that use integer variables, have 2 incorporate addition
  (+), 2 incorporate substraction (-) and 2 incorporate multiply (*).  For the 6 templates that use float variable, have 3 use addition (+) and 3 use subtraction (-).  A total of 12 new templates will be
  introduced, 6 using integer variables and 6 using float variables.

Excellent Work!  Let's update RELEASE_NOTES.md with the enhancements that were made to release 1.1.0.  Start the new release's section with the following heading:

## Version 1.1.0 (2025-12-31)