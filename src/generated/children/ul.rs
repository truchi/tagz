// 🤖 This file is generated!

use crate::*;
/// The `<ul>` element's children.
#[derive(Clone)]
pub enum UnorderedListChild {
    ListItem(ListItem),
    Script(Script),
    Template(Template),
}
impl From<ListItem> for UnorderedListChild {
    fn from(child: ListItem) -> Self {
        UnorderedListChild::ListItem(child)
    }
}
impl From<builders::ListItemBuilder> for UnorderedListChild {
    fn from(builder: builders::ListItemBuilder) -> Self {
        UnorderedListChild::ListItem(builder.build())
    }
}
impl From<Script> for UnorderedListChild {
    fn from(child: Script) -> Self {
        UnorderedListChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for UnorderedListChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        UnorderedListChild::Script(builder.build())
    }
}
impl From<Template> for UnorderedListChild {
    fn from(child: Template) -> Self {
        UnorderedListChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for UnorderedListChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        UnorderedListChild::Template(builder.build())
    }
}
impl std::fmt::Debug for UnorderedListChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnorderedListChild::ListItem(child) => std::fmt::Debug::fmt(child, f),
            UnorderedListChild::Script(child) => std::fmt::Debug::fmt(child, f),
            UnorderedListChild::Template(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for UnorderedListChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnorderedListChild::ListItem(child) => std::fmt::Display::fmt(child, f),
            UnorderedListChild::Script(child) => std::fmt::Display::fmt(child, f),
            UnorderedListChild::Template(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
