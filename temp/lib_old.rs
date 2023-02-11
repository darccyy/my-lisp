use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

use List::*;
use Value::*;

#[derive(Clone)]
pub enum List {
    Item(String),
    More(Vec<List>),
}

// pub enum Value {
//     Null,
//     Int(i32),
// }

// impl Debug for Value {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Null => write!(f, "NULL"),

//             Int(int) => write!(f, "{}", int),
//         }
//     }
// }

// impl Add for Value {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         match (self, rhs) {
//             (Null, _) | (_, Null) => Null,
//             (Int(a), Int(b)) => Int(a + b),
//         }
//     }
// }

// impl Mul for Value {
//     type Output = Self;

//     fn mul(self, rhs: Self) -> Self::Output {
//         match (self, rhs) {
//             (Null, _) | (_, Null) => Null,
//             (Int(a), Int(b)) => Int(a * b),
//         }
//     }
// }

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item(item) => write!(f, "{:?}", item),
            More(vec) => write!(f, "{:?}", vec),
        }
    }
}

// pub enum

impl List {
    pub fn from(text: &str) -> Self {
        Self::parse_component(text, 0).0.first().unwrap().clone()
    }

    pub fn compile(self) -> Compiled {}

    // pub fn run(&self) -> Value {
    //     self.run_component(0)
    // }

    // fn run_component(&self, recursion: usize) -> Value {
    //     // Check recursion limit
    //     assert!(recursion < 20, "Max recursion [debug]");

    //     let indent = "  ".repeat(recursion);

    //     println!("{}\x1b[32mRUN\x1b[0m >> {:?}", indent, self);

    //     match self {
    //         Item(item) => Int(item.parse().expect("Not a number!")),

    //         More(list) => {
    //             let mut it = list.iter();

    //             println!("{}\x1b[36mOP\x1b[0m {:?}", indent, it);
    //             let op = match it.next().expect("Missing operator") {
    //                 Item(item) => item,
    //                 More(_) => panic!("Operator cannot be list"),
    //             };

    //             println!("{}\x1b[36mRHS\x1b[0m {:?}", indent, it);
    //             let rhs = it
    //                 .next()
    //                 .expect(&format!("Missing left hand side for '{}'", op))
    //                 .run_component(recursion + 1);

    //             println!("{}\x1b[36mLHS\x1b[0m {:?}", indent, it);
    //             let lhs = if op.as_str() == "print" {
    //                 Null
    //             } else {
    //                 it.next()
    //                     .expect(&format!("Missing right hand side for '{}'", op))
    //                     .run_component(recursion + 1)
    //             };

    //             match op.as_str() {
    //                 "print" => {
    //                     println!("\x1b[35m>>>\x1b[0m {:?}", rhs);
    //                     Null
    //                 }

    //                 "*" => rhs * lhs,
    //                 "+" => rhs + lhs,

    //                 _ => panic!("Unknown operator '{}'", op),
    //             }
    //         }
    //     }
}

/// Parse single component (branch) of `List` from slice of original string
///
/// No error handling!
///
/// `recursion` is an arbitrary limit of recursion
fn parse_component(text: &str, recursion: usize) -> (Vec<Self>, usize) {
    // Check recursion limit
    assert!(recursion < 20, "Max recursion [debug]");

    // List building
    let mut list: Vec<List> = Vec::new();
    let mut item_building = String::new();

    // Loop over characters
    // Enumeration is used for returning `i` variable inside loop (on ']' match arm)
    let mut chars = text.chars().enumerate();
    while let Some((i, ch)) = chars.next() {
        match ch {
            // Ignore line breaks and spaces that do not separate items
            '\n' => (),
            ' ' if item_building.is_empty() => (),

            // Space separates items
            ' ' => {
                if !item_building.is_empty() {
                    list.push(Item(item_building));
                    item_building = String::new();
                }
            }

            // Start new recurse
            '(' => {
                // Get rest of string, from current index
                // This can be optimized
                let rest = chars
                    .clone()
                    .map(|x| x.1.to_string())
                    .collect::<Vec<_>>()
                    .join("");

                // Recurse with same function, with `rest` string, increase recursion
                let (item_branch, increase_index) = Self::parse_component(&rest, recursion + 1);

                // Add new branch to list
                list.push(More(item_branch));

                // Increase index of loop
                chars.nth(increase_index);
            }

            // End this recurse
            // This arm nearly mirrors the final statements of this function
            ')' => {
                // Add final item to list
                if !item_building.is_empty() {
                    list.push(Item(item_building));
                }

                // Return current list as item branch, and current index of iterated string slice
                return (list, i);
            }

            // Add any other character to item building string
            _ => item_building.push(ch),
        }
    }

    // Add final item to list
    if !item_building.is_empty() {
        list.push(Item(item_building));
    }

    // Return final list, and dummy index
    // Index would be better as an `Option<usize>`
    (list, 0)
}
