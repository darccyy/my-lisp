use std::fmt::Debug;

use crate::{parse::Tree, value::Value};

/// Compiled function
#[derive(Clone)]
pub enum Method {
    /// List of many compiled items
    Many(Vec<Method>),
    /// Literal value
    Literal(Value),
    /// Print value to stdout
    Print(Box<Method>),
    /// Add values
    Add(Box<Method>, Box<Method>),
    /// Multiply values
    Mul(Box<Method>, Box<Method>),
}

impl Debug for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Many(vec) => {
                write!(
                    f,
                    "\x1b[35m{{\x1b[0m\n{}\n\x1b[35m}}\x1b[0m",
                    vec.iter()
                        .map(|item| format!("  {:?}", item))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }

            Method::Literal(value) => write!(f, "{:?}", value),

            Method::Print(value) => write!(f, "\x1b[34mPrint\x1b[0m {:?}", value),

            Method::Add(rhs, lhs) => write!(
                f,
                "\x1b[36m(\x1b[0m{:?} \x1b[35m+\x1b[0m {:?}\x1b[36m)\x1b[0m",
                rhs, lhs
            ),
            Method::Mul(rhs, lhs) => write!(
                f,
                "\x1b[36m(\x1b[0m{:?} \x1b[35m*\x1b[0m {:?}\x1b[36m)\x1b[0m",
                rhs, lhs
            ),
        }
    }
}

impl Method {
    /// Compile script from parsed text
    pub fn from(parsed: Tree) -> Self {
        compile_branch(&parsed, 0)
    }
}

/// Compile single branch, recursively
fn compile_branch(parsed: &Tree, recursion: usize) -> Method {
    match parsed {
        // Single item
        // Parse value - must be literal
        Tree::Item(item) => Method::Literal(Value::from(item)),

        // List of items
        Tree::List(list) => {
            let mut it = list.iter();

            // Get operator of method
            // No operator must mean empty brackets
            let Some(op) = it.next() else {
                panic!("No operator - Empty brackets");
            };

            // Unwrap operator
            let Tree::Item(op) = op else {
                // Otherwise return list of many compiled methods
                return Method::Many(
                    list.iter()
                        .map(|item| compile_branch(item, recursion + 1))
                        .collect()
                    );
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
            match_operator! {
                Print: print value,
                Add: + lhs rhs,
                Mul: * lhs rhs,
            }
        }
    }
}
