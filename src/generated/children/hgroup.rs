// 🤖 This file is generated!

use crate::*;
/// The `<hgroup>` element's children.
#[derive(Clone)]
pub enum HeadingGroupChild {
    Heading1(Heading1),
    Heading2(Heading2),
    Heading3(Heading3),
    Heading4(Heading4),
    Heading5(Heading5),
    Heading6(Heading6),
    Paragraph(Paragraph),
    Script(Script),
    Template(Template),
}
impl From<Heading1> for HeadingGroupChild {
    fn from(child: Heading1) -> Self {
        HeadingGroupChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for HeadingGroupChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        HeadingGroupChild::Heading1(builder.build())
    }
}
impl From<Heading2> for HeadingGroupChild {
    fn from(child: Heading2) -> Self {
        HeadingGroupChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for HeadingGroupChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        HeadingGroupChild::Heading2(builder.build())
    }
}
impl From<Heading3> for HeadingGroupChild {
    fn from(child: Heading3) -> Self {
        HeadingGroupChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for HeadingGroupChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        HeadingGroupChild::Heading3(builder.build())
    }
}
impl From<Heading4> for HeadingGroupChild {
    fn from(child: Heading4) -> Self {
        HeadingGroupChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for HeadingGroupChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        HeadingGroupChild::Heading4(builder.build())
    }
}
impl From<Heading5> for HeadingGroupChild {
    fn from(child: Heading5) -> Self {
        HeadingGroupChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for HeadingGroupChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        HeadingGroupChild::Heading5(builder.build())
    }
}
impl From<Heading6> for HeadingGroupChild {
    fn from(child: Heading6) -> Self {
        HeadingGroupChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for HeadingGroupChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        HeadingGroupChild::Heading6(builder.build())
    }
}
impl From<Paragraph> for HeadingGroupChild {
    fn from(child: Paragraph) -> Self {
        HeadingGroupChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for HeadingGroupChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        HeadingGroupChild::Paragraph(builder.build())
    }
}
impl From<Script> for HeadingGroupChild {
    fn from(child: Script) -> Self {
        HeadingGroupChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for HeadingGroupChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        HeadingGroupChild::Script(builder.build())
    }
}
impl From<Template> for HeadingGroupChild {
    fn from(child: Template) -> Self {
        HeadingGroupChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for HeadingGroupChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        HeadingGroupChild::Template(builder.build())
    }
}
impl std::fmt::Debug for HeadingGroupChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeadingGroupChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            HeadingGroupChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            HeadingGroupChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            HeadingGroupChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            HeadingGroupChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            HeadingGroupChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            HeadingGroupChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            HeadingGroupChild::Script(child) => std::fmt::Debug::fmt(child, f),
            HeadingGroupChild::Template(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for HeadingGroupChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeadingGroupChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            HeadingGroupChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            HeadingGroupChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            HeadingGroupChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            HeadingGroupChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            HeadingGroupChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            HeadingGroupChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            HeadingGroupChild::Script(child) => std::fmt::Display::fmt(child, f),
            HeadingGroupChild::Template(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
