// 🤖 This file is generated!

use crate::*;
/// The `<tbody>` element's children.
#[derive(Clone)]
pub enum TableBodyChild {
    Script(Script),
    TableRow(TableRow),
    Template(Template),
}
impl From<Script> for TableBodyChild {
    fn from(child: Script) -> Self {
        TableBodyChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for TableBodyChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        TableBodyChild::Script(builder.build())
    }
}
impl From<TableRow> for TableBodyChild {
    fn from(child: TableRow) -> Self {
        TableBodyChild::TableRow(child)
    }
}
impl From<builders::TableRowBuilder> for TableBodyChild {
    fn from(builder: builders::TableRowBuilder) -> Self {
        TableBodyChild::TableRow(builder.build())
    }
}
impl From<Template> for TableBodyChild {
    fn from(child: Template) -> Self {
        TableBodyChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for TableBodyChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        TableBodyChild::Template(builder.build())
    }
}
impl std::fmt::Debug for TableBodyChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableBodyChild::Script(child) => std::fmt::Debug::fmt(child, f),
            TableBodyChild::TableRow(child) => std::fmt::Debug::fmt(child, f),
            TableBodyChild::Template(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for TableBodyChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableBodyChild::Script(child) => std::fmt::Display::fmt(child, f),
            TableBodyChild::TableRow(child) => std::fmt::Display::fmt(child, f),
            TableBodyChild::Template(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
