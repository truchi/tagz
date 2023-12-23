// 🤖 This file is generated!

use crate::*;
/// The `<select>` element's children.
#[derive(Clone)]
pub enum SelectChild {
    HorizontalRule(HorizontalRule),
    Option(Option),
    OptionGroup(OptionGroup),
    Script(Script),
    Template(Template),
}
impl From<HorizontalRule> for SelectChild {
    fn from(child: HorizontalRule) -> Self {
        SelectChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for SelectChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        SelectChild::HorizontalRule(builder.build())
    }
}
impl From<Option> for SelectChild {
    fn from(child: Option) -> Self {
        SelectChild::Option(child)
    }
}
impl From<builders::OptionBuilder> for SelectChild {
    fn from(builder: builders::OptionBuilder) -> Self {
        SelectChild::Option(builder.build())
    }
}
impl From<OptionGroup> for SelectChild {
    fn from(child: OptionGroup) -> Self {
        SelectChild::OptionGroup(child)
    }
}
impl From<builders::OptionGroupBuilder> for SelectChild {
    fn from(builder: builders::OptionGroupBuilder) -> Self {
        SelectChild::OptionGroup(builder.build())
    }
}
impl From<Script> for SelectChild {
    fn from(child: Script) -> Self {
        SelectChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for SelectChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        SelectChild::Script(builder.build())
    }
}
impl From<Template> for SelectChild {
    fn from(child: Template) -> Self {
        SelectChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for SelectChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        SelectChild::Template(builder.build())
    }
}
impl std::fmt::Debug for SelectChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            SelectChild::Option(child) => std::fmt::Debug::fmt(child, f),
            SelectChild::OptionGroup(child) => std::fmt::Debug::fmt(child, f),
            SelectChild::Script(child) => std::fmt::Debug::fmt(child, f),
            SelectChild::Template(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for SelectChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            SelectChild::Option(child) => std::fmt::Display::fmt(child, f),
            SelectChild::OptionGroup(child) => std::fmt::Display::fmt(child, f),
            SelectChild::Script(child) => std::fmt::Display::fmt(child, f),
            SelectChild::Template(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
