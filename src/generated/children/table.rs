// 🤖 This file is generated!

use crate::*;
/// The `<table>` element's children.
#[derive(Clone)]
pub enum TableChild {
    Caption(Caption),
    Script(Script),
    TableBody(TableBody),
    TableColumnGroup(TableColumnGroup),
    TableFoot(TableFoot),
    TableHead(TableHead),
    TableRow(TableRow),
    Template(Template),
}
impl From<Caption> for TableChild {
    fn from(child: Caption) -> Self {
        TableChild::Caption(child)
    }
}
impl From<builders::CaptionBuilder> for TableChild {
    fn from(builder: builders::CaptionBuilder) -> Self {
        TableChild::Caption(builder.build())
    }
}
impl From<Script> for TableChild {
    fn from(child: Script) -> Self {
        TableChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for TableChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        TableChild::Script(builder.build())
    }
}
impl From<TableBody> for TableChild {
    fn from(child: TableBody) -> Self {
        TableChild::TableBody(child)
    }
}
impl From<builders::TableBodyBuilder> for TableChild {
    fn from(builder: builders::TableBodyBuilder) -> Self {
        TableChild::TableBody(builder.build())
    }
}
impl From<TableColumnGroup> for TableChild {
    fn from(child: TableColumnGroup) -> Self {
        TableChild::TableColumnGroup(child)
    }
}
impl From<builders::TableColumnGroupBuilder> for TableChild {
    fn from(builder: builders::TableColumnGroupBuilder) -> Self {
        TableChild::TableColumnGroup(builder.build())
    }
}
impl From<TableFoot> for TableChild {
    fn from(child: TableFoot) -> Self {
        TableChild::TableFoot(child)
    }
}
impl From<builders::TableFootBuilder> for TableChild {
    fn from(builder: builders::TableFootBuilder) -> Self {
        TableChild::TableFoot(builder.build())
    }
}
impl From<TableHead> for TableChild {
    fn from(child: TableHead) -> Self {
        TableChild::TableHead(child)
    }
}
impl From<builders::TableHeadBuilder> for TableChild {
    fn from(builder: builders::TableHeadBuilder) -> Self {
        TableChild::TableHead(builder.build())
    }
}
impl From<TableRow> for TableChild {
    fn from(child: TableRow) -> Self {
        TableChild::TableRow(child)
    }
}
impl From<builders::TableRowBuilder> for TableChild {
    fn from(builder: builders::TableRowBuilder) -> Self {
        TableChild::TableRow(builder.build())
    }
}
impl From<Template> for TableChild {
    fn from(child: Template) -> Self {
        TableChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for TableChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        TableChild::Template(builder.build())
    }
}
impl std::fmt::Debug for TableChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableChild::Caption(child) => std::fmt::Debug::fmt(child, f),
            TableChild::Script(child) => std::fmt::Debug::fmt(child, f),
            TableChild::TableBody(child) => std::fmt::Debug::fmt(child, f),
            TableChild::TableColumnGroup(child) => std::fmt::Debug::fmt(child, f),
            TableChild::TableFoot(child) => std::fmt::Debug::fmt(child, f),
            TableChild::TableHead(child) => std::fmt::Debug::fmt(child, f),
            TableChild::TableRow(child) => std::fmt::Debug::fmt(child, f),
            TableChild::Template(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for TableChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableChild::Caption(child) => std::fmt::Display::fmt(child, f),
            TableChild::Script(child) => std::fmt::Display::fmt(child, f),
            TableChild::TableBody(child) => std::fmt::Display::fmt(child, f),
            TableChild::TableColumnGroup(child) => std::fmt::Display::fmt(child, f),
            TableChild::TableFoot(child) => std::fmt::Display::fmt(child, f),
            TableChild::TableHead(child) => std::fmt::Display::fmt(child, f),
            TableChild::TableRow(child) => std::fmt::Display::fmt(child, f),
            TableChild::Template(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
