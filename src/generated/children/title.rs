// 🤖 This file is generated!

use crate::*;
/// The `<title>` element's children.
#[derive(Clone)]
pub enum TitleChild {
    Text(CowStr),
}
impl From<&'static str> for TitleChild {
    fn from(s: &'static str) -> Self {
        TitleChild::Text(s.into())
    }
}
impl From<String> for TitleChild {
    fn from(s: String) -> Self {
        TitleChild::Text(s.into())
    }
}
impl From<CowStr> for TitleChild {
    fn from(s: CowStr) -> Self {
        TitleChild::Text(s)
    }
}
impl std::fmt::Debug for TitleChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TitleChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for TitleChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TitleChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
