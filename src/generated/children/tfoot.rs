// 🤖 This file is generated!

use crate::*;
/// The `<tfoot>` element's children.
#[derive(Clone)]
pub enum TableFootChild {
    Script(Script),
    TableRow(TableRow),
    Template(Template),
}
impl From<Script> for TableFootChild {
    fn from(child: Script) -> Self {
        TableFootChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for TableFootChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        TableFootChild::Script(builder.build())
    }
}
impl From<TableRow> for TableFootChild {
    fn from(child: TableRow) -> Self {
        TableFootChild::TableRow(child)
    }
}
impl From<builders::TableRowBuilder> for TableFootChild {
    fn from(builder: builders::TableRowBuilder) -> Self {
        TableFootChild::TableRow(builder.build())
    }
}
impl From<Template> for TableFootChild {
    fn from(child: Template) -> Self {
        TableFootChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for TableFootChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        TableFootChild::Template(builder.build())
    }
}
impl std::fmt::Debug for TableFootChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableFootChild::Script(child) => std::fmt::Debug::fmt(child, f),
            TableFootChild::TableRow(child) => std::fmt::Debug::fmt(child, f),
            TableFootChild::Template(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for TableFootChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableFootChild::Script(child) => std::fmt::Display::fmt(child, f),
            TableFootChild::TableRow(child) => std::fmt::Display::fmt(child, f),
            TableFootChild::Template(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
