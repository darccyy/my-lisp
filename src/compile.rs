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
    let indent = "  ".repeat(recursion);

    println!("{}\x1b[32mRUN\x1b[0m {:?}", indent, parsed);

    match parsed {
        Parsed::Item(item) => Item(Method::Literal(Int(item
            .parse()
            .expect(&format!("Could not parse {}", item))))),

        Parsed::List(list) => {
            let mut it = list.iter();

            let Some(op) = it.next() else {
                panic!("No operator");
            };

            println!("{}\x1b[36mOP\x1b[0m {:?}", indent, op);

            let Parsed::Item(op) = op else {
                let vec = list.iter().map(|item|compile_branch(item, recursion+1 )).collect();

                return Item(Method::Many(vec));
            };

            macro match_operator( $( $name: ident : $op: tt $( $arg: ident )* ),*  $(,)? ) {
                match op.as_str() {
                    $(
                        stringify!($op) => {
                            Method::$name(
                                $(
                                    box compile_branch(
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

                    op => panic!("Unknown operator '{}'", op),
                }
            }

            Item(match_operator! {
                Print: print value,
                Add: + rhs lhs,
                Mul: * rhs lhs,
            })
        }
    }
}
