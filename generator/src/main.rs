mod idl;
mod parser;
mod spec;

use serde::{Deserialize, Serialize};
use spec::Spec;
use std::str::FromStr;

/// [https://html.spec.whatwg.org/multipage/dom.html#content-models]
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Category {
    EmbeddedContent,
    FlowContent,
    HeadingContent,
    InteractiveContent,
    MetadataContent,
    PalpableContent,
    PhrasingContent,
    SectioningContent,

    FormAssociated, // All the following are form-associated categories:
    Listed,
    Submittable,
    Resettable,
    AutocapitalizeInheriting,
    Labelable, // (also includes non-form-associated elements)

    Document,
    Media,            // <audio> and <video>
    ScriptSupporting, // <script> and <template>
    Text,
    Transparent, // SHIT!

    Nothing,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .trim()
            .to_lowercase()
            .replace(' ', "")
            .replace('-', "")
            .as_str()
        {
            "embedded" => Ok(Self::EmbeddedContent),
            "flow" => Ok(Self::FlowContent),
            "heading" => Ok(Self::HeadingContent),
            "interactive" => Ok(Self::InteractiveContent),
            "metadata" => Ok(Self::MetadataContent),
            "palpable" => Ok(Self::PalpableContent),
            "phrasing" => Ok(Self::PhrasingContent),
            "sectioning" => Ok(Self::SectioningContent),
            "formassociated" => Ok(Self::FormAssociated),
            "listed" => Ok(Self::Listed),
            "submittable" => Ok(Self::Submittable),
            "resettable" => Ok(Self::Resettable),
            "autocapitalizeinheriting" => Ok(Self::AutocapitalizeInheriting),
            "labelable" => Ok(Self::Labelable),
            "document" => Ok(Self::Document),
            "media" => Ok(Self::Media),
            "scriptsupporting" => Ok(Self::ScriptSupporting),
            "text" => Ok(Self::Text),
            "transparent" => Ok(Self::Transparent),
            "nothing" => Ok(Self::Nothing),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub enum CategoryOrElement {
    Category(Category),
    Element(String),
}

impl From<&str> for CategoryOrElement {
    fn from(s: &str) -> Self {
        match Category::from_str(s) {
            Ok(category) => Self::Category(category),
            Err(_) => Self::Element(s.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum AttributeType {
    Bool,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    String,

    BoolOrF64OrString, // TODO collect all code.idl, parse and resolve
}

impl<'a> TryFrom<&weedle::interface::AttributeInterfaceMember<'a>> for AttributeType {
    type Error = ();

    fn try_from(
        attribute: &weedle::interface::AttributeInterfaceMember,
    ) -> Result<Self, Self::Error> {
        use weedle::types::*;

        match &attribute.type_.type_ {
            Type::Single(ty) => match ty {
                SingleType::NonAny(ty) => ty.try_into(),
                SingleType::Any(any) => {
                    tracing::error!(?any);
                    return Err(());
                }
            },
            Type::Union(union) => union
                .type_
                .body
                .list
                .iter()
                .filter_map(|ty| match ty {
                    UnionMemberType::Single(ty) => AttributeType::try_from(&ty.type_).ok(),
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
                .try_into(),
        }
    }
}

/// https://webidl.spec.whatwg.org/#idl-types
impl<'a> TryFrom<&weedle::types::NonAnyType<'a>> for AttributeType {
    type Error = ();

    fn try_from(ty: &weedle::types::NonAnyType) -> Result<Self, Self::Error> {
        use weedle::types::*;

        Ok(match ty {
            NonAnyType::Boolean(_) => AttributeType::Bool,
            NonAnyType::Integer(i) => match i.type_ {
                IntegerType::Short(t) => {
                    if t.unsigned.is_some() {
                        AttributeType::U16
                    } else {
                        AttributeType::I16
                    }
                }
                IntegerType::Long(t) => {
                    if t.unsigned.is_some() {
                        AttributeType::U32
                    } else {
                        AttributeType::I32
                    }
                }
                IntegerType::LongLong(t) => {
                    if t.unsigned.is_some() {
                        AttributeType::U64
                    } else {
                        AttributeType::I64
                    }
                }
            },
            NonAnyType::FloatingPoint(f) => match f.type_ {
                FloatingPointType::Float(_) => AttributeType::F32,
                FloatingPointType::Double(_) => AttributeType::F64,
            },
            NonAnyType::USVString(_) => AttributeType::String,
            NonAnyType::DOMString(_) => AttributeType::String,
            NonAnyType::Object(_) => return Err(()),
            NonAnyType::Identifier(_) => return Err(()),
            _ => panic!(),
        })
    }
}

impl TryFrom<Vec<Self>> for AttributeType {
    type Error = ();

    fn try_from(mut types: Vec<Self>) -> Result<Self, Self::Error> {
        types.sort();

        let mut bool_or_f64_or_string = vec![
            AttributeType::Bool,
            AttributeType::F64,
            AttributeType::String,
        ];
        bool_or_f64_or_string.sort();

        match types {
            types if types == bool_or_f64_or_string => Ok(AttributeType::BoolOrF64OrString),
            _ => Err(()),
        }
    }
}

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    let command = std::env::args().nth(1);
    match command {
        None => {
            Spec::fetch().await;
            Spec::prepare().await;
        }
        Some(arg) if arg == "fetch" => {
            Spec::fetch().await;
        }
        Some(arg) if arg == "prepare" => {
            Spec::prepare().await;
        }
        command => {
            tracing::error!(command, "Unknown command");
            return;
        }
    }
}
