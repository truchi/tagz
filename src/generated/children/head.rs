// 🤖 This file is generated!

use crate::*;
/// The `<head>` element's children.
#[derive(Clone)]
pub enum HeadChild {
    Base(Base),
    Link(Link),
    Metadata(Metadata),
    NoScript(NoScript),
    Script(Script),
    Style(Style),
    Template(Template),
    Title(Title),
}
impl From<Base> for HeadChild {
    fn from(child: Base) -> Self {
        HeadChild::Base(child)
    }
}
impl From<builders::BaseBuilder> for HeadChild {
    fn from(builder: builders::BaseBuilder) -> Self {
        HeadChild::Base(builder.build())
    }
}
impl From<Link> for HeadChild {
    fn from(child: Link) -> Self {
        HeadChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for HeadChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        HeadChild::Link(builder.build())
    }
}
impl From<Metadata> for HeadChild {
    fn from(child: Metadata) -> Self {
        HeadChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for HeadChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        HeadChild::Metadata(builder.build())
    }
}
impl From<NoScript> for HeadChild {
    fn from(child: NoScript) -> Self {
        HeadChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for HeadChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        HeadChild::NoScript(builder.build())
    }
}
impl From<Script> for HeadChild {
    fn from(child: Script) -> Self {
        HeadChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for HeadChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        HeadChild::Script(builder.build())
    }
}
impl From<Style> for HeadChild {
    fn from(child: Style) -> Self {
        HeadChild::Style(child)
    }
}
impl From<builders::StyleBuilder> for HeadChild {
    fn from(builder: builders::StyleBuilder) -> Self {
        HeadChild::Style(builder.build())
    }
}
impl From<Template> for HeadChild {
    fn from(child: Template) -> Self {
        HeadChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for HeadChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        HeadChild::Template(builder.build())
    }
}
impl From<Title> for HeadChild {
    fn from(child: Title) -> Self {
        HeadChild::Title(child)
    }
}
impl From<builders::TitleBuilder> for HeadChild {
    fn from(builder: builders::TitleBuilder) -> Self {
        HeadChild::Title(builder.build())
    }
}
impl std::fmt::Debug for HeadChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeadChild::Base(child) => std::fmt::Debug::fmt(child, f),
            HeadChild::Link(child) => std::fmt::Debug::fmt(child, f),
            HeadChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            HeadChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            HeadChild::Script(child) => std::fmt::Debug::fmt(child, f),
            HeadChild::Style(child) => std::fmt::Debug::fmt(child, f),
            HeadChild::Template(child) => std::fmt::Debug::fmt(child, f),
            HeadChild::Title(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for HeadChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeadChild::Base(child) => std::fmt::Display::fmt(child, f),
            HeadChild::Link(child) => std::fmt::Display::fmt(child, f),
            HeadChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            HeadChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            HeadChild::Script(child) => std::fmt::Display::fmt(child, f),
            HeadChild::Style(child) => std::fmt::Display::fmt(child, f),
            HeadChild::Template(child) => std::fmt::Display::fmt(child, f),
            HeadChild::Title(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
