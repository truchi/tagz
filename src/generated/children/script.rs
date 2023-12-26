// 🤖 This file is generated!

use crate::*;
/// The `<script>` element's children.
#[derive(Clone)]
pub enum ScriptChild {
    Text(CowStr),
}
impl From<&'static str> for ScriptChild {
    fn from(s: &'static str) -> Self {
        ScriptChild::Text(s.into())
    }
}
impl From<String> for ScriptChild {
    fn from(s: String) -> Self {
        ScriptChild::Text(s.into())
    }
}
impl From<CowStr> for ScriptChild {
    fn from(s: CowStr) -> Self {
        ScriptChild::Text(s)
    }
}
impl std::fmt::Debug for ScriptChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for ScriptChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
