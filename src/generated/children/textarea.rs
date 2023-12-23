// 🤖 This file is generated!

use crate::*;
/// The `<textarea>` element's children.
#[derive(Clone)]
pub enum TextAreaChild {
    Text(CowStr),
}
impl From<&'static str> for TextAreaChild {
    fn from(s: &'static str) -> Self {
        TextAreaChild::Text(s.into())
    }
}
impl From<String> for TextAreaChild {
    fn from(s: String) -> Self {
        TextAreaChild::Text(s.into())
    }
}
impl From<CowStr> for TextAreaChild {
    fn from(s: CowStr) -> Self {
        TextAreaChild::Text(s)
    }
}
impl std::fmt::Debug for TextAreaChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextAreaChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for TextAreaChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextAreaChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
