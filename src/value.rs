use std::collections::LinkedList;

/// Represented value kinds in the database.
#[derive(Debug)]
pub enum Value {
    Str(String),
    LL(LinkedList<String>),
}
