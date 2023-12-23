// 🤖 This file is generated!

use crate::*;
/// The `<colgroup>` element's children.
#[derive(Clone)]
pub enum TableColumnGroupChild {
    TableColumn(TableColumn),
    Template(Template),
}
impl From<TableColumn> for TableColumnGroupChild {
    fn from(child: TableColumn) -> Self {
        TableColumnGroupChild::TableColumn(child)
    }
}
impl From<builders::TableColumnBuilder> for TableColumnGroupChild {
    fn from(builder: builders::TableColumnBuilder) -> Self {
        TableColumnGroupChild::TableColumn(builder.build())
    }
}
impl From<Template> for TableColumnGroupChild {
    fn from(child: Template) -> Self {
        TableColumnGroupChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for TableColumnGroupChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        TableColumnGroupChild::Template(builder.build())
    }
}
impl std::fmt::Debug for TableColumnGroupChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableColumnGroupChild::TableColumn(child) => std::fmt::Debug::fmt(child, f),
            TableColumnGroupChild::Template(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for TableColumnGroupChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableColumnGroupChild::TableColumn(child) => std::fmt::Display::fmt(child, f),
            TableColumnGroupChild::Template(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
