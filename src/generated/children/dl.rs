// 🤖 This file is generated!

use crate::*;
/// The `<dl>` element's children.
#[derive(Clone)]
pub enum DescriptionListChild {
    DescriptionDetails(DescriptionDetails),
    DescriptionTerm(DescriptionTerm),
    Division(Division),
    Script(Script),
    Template(Template),
}
impl From<DescriptionDetails> for DescriptionListChild {
    fn from(child: DescriptionDetails) -> Self {
        DescriptionListChild::DescriptionDetails(child)
    }
}
impl From<builders::DescriptionDetailsBuilder> for DescriptionListChild {
    fn from(builder: builders::DescriptionDetailsBuilder) -> Self {
        DescriptionListChild::DescriptionDetails(builder.build())
    }
}
impl From<DescriptionTerm> for DescriptionListChild {
    fn from(child: DescriptionTerm) -> Self {
        DescriptionListChild::DescriptionTerm(child)
    }
}
impl From<builders::DescriptionTermBuilder> for DescriptionListChild {
    fn from(builder: builders::DescriptionTermBuilder) -> Self {
        DescriptionListChild::DescriptionTerm(builder.build())
    }
}
impl From<Division> for DescriptionListChild {
    fn from(child: Division) -> Self {
        DescriptionListChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for DescriptionListChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        DescriptionListChild::Division(builder.build())
    }
}
impl From<Script> for DescriptionListChild {
    fn from(child: Script) -> Self {
        DescriptionListChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for DescriptionListChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        DescriptionListChild::Script(builder.build())
    }
}
impl From<Template> for DescriptionListChild {
    fn from(child: Template) -> Self {
        DescriptionListChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for DescriptionListChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        DescriptionListChild::Template(builder.build())
    }
}
impl std::fmt::Debug for DescriptionListChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DescriptionListChild::DescriptionDetails(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DescriptionListChild::DescriptionTerm(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DescriptionListChild::Division(child) => std::fmt::Debug::fmt(child, f),
            DescriptionListChild::Script(child) => std::fmt::Debug::fmt(child, f),
            DescriptionListChild::Template(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for DescriptionListChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DescriptionListChild::DescriptionDetails(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionListChild::DescriptionTerm(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionListChild::Division(child) => std::fmt::Display::fmt(child, f),
            DescriptionListChild::Script(child) => std::fmt::Display::fmt(child, f),
            DescriptionListChild::Template(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
