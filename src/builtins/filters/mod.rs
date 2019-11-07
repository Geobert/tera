use std::collections::HashMap;

use crate::errors::Result;
use serde_json::value::Value;

pub mod array;
pub mod common;
pub mod number;
pub mod object;
pub mod string;
#[cfg(feature = "builtins")]
pub mod stringdeunicode;

/// The filter function type definition
pub trait Filter: Sync + Send {
    /// The filter function type definition
    fn filter(&self, value: &Value, args: &HashMap<String, Value>) -> Result<Value>;
}

impl<F> Filter for F
where
    F: Fn(&Value, &HashMap<String, Value>) -> Result<Value> + Sync + Send,
{
    fn filter(&self, value: &Value, args: &HashMap<String, Value>) -> Result<Value> {
        self(value, args)
    }
}
