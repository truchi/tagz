#![recursion_limit = "1024"]

mod generated;
pub use generated::*;

#[derive(Clone, Debug)]
pub enum BoolOrF64OrString {
    Bool(bool),
    F64(f64),
    String(String),
}
