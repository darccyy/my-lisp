use crate::compile::{
    Compiled,
    Value::{self, *},
};

impl Compiled {
    /// Run compiled script, returning last value
    pub fn run(self) -> Value {
        Null
    }
}
