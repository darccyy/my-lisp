use std::fmt::Debug;

use crate::parse::Parsed;

use Compiled::*;
use Value::*;

/// Weakly-typed variable value
///
/// Similar to JSON
#[derive(Clone)]
pub enum Value {
    /// Null value
    Null,
    /// Integer
    Int(i32),
    /// String
    Str(String),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Null => write!(f, "ɴᴜʟʟ"),
            Int(int) => write!(f, "ɪɴᴛ {}", int),
            Str(string) => write!(f, "sᴛʀ '{}'", string),
        }
    }
}

impl Value {
    /// Parse `Value` from string
    pub fn from(string: &str) -> Self {
        // Null
        if string == "NUL" {
            return Null;
        }

        // String
        if string.starts_with('\'') && string.ends_with('\'')
            || string.starts_with('"') && string.ends_with('"')
        {
            // Remove first and last characters
            let mut chars = string.chars();
            chars.next();
            chars.next_back();
            return Str(chars.as_str().to_string());
        }

        // Integer
        Int(string
            .parse()
            .expect(&format!("Could not parse '{}' as integer", string)))
    }
}

/// Compiled function
#[derive(Clone)]
pub enum Method {
    /// List of many compiled items
    Many(Vec<Compiled>),
    /// Literal value
    Literal(Value),
    /// Print value to stdout
    Print(Box<Compiled>),
    /// Add values
    Add(Box<Compiled>, Box<Compiled>),
    /// Multiply values
    Mul(Box<Compiled>, Box<Compiled>),
}

impl Debug for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Many(vec) => {
                write!(
                    f,
                    "{{\n{}\n}}",
                    vec.iter()
                        .map(|item| format!("  {:?};", item))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }

            Method::Literal(value) => write!(f, "{:?}", value),

            Method::Print(value) => write!(f, "Print {:?}", value),

            Method::Add(rhs, lhs) => write!(f, "({:?} + {:?})", rhs, lhs),
            Method::Mul(rhs, lhs) => write!(f, "({:?} * {:?})", rhs, lhs),
        }
    }
}

/// Compiled script object
#[derive(Clone)]
pub enum Compiled {
    /// Single method item
    Item(Method),
    /// List of compiled branches
    List(Vec<Compiled>),
}

impl Debug for Compiled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item(item) => write!(f, "{:?}", item),
            List(list) => write!(f, "{:?}", list),
        }
    }
}

impl Compiled {
    /// Compile script from parsed text
    pub fn from(parsed: Parsed) -> Self {
        compile_branch(&parsed, 0)
    }
}

/// Compile single branch, recursively
fn compile_branch(parsed: &Parsed, recursion: usize) -> Compiled {
    match parsed {
        // Single item
        // Parse value - must be literal
        Parsed::Item(item) => Item(Method::Literal(Value::from(item))),

        // List of items
        Parsed::List(list) => {
            let mut it = list.iter();

            // Get operator of method
            // No operator must mean empty brackets
            let Some(op) = it.next() else {
                panic!("No operator - Empty brackets");
            };

            // Unwrap operator
            let Parsed::Item(op) = op else {
                // Otherwise return list of many compiled methods
                return Item(Method::Many(
                    list.iter()
                        .map(|item| compile_branch(item, recursion + 1))
                        .collect()
                    ));
            };

            /// Match operator and arguments to method
            macro match_operator(
                // List of methods, operators, and arguments
                $(
                    // Name of method
                    $method: ident
                    // Operator, as token tree (so that arithmetic operators can be used)
                    : $op: tt
                    // List of arguments names, as identifiers
                    $( $arg: ident )*
                ),*  $(,)?
            ) {
                // Match operator
                match op.as_str() {
                    $(
                        // Operator
                        stringify!($op) => {
                            // Return matching method
                            Method::$method(
                                $(
                                    // Include arguments, compiled recursively
                                    box compile_branch(
                                        // Next item of iterated list
                                        it.next().expect(&format!(
                                            "Missing argument '{}', for '{}' operator",
                                            stringify!($arg),
                                            stringify!($op),
                                        )),
                                        recursion + 1,
                                    ),
                                )*
                            )
                        }
                    )*

                    // Fallback - Error
                    op => panic!("Unknown operator '{}'", op),
                }
            }

            // Match operators
            Item(match_operator! {
                Print: print value,
                Add: + rhs lhs,
                Mul: * rhs lhs,
            })
        }
    }
}
