// 🤖 This file is generated!

use crate::*;
/// The `<tr>` element's children.
#[derive(Clone)]
pub enum TableRowChild {
    Script(Script),
    TableCell(TableCell),
    TableHeader(TableHeader),
    Template(Template),
}
impl From<Script> for TableRowChild {
    fn from(child: Script) -> Self {
        TableRowChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for TableRowChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        TableRowChild::Script(builder.build())
    }
}
impl From<TableCell> for TableRowChild {
    fn from(child: TableCell) -> Self {
        TableRowChild::TableCell(child)
    }
}
impl From<builders::TableCellBuilder> for TableRowChild {
    fn from(builder: builders::TableCellBuilder) -> Self {
        TableRowChild::TableCell(builder.build())
    }
}
impl From<TableHeader> for TableRowChild {
    fn from(child: TableHeader) -> Self {
        TableRowChild::TableHeader(child)
    }
}
impl From<builders::TableHeaderBuilder> for TableRowChild {
    fn from(builder: builders::TableHeaderBuilder) -> Self {
        TableRowChild::TableHeader(builder.build())
    }
}
impl From<Template> for TableRowChild {
    fn from(child: Template) -> Self {
        TableRowChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for TableRowChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        TableRowChild::Template(builder.build())
    }
}
impl std::fmt::Debug for TableRowChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableRowChild::Script(child) => std::fmt::Debug::fmt(child, f),
            TableRowChild::TableCell(child) => std::fmt::Debug::fmt(child, f),
            TableRowChild::TableHeader(child) => std::fmt::Debug::fmt(child, f),
            TableRowChild::Template(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for TableRowChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableRowChild::Script(child) => std::fmt::Display::fmt(child, f),
            TableRowChild::TableCell(child) => std::fmt::Display::fmt(child, f),
            TableRowChild::TableHeader(child) => std::fmt::Display::fmt(child, f),
            TableRowChild::Template(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
