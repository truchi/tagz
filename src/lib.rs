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

impl Default for BoolOrF64OrString {
    fn default() -> Self {
        Self::Bool(false)
    }
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

impl Default for AttributeType {
    fn default() -> Self {
        Self::Bool(false)
    }
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

/// https://html.spec.whatwg.org/multipage/dom.html#global-attributes
/// > The id attribute value [...] must contain at least one character.
/// > The value must not contain any ASCII whitespace.
#[cfg(debug_assertions)]
fn check_id(id: &str) -> bool {
    !id.is_empty() && !id.chars().any(|c| c.is_ascii_whitespace())
}

///
#[cfg(debug_assertions)]
fn check_class(value: &str) -> bool {
    check_attribute_value(&AttributeType::String(value.to_string().into()))
}

/// https://html.spec.whatwg.org/multipage/syntax.html#attributes-2
/// > Attribute names must consist of one or more characters other than controls,
/// > U+0020 SPACE, U+0022 ("), U+0027 ('), U+003E (>), U+002F (/), U+003D (=), and noncharacters.
///
/// https://html.spec.whatwg.org/multipage/dom.html#embedding-custom-non-visible-data-with-the-data-*-attributes
/// > [...] has at least one character after the hyphen, is XML-compatible,
/// > and contains no ASCII upper alphas.
/// (NOTE: we relax the uppercase constraint)
#[cfg(debug_assertions)]
fn check_attribute_name(value: &str) -> bool {
    fn contains_noncharacter(s: &str) -> bool {
        s.chars().any(|c| match c as u32 {
            #[rustfmt::skip]
            0xFDD0..=0xFDEF
            | 0xFFFE  | 0xFFFF  | 0x1FFFE | 0x1FFFF | 0x2FFFE | 0x2FFFF | 0x3FFFE | 0x3FFFF
            | 0x4FFFE | 0x4FFFF | 0x5FFFE | 0x5FFFF | 0x6FFFE | 0x6FFFF | 0x7FFFE | 0x7FFFF
            | 0x8FFFE | 0x8FFFF | 0x9FFFE | 0x9FFFF | 0xAFFFE | 0xAFFFF | 0xBFFFE | 0xBFFFF
            | 0xCFFFE | 0xCFFFF | 0xDFFFE | 0xDFFFF | 0xEFFFE | 0xEFFFF | 0xFFFFE | 0xFFFFF
            | 0x10FFFE | 0x10FFFF => true,
            _ => false,
        })
    }

    fn contains_control(s: &str) -> bool {
        s.chars()
            .any(|c| matches!(c as u32, 0x0000..=0x001F | 0x007F..=0x009F))
    }

    !value.is_empty()
        && !contains_control(value)
        && !contains_noncharacter(value)
        && !value.chars().any(|c| {
            c.is_ascii_control()
                || c.is_ascii_whitespace()
                || c == '"'
                || c == '\''
                || c == '>'
                || c == '/'
                || c == '='
        })
}

/// https://html.spec.whatwg.org/multipage/syntax.html#attributes-2
/// > Single-quoted attribute value syntax
/// > [...] must not contain any literal U+0027 APOSTROPHE characters (') [...]
#[cfg(debug_assertions)]
fn check_attribute_value(value: &AttributeType) -> bool {
    match value {
        AttributeType::String(value) => !value.contains('\''),
        _ => true,
    }
}
