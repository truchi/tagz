// 🤖 This file is generated!

use crate::*;
/// The `<option>` element's children.
#[derive(Clone)]
pub enum OptionChild {
    Text(CowStr),
}
impl From<&'static str> for OptionChild {
    fn from(s: &'static str) -> Self {
        OptionChild::Text(s.into())
    }
}
impl From<String> for OptionChild {
    fn from(s: String) -> Self {
        OptionChild::Text(s.into())
    }
}
impl From<CowStr> for OptionChild {
    fn from(s: CowStr) -> Self {
        OptionChild::Text(s)
    }
}
impl std::fmt::Debug for OptionChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for OptionChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
