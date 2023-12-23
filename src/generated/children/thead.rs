// 🤖 This file is generated!

use crate::*;
/// The `<thead>` element's children.
#[derive(Clone)]
pub enum TableHeadChild {
    Script(Script),
    TableRow(TableRow),
    Template(Template),
}
impl From<Script> for TableHeadChild {
    fn from(child: Script) -> Self {
        TableHeadChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for TableHeadChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        TableHeadChild::Script(builder.build())
    }
}
impl From<TableRow> for TableHeadChild {
    fn from(child: TableRow) -> Self {
        TableHeadChild::TableRow(child)
    }
}
impl From<builders::TableRowBuilder> for TableHeadChild {
    fn from(builder: builders::TableRowBuilder) -> Self {
        TableHeadChild::TableRow(builder.build())
    }
}
impl From<Template> for TableHeadChild {
    fn from(child: Template) -> Self {
        TableHeadChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for TableHeadChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        TableHeadChild::Template(builder.build())
    }
}
impl std::fmt::Debug for TableHeadChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableHeadChild::Script(child) => std::fmt::Debug::fmt(child, f),
            TableHeadChild::TableRow(child) => std::fmt::Debug::fmt(child, f),
            TableHeadChild::Template(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for TableHeadChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableHeadChild::Script(child) => std::fmt::Display::fmt(child, f),
            TableHeadChild::TableRow(child) => std::fmt::Display::fmt(child, f),
            TableHeadChild::Template(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
