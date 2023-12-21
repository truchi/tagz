#![recursion_limit = "1024"]

mod generated;

pub use generated::*;

type StdOption<T> = std::option::Option<T>;
type HashMap<K, V> = std::collections::HashMap<K, V>;
type HashSet<T> = std::collections::HashSet<T>;
type CowStr = std::borrow::Cow<'static, str>;

#[derive(Clone, Debug)]
pub enum BoolOrF64OrString {
    Bool(bool),
    F64(f64),
    String(CowStr),
}

impl From<bool> for BoolOrF64OrString {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<f64> for BoolOrF64OrString {
    fn from(f: f64) -> Self {
        Self::F64(f)
    }
}

impl From<&'static str> for BoolOrF64OrString {
    fn from(s: &'static str) -> Self {
        Self::String(s.into())
    }
}

impl From<String> for BoolOrF64OrString {
    fn from(s: String) -> Self {
        Self::String(s.into())
    }
}
