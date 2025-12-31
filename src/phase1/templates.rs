use crate::common::Language;

/// Variable name counter for generating unique variable names
pub struct VarCounter {
    counter: usize,
}

impl VarCounter {
    pub fn new() -> Self {
        Self { counter: 1 }
    }

    /// Generate next integer variable name (e.g., "i1", "i2", ...)
    pub fn next_int(&mut self) -> String {
        let name = format!("i{}", self.counter);
        self.counter += 1;
        name
    }

    /// Generate next float variable name (e.g., "f3", "f4", ...)
    pub fn next_float(&mut self) -> String {
        let name = format!("f{}", self.counter);
        self.counter += 1;
        name
    }

    /// Generate next string variable name (e.g., "s5", "s6", ...)
    pub fn next_string(&mut self) -> String {
        let name = format!("s{}", self.counter);
        self.counter += 1;
        name
    }

    /// Generate next boolean variable name (e.g., "b7", "b8", ...)
    pub fn next_bool(&mut self) -> String {
        let name = format!("b{}", self.counter);
        self.counter += 1;
        name
    }

    /// Get current counter value
    pub fn current(&self) -> usize {
        self.counter
    }
}

/// Value types supported in expressions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Integer,
    Float,
    String,
    Boolean,
}

/// Pattern for an operand in an expression
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Variable(String),      // e.g., "i1", "f2", "s3", "b1"
    Literal(LiteralValue), // Concrete value
}

/// Literal values that can appear in expressions
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Integer(i) => i.to_string(),
            LiteralValue::Float(f) => f.to_string(),
            LiteralValue::String(s) => format!("'{}'", s),
        }
    }
}

impl Pattern {
    pub fn to_string(&self) -> String {
        match self {
            Pattern::Variable(name) => name.clone(),
            Pattern::Literal(lit) => lit.to_string(),
        }
    }
}

/// Template for comparison operators: =, <>, !=, >, >=, <, <=
#[derive(Debug, Clone)]
pub struct ComparisonTemplate {
    pub operator: &'static str,
    pub left: Pattern,
    pub right: Pattern,
    pub value_type: ValueType,
}

/// Template for LIKE and NOT LIKE operators
#[derive(Debug, Clone)]
pub struct LikeTemplate {
    pub not: bool,              // true for NOT LIKE
    pub variable: String,
    pub pattern: String,
}

/// Template for BETWEEN and NOT BETWEEN operators
#[derive(Debug, Clone)]
pub struct BetweenTemplate {
    pub not: bool,              // true for NOT BETWEEN
    pub variable: String,
    pub lower: LiteralValue,
    pub upper: LiteralValue,
    pub value_type: ValueType,
}

/// Template for IN and NOT IN operators
#[derive(Debug, Clone)]
pub struct InTemplate {
    pub not: bool,              // true for NOT IN
    pub variable: String,
    pub values: Vec<LiteralValue>,
    pub value_type: ValueType,
}

/// Template for IS NULL and IS NOT NULL operators
#[derive(Debug, Clone)]
pub struct IsNullTemplate {
    pub not: bool,              // true for IS NOT NULL
    pub variable: String,
    pub value_type: ValueType,
}

/// Template for simple boolean expressions
#[derive(Debug, Clone)]
pub struct BooleanTemplate {
    pub not: bool,              // true for NOT b
    pub variable: String,
}

/// Template for arithmetic comparison expressions: var {+|-|*} literal {cmp_op} literal
#[derive(Debug, Clone)]
pub struct ArithmeticComparisonTemplate {
    pub left_var: String,          // e.g., "i1"
    pub arith_op: &'static str,    // +, -, or *
    pub arith_operand: LiteralValue,  // operand for arithmetic (e.g., 10)
    pub cmp_op: &'static str,      // =, !=, >, >=, <, <=
    pub right: LiteralValue,       // comparison target (e.g., 20)
    pub value_type: ValueType,     // Integer or Float
}

/// Unified template enum
#[derive(Debug, Clone)]
pub enum Template {
    Comparison(ComparisonTemplate),
    Like(LikeTemplate),
    Between(BetweenTemplate),
    In(InTemplate),
    IsNull(IsNullTemplate),
    Boolean(BooleanTemplate),
    ArithmeticComparison(ArithmeticComparisonTemplate),
}

