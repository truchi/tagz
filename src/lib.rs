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

impl From<CowStr> for BoolOrF64OrString {
    fn from(s: CowStr) -> Self {
        Self::String(s)
    }
}

#[derive(Clone, Debug)]
pub enum AttributeType {
    String(CowStr),
    Bool(bool),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    F32(f32),
    I64(i64),
    U64(u64),
    F64(f64),
}

impl From<&'static str> for AttributeType {
    fn from(s: &'static str) -> Self {
        Self::String(s.into())
    }
}

impl From<String> for AttributeType {
    fn from(s: String) -> Self {
        Self::String(s.into())
    }
}

impl From<CowStr> for AttributeType {
    fn from(s: CowStr) -> Self {
        Self::String(s)
    }
}

impl From<bool> for AttributeType {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<i16> for AttributeType {
    fn from(i: i16) -> Self {
        Self::I16(i)
    }
}

impl From<u16> for AttributeType {
    fn from(u: u16) -> Self {
        Self::U16(u)
    }
}

impl From<i32> for AttributeType {
    fn from(i: i32) -> Self {
        Self::I32(i)
    }
}

impl From<u32> for AttributeType {
    fn from(u: u32) -> Self {
        Self::U32(u)
    }
}

impl From<f32> for AttributeType {
    fn from(f: f32) -> Self {
        Self::F32(f)
    }
}

impl From<i64> for AttributeType {
    fn from(i: i64) -> Self {
        Self::I64(i)
    }
}

impl From<u64> for AttributeType {
    fn from(u: u64) -> Self {
        Self::U64(u)
    }
}

impl From<f64> for AttributeType {
    fn from(f: f64) -> Self {
        Self::F64(f)
    }
}
