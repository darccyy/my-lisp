use std::fmt::Debug;

use Parsed::*;

use crate::Compiled;

/// Parsed, uncompiled text
#[derive(Clone)]
pub enum Parsed {
    /// Single item token
    Item(String),
    /// List of items
    List(Vec<Parsed>),
}

impl Debug for Parsed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item(item) => write!(f, "{:?}", item),
            List(list) => write!(f, "{:?}", list),
        }
    }
}

impl Parsed {
    /// Parse uncompiled script from text
    pub fn from(text: &str) -> Parsed {
        List(parse_branch(text, 0).0)
    }

    /// Compile script from parsed text
    pub fn compile(self) -> Compiled {
        Compiled::from(self)
    }
}

/// Parse single branch, recursively
fn parse_branch(text: &str, recursion: usize) -> (Vec<Parsed>, usize) {
    // Check recursion limit
    assert!(recursion < 100, "Recursion limit [debug]");

    // List building
    let mut list: Vec<Parsed> = vec![];
    let mut item_building = String::new();

    // Loop over characters
    // Enumeration is used for returning `i` variable inside loop (on ']' match arm)
    let mut chars = text.chars().enumerate();
    while let Some((i, ch)) = chars.next() {
        match ch {
            // Skip line breaks, and spaces between items
            '\n' => (),
            ' ' if item_building.is_empty() => (),

            // Space separates items
            ' ' => {
                // Add current item to list, reset item
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
                let (item_branch, increase_index) = parse_branch(&rest, recursion + 1);

                // Add new branch to list
                list.push(List(item_branch));

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

    // Return final list, and dummy index
    // Index would be better as an `Option<usize>`
    (list, 0)
}