/// Get all templates for Phase I test generation
pub fn get_all_templates(lang: Language) -> Vec<Template> {
    let mut templates = Vec::new();
    let mut counter = VarCounter::new();

    // Comparison operators
    templates.extend(get_comparison_templates(&mut counter, lang));

    // LIKE operators
    templates.extend(get_like_templates(&mut counter));

    // BETWEEN operators
    templates.extend(get_between_templates(&mut counter, lang));

    // IN operators
    templates.extend(get_in_templates(&mut counter, lang));

    // IS NULL operators
    templates.extend(get_is_null_templates(&mut counter));

    // Boolean expressions
    templates.extend(get_boolean_templates(&mut counter));

    // Arithmetic comparison expressions
    templates.extend(get_arithmetic_comparison_templates(&mut counter));

    println!("Generated {} templates using {} unique variables", templates.len(), counter.current() - 1);

    templates
}

/// Generate comparison templates (9 prototypes × 6 operators = 54 templates)
fn get_comparison_templates(counter: &mut VarCounter, lang: Language) -> Vec<Template> {
    let operators = vec!["=", "<>", ">", ">=", "<", "<="];
    let mut templates = Vec::new();

    for operator in operators {
        // Integer comparisons (9 prototypes)
        templates.push(Template::Comparison(ComparisonTemplate {
            operator,
            left: Pattern::Variable(counter.next_int()),
            right: Pattern::Literal(LiteralValue::Integer(7)),
            value_type: ValueType::Integer,
        }));
        templates.push(Template::Comparison(ComparisonTemplate {
            operator,
            left: Pattern::Literal(LiteralValue::Integer(-3)),
            right: Pattern::Variable(counter.next_int()),
            value_type: ValueType::Integer,
        }));
        templates.push(Template::Comparison(ComparisonTemplate {
            operator,
            left: Pattern::Variable(counter.next_int()),
            right: Pattern::Variable(counter.next_int()),
            value_type: ValueType::Integer,
        }));

        // Float comparisons (3 prototypes)
        templates.push(Template::Comparison(ComparisonTemplate {
            operator,
            left: Pattern::Variable(counter.next_float()),
            right: Pattern::Literal(LiteralValue::Float(22.4)),
            value_type: ValueType::Float,
        }));
        templates.push(Template::Comparison(ComparisonTemplate {
            operator,
            left: Pattern::Literal(LiteralValue::Float(50.1)),
            right: Pattern::Variable(counter.next_float()),
            value_type: ValueType::Float,
        }));
        templates.push(Template::Comparison(ComparisonTemplate {
            operator,
            left: Pattern::Variable(counter.next_float()),
            right: Pattern::Variable(counter.next_float()),
            value_type: ValueType::Float,
        }));

        // String comparisons (3 prototypes)
        // Limited language: exclude string comparisons for >, >=, <, <=
        let is_ordering_op = operator == ">" || operator == ">=" || operator == "<" || operator == "<=";
        if lang == Language::Max || !is_ordering_op {
            templates.push(Template::Comparison(ComparisonTemplate {
                operator,
                left: Pattern::Variable(counter.next_string()),
                right: Pattern::Literal(LiteralValue::String("banana".to_string())),
                value_type: ValueType::String,
            }));
            templates.push(Template::Comparison(ComparisonTemplate {
                operator,
                left: Pattern::Literal(LiteralValue::String("united".to_string())),
                right: Pattern::Variable(counter.next_string()),
                value_type: ValueType::String,
            }));
            templates.push(Template::Comparison(ComparisonTemplate {
                operator,
                left: Pattern::Variable(counter.next_string()),
                right: Pattern::Variable(counter.next_string()),
                value_type: ValueType::String,
            }));
        }
    }

    templates
}

/// Generate LIKE templates (8 LIKE + 8 NOT LIKE = 16 templates)
fn get_like_templates(counter: &mut VarCounter) -> Vec<Template> {
    let mut templates = Vec::new();
    let patterns = vec![
        "%sometext",
        "_sometext",
        "sometext%",
        "sometext_",
        "some%text",
        "some_text",
        "some_text%",
        "some%text_",
    ];

    for pattern in patterns.iter() {
        // LIKE
        templates.push(Template::Like(LikeTemplate {
            not: false,
            variable: counter.next_string(),
            pattern: pattern.to_string(),
        }));

        // NOT LIKE
        templates.push(Template::Like(LikeTemplate {
            not: true,
            variable: counter.next_string(),
            pattern: pattern.to_string(),
        }));
    }

    templates
}

