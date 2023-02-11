use std::fmt::Debug;

use crate::parse::Parsed;

use Compiled::*;
use Value::*;

#[derive(Clone)]
pub enum Value {
    Null,
    Int(i32),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Null => write!(f, "ɴᴜʟʟ"),
            Int(int) => write!(f, "ɪɴᴛ:{}", int),
        }
    }
}

impl Value {
    pub fn from(string: &str) -> Self {
        if string == "NUL" {
            return Null;
        }

        Int(string
            .parse()
            .expect(&format!("Could not parse '{}' as integer", string)))
    }
}

#[derive(Clone)]
pub enum Method {
    Many(Vec<Compiled>),
    Literal(Value),
    Print(Box<Compiled>),
    Add(Box<Compiled>, Box<Compiled>),
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

            Method::Print(value) => write!(f, "Print [{:?}]", value),

            Method::Add(rhs, lhs) => write!(f, "[{:?}] + [{:?}]", rhs, lhs),
            Method::Mul(rhs, lhs) => write!(f, "[{:?}] * [{:?}]", rhs, lhs),
        }
    }
}

#[derive(Clone)]
pub enum Compiled {
    Item(Method),
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

pub fn compile(parsed: Parsed) -> Compiled {
    compile_branch(&parsed, 0)
}

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
