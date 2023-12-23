// 🤖 This file is generated!

use crate::*;
/// The `<html>` element's children.
#[derive(Clone)]
pub enum HtmlChild {
    Body(Body),
    Head(Head),
}
impl From<Body> for HtmlChild {
    fn from(child: Body) -> Self {
        HtmlChild::Body(child)
    }
}
impl From<builders::BodyBuilder> for HtmlChild {
    fn from(builder: builders::BodyBuilder) -> Self {
        HtmlChild::Body(builder.build())
    }
}
impl From<Head> for HtmlChild {
    fn from(child: Head) -> Self {
        HtmlChild::Head(child)
    }
}
impl From<builders::HeadBuilder> for HtmlChild {
    fn from(builder: builders::HeadBuilder) -> Self {
        HtmlChild::Head(builder.build())
    }
}
impl std::fmt::Debug for HtmlChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HtmlChild::Body(child) => std::fmt::Debug::fmt(child, f),
            HtmlChild::Head(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for HtmlChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HtmlChild::Body(child) => std::fmt::Display::fmt(child, f),
            HtmlChild::Head(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