/// Generate BETWEEN templates (3 BETWEEN + 3 NOT BETWEEN = 6 templates)
fn get_between_templates(counter: &mut VarCounter, lang: Language) -> Vec<Template> {
    let mut templates = vec![
        // Integer BETWEEN
        Template::Between(BetweenTemplate {
            not: false,
            variable: counter.next_int(),
            lower: LiteralValue::Integer(1),
            upper: LiteralValue::Integer(10),
            value_type: ValueType::Integer,
        }),
        Template::Between(BetweenTemplate {
            not: true,
            variable: counter.next_int(),
            lower: LiteralValue::Integer(20),
            upper: LiteralValue::Integer(30),
            value_type: ValueType::Integer,
        }),

        // Float BETWEEN
        Template::Between(BetweenTemplate {
            not: false,
            variable: counter.next_float(),
            lower: LiteralValue::Float(1.0),
            upper: LiteralValue::Float(10.0),
            value_type: ValueType::Float,
        }),
        Template::Between(BetweenTemplate {
            not: true,
            variable: counter.next_float(),
            lower: LiteralValue::Float(20.0),
            upper: LiteralValue::Float(30.0),
            value_type: ValueType::Float,
        }),
    ];

    // String BETWEEN - only in Max language
    if lang == Language::Max {
        templates.extend(vec![
            Template::Between(BetweenTemplate {
                not: false,
                variable: counter.next_string(),
                lower: LiteralValue::String("aa".to_string()),
                upper: LiteralValue::String("bb".to_string()),
                value_type: ValueType::String,
            }),
            Template::Between(BetweenTemplate {
                not: true,
                variable: counter.next_string(),
                lower: LiteralValue::String("mm".to_string()),
                upper: LiteralValue::String("zz".to_string()),
                value_type: ValueType::String,
            }),
        ]);
    }

    templates
}

/// Generate IN templates (3 IN + 3 NOT IN = 6 templates)
fn get_in_templates(counter: &mut VarCounter, lang: Language) -> Vec<Template> {
    let mut templates = vec![];

    // Numeric IN - only in Max language
    if lang == Language::Max {
        templates.extend(vec![
            // Integer IN
            Template::In(InTemplate {
                not: false,
                variable: counter.next_int(),
                values: vec![
                    LiteralValue::Integer(1),
                    LiteralValue::Integer(2),
                    LiteralValue::Integer(3),
                ],
                value_type: ValueType::Integer,
            }),
            Template::In(InTemplate {
                not: true,
                variable: counter.next_int(),
                values: vec![
                    LiteralValue::Integer(10),
                    LiteralValue::Integer(20),
                    LiteralValue::Integer(30),
                ],
                value_type: ValueType::Integer,
            }),
            // Float IN
            Template::In(InTemplate {
                not: false,
                variable: counter.next_float(),
                values: vec![
                    LiteralValue::Float(1.0),
                    LiteralValue::Float(2.0),
                    LiteralValue::Float(3.0),
                ],
                value_type: ValueType::Float,
            }),
            Template::In(InTemplate {
                not: true,
                variable: counter.next_float(),
                values: vec![
                    LiteralValue::Float(10.0),
                    LiteralValue::Float(20.0),
                    LiteralValue::Float(30.0),
                ],
                value_type: ValueType::Float,
            }),
        ]);
    }

    // String IN - always included
    templates.extend(vec![
        Template::In(InTemplate {
            not: false,
            variable: counter.next_string(),
            values: vec![
                LiteralValue::String("apple".to_string()),
                LiteralValue::String("banana".to_string()),
                LiteralValue::String("cherry".to_string()),
            ],
            value_type: ValueType::String,
        }),
        Template::In(InTemplate {
            not: true,
            variable: counter.next_string(),
            values: vec![
                LiteralValue::String("dog".to_string()),
                LiteralValue::String("elephant".to_string()),
                LiteralValue::String("fox".to_string()),
            ],
            value_type: ValueType::String,
        }),
    ]);

    templates
}

/// Generate IS NULL templates (3 IS NULL + 3 IS NOT NULL = 6 templates)
fn get_is_null_templates(counter: &mut VarCounter) -> Vec<Template> {
    vec![
        // Integer IS NULL
        Template::IsNull(IsNullTemplate {
            not: false,
            variable: counter.next_int(),
            value_type: ValueType::Integer,
        }),
        Template::IsNull(IsNullTemplate {
            not: true,
            variable: counter.next_int(),
            value_type: ValueType::Integer,
        }),

        // Float IS NULL
        Template::IsNull(IsNullTemplate {
            not: false,
            variable: counter.next_float(),
            value_type: ValueType::Float,
        }),
        Template::IsNull(IsNullTemplate {
            not: true,
            variable: counter.next_float(),
            value_type: ValueType::Float,
        }),

        // String IS NULL
        Template::IsNull(IsNullTemplate {
            not: false,
            variable: counter.next_string(),
            value_type: ValueType::String,
        }),
        Template::IsNull(IsNullTemplate {
            not: true,
            variable: counter.next_string(),
            value_type: ValueType::String,
        }),
    ]
}

