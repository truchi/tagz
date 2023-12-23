// 🤖 This file is generated!

use crate::*;
/// The `<menu>` element's children.
#[derive(Clone)]
pub enum MenuChild {
    ListItem(ListItem),
    Script(Script),
    Template(Template),
}
impl From<ListItem> for MenuChild {
    fn from(child: ListItem) -> Self {
        MenuChild::ListItem(child)
    }
}
impl From<builders::ListItemBuilder> for MenuChild {
    fn from(builder: builders::ListItemBuilder) -> Self {
        MenuChild::ListItem(builder.build())
    }
}
impl From<Script> for MenuChild {
    fn from(child: Script) -> Self {
        MenuChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for MenuChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        MenuChild::Script(builder.build())
    }
}
impl From<Template> for MenuChild {
    fn from(child: Template) -> Self {
        MenuChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for MenuChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        MenuChild::Template(builder.build())
    }
}
impl std::fmt::Debug for MenuChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuChild::ListItem(child) => std::fmt::Debug::fmt(child, f),
            MenuChild::Script(child) => std::fmt::Debug::fmt(child, f),
            MenuChild::Template(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for MenuChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuChild::ListItem(child) => std::fmt::Display::fmt(child, f),
            MenuChild::Script(child) => std::fmt::Display::fmt(child, f),
            MenuChild::Template(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
