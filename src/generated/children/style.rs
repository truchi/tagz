// 🤖 This file is generated!

use crate::*;
/// The `<style>` element's children.
#[derive(Clone)]
pub enum StyleChild {
    Text(CowStr),
}
impl From<&'static str> for StyleChild {
    fn from(s: &'static str) -> Self {
        StyleChild::Text(s.into())
    }
}
impl From<String> for StyleChild {
    fn from(s: String) -> Self {
        StyleChild::Text(s.into())
    }
}
impl From<CowStr> for StyleChild {
    fn from(s: CowStr) -> Self {
        StyleChild::Text(s)
    }
}
impl std::fmt::Debug for StyleChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StyleChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for StyleChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StyleChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