/// Generate boolean expression templates
fn get_boolean_templates(counter: &mut VarCounter) -> Vec<Template> {
    vec![
        Template::Boolean(BooleanTemplate {
            not: false,
            variable: counter.next_bool(),
        }),
        Template::Boolean(BooleanTemplate {
            not: true,
            variable: counter.next_bool(),
        }),
    ]
}

/// Generate arithmetic comparison templates (12 templates)
/// Integer: 6 templates (2 add, 2 sub, 2 mul)
/// Float: 6 templates (3 add, 3 sub)
fn get_arithmetic_comparison_templates(counter: &mut VarCounter) -> Vec<Template> {
    let mut templates = Vec::new();

    // Integer arithmetic comparisons (6 templates)
    // Addition: =, !=
    templates.push(Template::ArithmeticComparison(ArithmeticComparisonTemplate {
        left_var: counter.next_int(),
        arith_op: "+",
        arith_operand: LiteralValue::Integer(10),
        cmp_op: "=",
        right: LiteralValue::Integer(20),
        value_type: ValueType::Integer,
    }));

    templates.push(Template::ArithmeticComparison(ArithmeticComparisonTemplate {
        left_var: counter.next_int(),
        arith_op: "+",
        arith_operand: LiteralValue::Integer(5),
        cmp_op: "!=",
        right: LiteralValue::Integer(15),
        value_type: ValueType::Integer,
    }));

    // Subtraction: >, >=
    templates.push(Template::ArithmeticComparison(ArithmeticComparisonTemplate {
        left_var: counter.next_int(),
        arith_op: "-",
        arith_operand: LiteralValue::Integer(7),
        cmp_op: ">",
        right: LiteralValue::Integer(10),
        value_type: ValueType::Integer,
    }));

    templates.push(Template::ArithmeticComparison(ArithmeticComparisonTemplate {
        left_var: counter.next_int(),
        arith_op: "-",
        arith_operand: LiteralValue::Integer(3),
        cmp_op: ">=",
        right: LiteralValue::Integer(20),
        value_type: ValueType::Integer,
    }));

    // Multiplication: <, <=
    templates.push(Template::ArithmeticComparison(ArithmeticComparisonTemplate {
        left_var: counter.next_int(),
        arith_op: "*",
        arith_operand: LiteralValue::Integer(2),
        cmp_op: "<",
        right: LiteralValue::Integer(100),
        value_type: ValueType::Integer,
    }));

    templates.push(Template::ArithmeticComparison(ArithmeticComparisonTemplate {
        left_var: counter.next_int(),
        arith_op: "*",
        arith_operand: LiteralValue::Integer(3),
        cmp_op: "<=",
        right: LiteralValue::Integer(50),
        value_type: ValueType::Integer,
    }));

    // Float arithmetic comparisons (6 templates)
    // Addition: =, >, <
    templates.push(Template::ArithmeticComparison(ArithmeticComparisonTemplate {
        left_var: counter.next_float(),
        arith_op: "+",
        arith_operand: LiteralValue::Float(5.0),
        cmp_op: "=",
        right: LiteralValue::Float(15.0),
        value_type: ValueType::Float,
    }));

    templates.push(Template::ArithmeticComparison(ArithmeticComparisonTemplate {
        left_var: counter.next_float(),
        arith_op: "+",
        arith_operand: LiteralValue::Float(2.5),
        cmp_op: ">",
        right: LiteralValue::Float(10.0),
        value_type: ValueType::Float,
    }));

    templates.push(Template::ArithmeticComparison(ArithmeticComparisonTemplate {
        left_var: counter.next_float(),
        arith_op: "+",
        arith_operand: LiteralValue::Float(1.0),
        cmp_op: "<",
        right: LiteralValue::Float(20.0),
        value_type: ValueType::Float,
    }));

    // Subtraction: !=, >=, <=
    templates.push(Template::ArithmeticComparison(ArithmeticComparisonTemplate {
        left_var: counter.next_float(),
        arith_op: "-",
        arith_operand: LiteralValue::Float(3.0),
        cmp_op: "!=",
        right: LiteralValue::Float(7.0),
        value_type: ValueType::Float,
    }));

    templates.push(Template::ArithmeticComparison(ArithmeticComparisonTemplate {
        left_var: counter.next_float(),
        arith_op: "-",
        arith_operand: LiteralValue::Float(2.0),
        cmp_op: ">=",
        right: LiteralValue::Float(5.0),
        value_type: ValueType::Float,
    }));

    templates.push(Template::ArithmeticComparison(ArithmeticComparisonTemplate {
        left_var: counter.next_float(),
        arith_op: "-",
        arith_operand: LiteralValue::Float(1.5),
        cmp_op: "<=",
        right: LiteralValue::Float(10.0),
        value_type: ValueType::Float,
    }));

    templates
}
