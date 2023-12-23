// 🤖 This file is generated!

use crate::*;
/// The `<rp>` element's children.
#[derive(Clone)]
pub enum RubyParenthesisChild {
    Text(CowStr),
}
impl From<&'static str> for RubyParenthesisChild {
    fn from(s: &'static str) -> Self {
        RubyParenthesisChild::Text(s.into())
    }
}
impl From<String> for RubyParenthesisChild {
    fn from(s: String) -> Self {
        RubyParenthesisChild::Text(s.into())
    }
}
impl From<CowStr> for RubyParenthesisChild {
    fn from(s: CowStr) -> Self {
        RubyParenthesisChild::Text(s)
    }
}
impl std::fmt::Debug for RubyParenthesisChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RubyParenthesisChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for RubyParenthesisChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RubyParenthesisChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
