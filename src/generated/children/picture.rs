// 🤖 This file is generated!

use crate::*;
/// The `<picture>` element's children.
#[derive(Clone)]
pub enum PictureChild {
    Image(Image),
    Script(Script),
    Source(Source),
    Template(Template),
}
impl From<Image> for PictureChild {
    fn from(child: Image) -> Self {
        PictureChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for PictureChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        PictureChild::Image(builder.build())
    }
}
impl From<Script> for PictureChild {
    fn from(child: Script) -> Self {
        PictureChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for PictureChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        PictureChild::Script(builder.build())
    }
}
impl From<Source> for PictureChild {
    fn from(child: Source) -> Self {
        PictureChild::Source(child)
    }
}
impl From<builders::SourceBuilder> for PictureChild {
    fn from(builder: builders::SourceBuilder) -> Self {
        PictureChild::Source(builder.build())
    }
}
impl From<Template> for PictureChild {
    fn from(child: Template) -> Self {
        PictureChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for PictureChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        PictureChild::Template(builder.build())
    }
}
impl std::fmt::Debug for PictureChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PictureChild::Image(child) => std::fmt::Debug::fmt(child, f),
            PictureChild::Script(child) => std::fmt::Debug::fmt(child, f),
            PictureChild::Source(child) => std::fmt::Debug::fmt(child, f),
            PictureChild::Template(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for PictureChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PictureChild::Image(child) => std::fmt::Display::fmt(child, f),
            PictureChild::Script(child) => std::fmt::Display::fmt(child, f),
            PictureChild::Source(child) => std::fmt::Display::fmt(child, f),
            PictureChild::Template(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
