use crate::{text, AttributeType};
use std::collections::{BTreeMap, BTreeSet};
use weedle::{interface::*, mixin::*, types::*, *};

#[derive(Clone, Debug)]
pub struct ParsedInterface {
    pub name: String,
    pub inherits: Option<String>,
    pub attributes: BTreeMap<String, AttributeType>,
}

#[derive(Debug)]
pub struct ParsedIdl {
    pub interfaces: BTreeMap<String, ParsedInterface>,
    pub mixins: BTreeMap<String, ParsedInterface>,
    pub includes: BTreeMap<String, BTreeSet<String>>,
}

impl ParsedIdl {
    pub fn parse(text: &str) -> Self {
        let definitions = weedle::parse(&text).unwrap();

        definitions.iter().for_each(|definition| match definition {
            // We only care about interfaces, mixins, and includes.
            Definition::Interface(_) => {}
            Definition::InterfaceMixin(_) => {}
            Definition::IncludesStatement(_) => {}
            // We could care about those, but don't.
            // ("... : ..." and "... includes ..." don't reference partial definitions)
            // (from this spec, at least)
            Definition::PartialInterface(_) => {}
            Definition::PartialInterfaceMixin(_) => {}
            // We don't care about these.
            Definition::Callback(_) => {}
            Definition::Dictionary(_) => {}
            Definition::Enum(_) => {}
            Definition::Typedef(_) => {}
            // Those are not found in the spec.
            Definition::CallbackInterface(_) => unreachable!(),
            Definition::Namespace(_) => unreachable!(),
            Definition::PartialDictionary(_) => unreachable!(),
            Definition::PartialNamespace(_) => unreachable!(),
            Definition::Implements(_) => unreachable!(),
        });

        let interfaces = definitions
            .iter()
            .filter_map(|definition| match definition {
                Definition::Interface(interface) => Some(interface),
                _ => None,
            })
            .map(|interface| Self::parse_interface(interface))
            .map(|interface| (interface.name.clone(), interface))
            .fold(BTreeMap::new(), |mut map, (name, interface)| {
                assert!(map.insert(name, interface).is_none());
                map
            });
        let mixins = definitions
            .iter()
            .filter_map(|definition| match definition {
                Definition::InterfaceMixin(mixin) => Some(mixin),
                _ => None,
            })
            .map(|mixin| Self::parse_mixin(mixin))
            .map(|mixin| (mixin.name.clone(), mixin))
            .fold(BTreeMap::new(), |mut map, (name, mixin)| {
                assert!(map.insert(name, mixin).is_none());
                map
            });
        let includes = definitions
            .iter()
            .filter_map(|definition| match definition {
                Definition::IncludesStatement(includes) => Some(includes),
                _ => None,
            })
            .map(|includes| Self::parse_includes(includes))
            .fold(
                BTreeMap::<String, BTreeSet<String>>::new(),
                |mut map, (left, right)| {
                    assert!(map.entry(left).or_default().insert(right));
                    map
                },
            );

        Self {
            interfaces,
            mixins,
            includes,
        }
    }

    pub fn check(self) -> Self {
        // NOTE
        // Are we complete?
        // Fow now, we only miss interfaces/mixins defined in other specifcations.
        for interface in self.interfaces.values() {
            let name = &interface.name;

            if let Some(includes) = self.includes.get(&interface.name) {
                for include in includes {
                    if !self.mixins.contains_key(include) {
                        tracing::debug!("😳 {name} includes {include}");
                    }
                }
            }

            if let Some(inherits) = interface.inherits.as_ref() {
                if !self.interfaces.contains_key(inherits) {
                    tracing::debug!("😳 {name} : {inherits}");
                }
            }
        }

        self
    }

    pub fn resolve(&self, name: &str) -> BTreeMap<String, (String, AttributeType)> {
        let interface = self.interfaces.get(name).unwrap();
        let mut inheritances = vec![interface];

        while let Some(inherits) = inheritances.last().unwrap().inherits.as_ref() {
            if let Some(inherits) = self.interfaces.get(inherits) {
                inheritances.push(inherits);
            } else {
                break;
            }
        }

        if let Some(mixins) = self.includes.get(name) {
            for mixin in mixins {
                if let Some(mixin) = self.mixins.get(mixin) {
                    inheritances.insert(0, mixin);
                }
            }
        }

        inheritances
            .into_iter()
            .rev()
            .fold(BTreeMap::new(), |mut attributes, interface| {
                for (name, ty) in &interface.attributes {
                    assert!(attributes
                        .insert(text::flat(name), (name.clone(), *ty))
                        .is_none());
                }
                attributes
            })
    }
}

/// Private.
impl ParsedIdl {
    fn parse_interface(interface: &InterfaceDefinition) -> ParsedInterface {
        ParsedInterface {
            name: interface.identifier.0.to_string(),
            inherits: interface
                .inheritance
                .map(|parent| parent.identifier.0.to_string()),
            attributes: interface
                .members
                .body
                .iter()
                .filter_map(|member| match member {
                    InterfaceMember::Attribute(attribute) => Some((
                        attribute.identifier.0.to_string(),
                        Self::parse_type(&attribute.type_.type_)?,
                    )),
                    _ => None,
                })
                .collect(),
        }
    }

    fn parse_mixin(interface: &InterfaceMixinDefinition) -> ParsedInterface {
        ParsedInterface {
            name: interface.identifier.0.to_string(),
            inherits: None,
            attributes: interface
                .members
                .body
                .iter()
                .filter_map(|member| match member {
                    MixinMember::Attribute(attribute) => Some((
                        attribute.identifier.0.to_string(),
                        Self::parse_type(&attribute.type_.type_)?,
                    )),
                    _ => None,
                })
                .collect(),
        }
    }

    fn parse_includes(includes: &IncludesStatementDefinition) -> (String, String) {
        (
            includes.lhs_identifier.0.to_string(),
            includes.rhs_identifier.0.to_string(),
        )
    }

    fn parse_type(ty: &Type) -> Option<AttributeType> {
        match ty {
            Type::Single(ty) => match ty {
                SingleType::NonAny(ty) => Self::parse_non_any_type(ty),
                SingleType::Any(_) => {
                    tracing::debug!("🫠 Ignoring SingleType::Any");
                    return None;
                }
            },
            Type::Union(union) => AttributeType::collapse(
                union
                    .type_
                    .body
                    .list
                    .iter()
                    .filter_map(|ty| match ty {
                        UnionMemberType::Single(ty) => Self::parse_non_any_type(&ty.type_),
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>(),
            ),
        }
    }

    fn parse_non_any_type(ty: &NonAnyType) -> Option<AttributeType> {
        // https://webidl.spec.whatwg.org/#idl-types
        Some(match ty {
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
                tracing::debug!("🫠 Ignoring NonAnyType::Object");
                return None;
            }
            NonAnyType::Identifier(_) => {
                tracing::debug!("🫠 Ignoring NonAnyType::Identifier");
                return None;
            }
            ty => {
                tracing::debug!(?ty, "🫠 Ignoring NonAnyType::_");
                return None;
            }
        })
    }
}
