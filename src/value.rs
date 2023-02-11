use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

use serde::Serialize;
use serde_variant::to_variant_name;

use Value::*;

/// Weakly-typed variable value
///
/// Similar to JSON
#[derive(Clone, Serialize)]
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
            Null => write!(f, "\x1b[38;2;247;140;108mɴᴜʟʟ\x1b[0m"),
            Int(int) => write!(f, "\x1b[33m{}\x1b[0m", int),
            Str(string) => write!(f, "\x1b[32m'{}'\x1b[0m", string),
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

/// Panic with a descriptive message, for an invalid arithmetic operation
fn panic_invalid_operation(op: &'static str, a: Value, b: Value) -> ! {
    panic!(
        "Cannot {} {:?} to {:?}",
        op,
        to_variant_name(&a).unwrap(),
        to_variant_name(&b).unwrap()
    );
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Int(a), Int(b)) => Int(a + b),
            (Str(a), Str(b)) => Str(a + &b),

            (a, b) => panic_invalid_operation("add", a, b),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Int(a), Int(b)) => Int(a * b),
            (Str(a), Int(b)) => Str(a.repeat(b as usize)),

            (a, b) => panic_invalid_operation("multiply", a, b),
        }
    }
}
