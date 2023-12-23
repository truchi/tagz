// 🤖 This file is generated!

use crate::*;
/// The `<optgroup>` element's children.
#[derive(Clone)]
pub enum OptionGroupChild {
    Option(Option),
    Script(Script),
    Template(Template),
}
impl From<Option> for OptionGroupChild {
    fn from(child: Option) -> Self {
        OptionGroupChild::Option(child)
    }
}
impl From<builders::OptionBuilder> for OptionGroupChild {
    fn from(builder: builders::OptionBuilder) -> Self {
        OptionGroupChild::Option(builder.build())
    }
}
impl From<Script> for OptionGroupChild {
    fn from(child: Script) -> Self {
        OptionGroupChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for OptionGroupChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        OptionGroupChild::Script(builder.build())
    }
}
impl From<Template> for OptionGroupChild {
    fn from(child: Template) -> Self {
        OptionGroupChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for OptionGroupChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        OptionGroupChild::Template(builder.build())
    }
}
impl std::fmt::Debug for OptionGroupChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionGroupChild::Option(child) => std::fmt::Debug::fmt(child, f),
            OptionGroupChild::Script(child) => std::fmt::Debug::fmt(child, f),
            OptionGroupChild::Template(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for OptionGroupChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionGroupChild::Option(child) => std::fmt::Display::fmt(child, f),
            OptionGroupChild::Script(child) => std::fmt::Display::fmt(child, f),
            OptionGroupChild::Template(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
