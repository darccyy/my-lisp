use crate::{
    compile::Method::{self, *},
    Value::{self, *},
};

impl Method {
    /// Run compiled script, returning last value
    pub fn run(&self) -> Value {
        match self {
            // Run many statements-like methods, and return last method result (like Rust)
            Many(list) => {
                // Create peekable iterator
                let mut it = list.iter().peekable();

                // Loop over list
                // Must be `while let` to use peekable
                while let Some(method) = it.next() {
                    // Run method
                    let result = method.run();

                    // Return result, if is last item
                    if it.peek().is_none() {
                        return result;
                    }
                }

                // Default return value
                Null
            }

            // Literal
            Literal(value) => value.clone(),

            // Print value, return Null
            Print(value) => {
                println!("{:?}", value);
                Null
            }

            // Arithmetic operations
            Add(rhs, lhs) => rhs.run() + lhs.run(),
            Mul(rhs, lhs) => rhs.run() * lhs.run(),
        }
    }
}
