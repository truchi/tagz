mod parser;
mod spec;

use serde::{Deserialize, Serialize};
use spec::Spec;

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

    BoolOrF64OrString,
}

impl<'a> TryFrom<&weedle::types::Type<'a>> for AttributeType {
    type Error = ();

    fn try_from(ty: &weedle::types::Type<'a>) -> Result<Self, Self::Error> {
        use weedle::types::*;

        match ty {
            Type::Single(ty) => match ty {
                SingleType::NonAny(ty) => ty.try_into(),
                SingleType::Any(_) => {
                    tracing::trace!("Ignoring SingleType::Any");
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
                    _ => unreachable!(),
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
            NonAnyType::Object(_) => {
                tracing::trace!("Ignoring NonAnyType::Object");
                return Err(());
            }
            NonAnyType::Identifier(_) => {
                tracing::trace!("Ignoring NonAnyType::Identifier");
                return Err(());
            }
            ty => {
                tracing::trace!(?ty, "Ignoring NonAnyType::_");
                return Err(());
            }
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
            types => {
                tracing::trace!(?types, "Unknown types");
                Err(())
            }
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
