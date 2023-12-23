// 🤖 This file is generated!

use crate::*;
/// The `<ol>` element's children.
#[derive(Clone)]
pub enum OrderedListChild {
    ListItem(ListItem),
    Script(Script),
    Template(Template),
}
impl From<ListItem> for OrderedListChild {
    fn from(child: ListItem) -> Self {
        OrderedListChild::ListItem(child)
    }
}
impl From<builders::ListItemBuilder> for OrderedListChild {
    fn from(builder: builders::ListItemBuilder) -> Self {
        OrderedListChild::ListItem(builder.build())
    }
}
impl From<Script> for OrderedListChild {
    fn from(child: Script) -> Self {
        OrderedListChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for OrderedListChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        OrderedListChild::Script(builder.build())
    }
}
impl From<Template> for OrderedListChild {
    fn from(child: Template) -> Self {
        OrderedListChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for OrderedListChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        OrderedListChild::Template(builder.build())
    }
}
impl std::fmt::Debug for OrderedListChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderedListChild::ListItem(child) => std::fmt::Debug::fmt(child, f),
            OrderedListChild::Script(child) => std::fmt::Debug::fmt(child, f),
            OrderedListChild::Template(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for OrderedListChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderedListChild::ListItem(child) => std::fmt::Display::fmt(child, f),
            OrderedListChild::Script(child) => std::fmt::Display::fmt(child, f),
            OrderedListChild::Template(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
