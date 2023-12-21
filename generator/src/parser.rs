use crate::{flat_case, simplify, AttributeType};
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use weedle::{interface::*, mixin::*, *};

type Parse = fn(&Regex, &str, &mut ParsedElement) -> Option<()>;
type Counts = HashMap<(usize, Parse), usize>;

#[derive(Debug)]
pub struct Parser {
    category: Counts,
    context: Counts,
    content: Counts,
    tag_omission: Counts,
    attribute: Counts,
    idl: Counts,
    regexes: Vec<Regex>,
}

impl Parser {
    pub fn new() -> Self {
        let mut parser = Self {
            category: HashMap::new(),
            context: HashMap::new(),
            content: HashMap::new(),
            tag_omission: HashMap::new(),
            attribute: HashMap::new(),
            idl: HashMap::new(),
            regexes: Vec::new(),
        };

        let mut push = |res: &[&str], f, map: &mut Counts| {
            for re in res {
                let index = parser.regexes.len();
                let regex = Regex::new(&format!("^{re}$")).unwrap();

                map.insert((index, f), 0);
                parser.regexes.push(regex);
            }
        };

        for (res, f) in Self::category_parsers() {
            push(res, *f, &mut parser.category);
        }
        for (res, f) in Self::context_parsers() {
            push(res, *f, &mut parser.context);
        }
        for (res, f) in Self::content_parsers() {
            push(res, *f, &mut parser.content);
        }
        for (res, f) in Self::tag_omission_parsers() {
            push(res, *f, &mut parser.tag_omission);
        }
        for (res, f) in Self::attribute_parsers() {
            push(res, *f, &mut parser.attribute);
        }
        for (res, f) in Self::idl_parsers() {
            push(res, *f, &mut parser.idl);
        }

        parser
    }

    pub fn category(&mut self, text: &str, element: &mut ParsedElement) {
        Self::parse(
            "category",
            &mut self.category,
            &mut self.regexes,
            text,
            element,
        );
    }

    pub fn context(&mut self, text: &str, element: &mut ParsedElement) {
        Self::parse(
            "context",
            &mut self.context,
            &mut self.regexes,
            text,
            element,
        );
    }

    pub fn content(&mut self, text: &str, element: &mut ParsedElement) {
        Self::parse(
            "content",
            &mut self.content,
            &mut self.regexes,
            text,
            element,
        );
    }

    pub fn tag_omission(&mut self, text: &str, element: &mut ParsedElement) {
        Self::parse(
            "tag_omission",
            &mut self.tag_omission,
            &mut self.regexes,
            text,
            element,
        );
    }

    pub fn attribute(&mut self, text: &str, element: &mut ParsedElement) {
        Self::parse(
            "attribute",
            &mut self.attribute,
            &mut self.regexes,
            text,
            element,
        );
    }

    pub fn idl(&mut self, text: &str, element: &mut ParsedElement) {
        Self::parse("idl", &mut self.idl, &mut self.regexes, text, element);
    }

    pub fn errors(&self) {
        let errors = |name: &str, counts: &Counts| {
            for ((index, _), count) in counts {
                if *count == 0 {
                    tracing::warn!(
                        index,
                        re = %self.regexes[*index],
                        "No matches (Parser.{name})",
                    );
                }
            }
        };

        errors("categories", &self.category);
        errors("context", &self.context);
        errors("content", &self.content);
        errors("tag_omission", &self.tag_omission);
        errors("attribute", &self.attribute);
        errors("idl", &self.idl);
    }
}

/// Private.
impl Parser {
    fn parse(
        section: &str,
        counts: &mut Counts,
        regexes: &mut [Regex],
        text: &str,
        element: &mut ParsedElement,
    ) {
        for ((index, parse), count) in counts {
            if parse(&regexes[*index], text, element).is_some() {
                *count += 1;
                return;
            }
        }

        tracing::warn!(element.name, section, text, "😓");
    }

    fn category_parsers() -> &'static [(&'static [&'static str], Parse)] {
        &[(
            &[
                r"(\S+) content",
                r"(\S+) element",
                r"(\S+) (\S+) element",
                r"if the element is allowed in the body: (\S+) content",
                r"if the element has an? \S+ attribute: (\S+) content",
                r"if the element's children include at least one \S+ element: (\S+) content",
                r"if the element's children include at least one \S+ group: (\S+) content",
                r"if the \S+ attribute is present: (\S+) content",
                r"if the \S+ attribute is in the \S+ state: (\S+), (\S+), (\S+), and (\S+) (\S+) element",
                r"if the \S+ attribute is not in the \S+ state: (\S+) content",
                r"if the \S+ attribute is not in the \S+ state: (\S+), (\S+), (\S+), (\S+), and (\S+) (\S+) element",
                r"(\S+) and (\S+) (\S+) element",
                r"(\S+), (\S+), (\S+), and (\S+) (\S+) element",
                r"(\S+), (\S+), (\S+), (\S+), and (\S+) (\S+) element",
                r"none",
            ],
            |re: &Regex, text: &str, element: &mut ParsedElement| {
                let text = simplify(text);
                let categories = re
                    .captures(&text)?
                    .iter()
                    .skip(1)
                    .map(|capture| capture.unwrap().as_str())
                    .collect::<Vec<_>>();

                tracing::debug!(element.name, ?categories, text, "😍");
                for category in categories {
                    element.categories.insert(category.to_owned());
                }

                Some(())
            },
        )]
    }

    fn context_parsers() -> &'static [(&'static [&'static str], Parse)] {
        &[
            (
                &[
                    r"inside (\S+) elements",
                    r"where (\S+) elements are expected",
                    r"where (\S+) content is expected",
                    r"where (\S+) content is expected, but only if there is an? \S+ element ancestor",
                    r"where (\S+) content is expected, but only if it is a hierarchically correct \S+ element",
                    r"where (\S+) content is expected in html documents, if there are no ancestor \S+ elements",
                    r"as a child of an? (\S+) element",
                    r"as a child of an? (\S+) element that doesn't have an? \S+ attribute",
                    r"as a child of an? (\S+) element, before the \S+ element",
                    r"as a child of an? (\S+) element, before any \S+ content",
                    r"as a child of an? (\S+) element, before any \S+ content or \S+ elements",
                    r"as a child of an? (\S+) element, after all \S+ elements",
                    r"as a child of an? (\S+) element, after any \S+, \S+, and \S+ elements, but only if there are no \S+ elements that are children of the \S+ element",
                    r"as a child of an? (\S+) element, after any \S+, \S+, \S+, \S+, and \S+ elements, but only if there are no other \S+ elements that are children of the \S+ element",
                    r"as a child of an? (\S+) element, after any \S+ elements and before any \S+, \S+, \S+, and \S+ elements",
                    r"as a child of an? (\S+) element, after any \S+, and \S+ elements and before any \S+, \S+, and \S+ elements, but only if there are no other \S+ elements that are children of the \S+ element",
                    r"as a child of an? (\S+) element, either immediately before or immediately after an? \S+ element",
                    r"as the first child of an? (\S+) element",
                    r"as the first element in an? (\S+) element",
                    r"as the first element child of an? (\S+) element",
                    r"as the second element in an? (\S+) element",
                    r"as the first or last child of an? (\S+) element",
                    r"in an? (\S+) element that is a child of an? \S+ element",
                    r"in an? (\S+) element containing no other \S+ elements",
                    r"in an? (\S+) element of an html document, if there are no ancestor \S+ elements",
                    r"if the \S+ attribute is present: where (\S+) content is expected",
                    r"if the \S+ attribute is present but not in the \S+ declaration state: in an? (\S+) element",
                    r"if the \S+ attribute is present but not in the \S+ declaration state: in an? (\S+) element that is a child of an? \S+ element",
                    r"if the \S+ attribute is present, or if the element's \S+ attribute is in the \S+ declaration state: in an? (\S+) element",
                    r"if the element is allowed in the body: where (\S+) content is expected",
                    r"before \S+ or \S+ elements inside (\S+) elements",
                    r"after \S+ or \S+ elements inside (\S+) elements",
                    r"before \S+ or \S+ elements inside (\S+) elements that are children of an? \S+ element",
                    r"after \S+ or \S+ elements inside (\S+) elements that are children of an? \S+ element",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = simplify(text);
                    let contexts = re
                        .captures(&text)?
                        .iter()
                        .skip(1)
                        .map(|capture| capture.unwrap().as_str())
                        .collect::<Vec<_>>();

                    tracing::trace!(element.name, ?contexts, text, "😍");
                    for context in contexts {
                        element.contexts.insert(context.to_owned());
                    }

                    Some(())
                },
            ),
            (
                &[
                    r"as document's document element",
                    r"wherever a subdocument fragment is allowed in a compound document",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = simplify(text);
                    re.captures(&text)?;
                    assert_eq!(element.name, "html");

                    tracing::trace!(element.name, text, "😍");
                    Some(())
                },
            ),
        ]
    }

    // TODO ..., but with no ... descendants. (Pass that to next stage?)
    fn content_parsers() -> &'static [(&'static [&'static str], Parse)] {
        &[
            (
                &[
                    r"(\S+)",
                    r"(\S+) content",
                    r"(\S+) content, optionally intermixed with \S+ content",
                    r"(\S+) content, but there must be no \S+ element descendants",
                    r"(\S+) content, but with no descendant \S+ elements",
                    r"(\S+) content, but with no \S+ element descendants",
                    r"(\S+) content, but with no \S+ or \S+ element descendants",
                    r"(\S+) content, but with no \S+, \S+, \S+ content, or \S+ content descendants",
                    r"(\S+) content, but with no descendant \S+ elements unless it is the element's labeled control, and no descendant \S+ elements",
                    r"(\S+) content, but with no \S+ content descendants, no \S+ content descendants, and no \S+, \S+, or \S+ element descendants",
                    r"(\S+) content, but there must be no \S+ content descendant and no descendant with the \S+ attribute specified",
                    r"(\S+), but there must be no \S+ content descendant, \S+ element descendant, or descendant with the \S+ attribute specified",
                    r"(\S+), but with no \S+ content descendants except for \S+ elements, \S+ elements with \S+ attributes, \S+ elements, \S+ elements whose \S+ attribute are in the \S+ or \S+ \S+ states, \S+ elements that are \S+, and \S+ elements with an? \S+ attribute or an? \S+ \S+ greater than 1",
                    r"(\S+) that is not inter-element whitespace",
                    r"(\S+) that gives a conformant style sheet",
                    r"(\S+) \(for clarification, see example\)",
                    r"if the \S+ attribute is present: (\S+)",
                    r"if the \S+ attribute is absent: zero or more (\S+) and (\S+) elements",
                    r"if the element is not a child of an? \S+ element: (\S+) content",
                    r"if the element has no \S+ attribute and is a child of an? \S+ element: (\S+)",
                    r"if the element has no \S+ attribute and is not a child of an? \S+ element: (\S+) that is not inter-element whitespace",
                    r"if the element has an? \S+ attribute: (\S+) content",
                    r"if the element has an? \S+ attribute but no \S+ attribute: (\S+)",
                    r"if the element has an? \S+ attribute: zero or more (\S+) elements, then (\S+), but with no \S+ element descendants",
                    r"if the element does not have an? \S+ attribute: zero or more (\S+) elements, then zero or more (\S+) elements, then (\S+), but with no \S+ element descendants",
                    r"if the element is a child of an? \S+ element: one or more (\S+) elements followed by one or more (\S+) elements, optionally intermixed with (\S+) elements",
                    r"if the element has an? \S+ attribute and an? \S+ attribute: (\S+)",
                    r"if the document is an? \S+ \S+ document or if \S+ information is available from a higher-level protocol: zero or more elements of (\S+) content, of which no more than one is a (\S+) element and no more than one is a (\S+) element",
                    r"in this order: optionally an? (\S+) element, followed by zero or more (\S+) elements, followed optionally by an? (\S+) element, followed by either zero or more (\S+) elements or one or more (\S+) elements, followed optionally by a (\S+) element, optionally intermixed with one or more (\S+) elements",
                    r"a (\S+) element followed by a (\S+) element",
                    r"one (\S+) element followed by (\S+) content",
                    r"optionally a (\S+) element, followed by (\S+) content",
                    r"zero or more (\S+) and (\S+) elements",
                    r"zero or more (\S+), (\S+), and (\S+) elements",
                    r"zero or more (\S+), (\S+), (\S+), and (\S+) elements",
                    r"zero or more (\S+) elements, followed by one (\S+) element, optionally intermixed with (\S+) elements",
                    r"zero or more (\S+) elements, followed by one (\S+), (\S+), (\S+), (\S+), (\S+), or (\S+) element, followed by zero or more (\S+) elements, optionally intermixed with (\S+) elements",
                    r"either: (\S+) content",
                    r"either: one (\S+) element followed by (\S+) content",
                    r"either: zero or more groups each consisting of one or more (\S+) elements followed by one or more (\S+) elements, optionally intermixed with (\S+) elements",
                    r"or: zero or more (\S+) and (\S+) elements",
                    r"or: (\S+) content",
                    r"or: (\S+) content followed by one (\S+) element",
                    r"or: one or more (\S+) elements, optionally intermixed with (\S+) elements",
                    r"otherwise: (\S+), but must match requirements described in \S+ below",
                    r"otherwise: one or more elements of (\S+) content, of which exactly one is a (\S+) element and no more than one is a (\S+) element",
                    r"when scripting is disabled, not in an? \S+ element: (\S+), but there must be no \S+ element descendants",
                    r"when scripting is disabled, in an? \S+ element: in any order, zero or more (\S+) elements, zero or more (\S+) elements, and zero or more (\S+) elements",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = simplify(text);
                    let contents = re
                        .captures(&text)?
                        .iter()
                        .skip(1)
                        .map(|capture| capture.unwrap().as_str())
                        .collect::<Vec<_>>();

                    tracing::trace!(element.name, ?contents, text, "😍");
                    for content in contents {
                        element.contents.insert(content.to_owned());
                    }

                    Some(())
                },
            ),
            (
                &[
                    r"if there is no \S+ attribute, depends on the value of the \S+ attribute, but must match script content restrictions",
                    r"if there is an? \S+ attribute, the element must be either empty or contain only script documentation that also matches script content restrictions",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = simplify(text);
                    re.captures(&text)?;
                    assert_eq!(element.name, "script");

                    tracing::trace!(element.name, text, "😍");
                    Some(())
                },
            ),
            (
                &[r"otherwise: text that conforms to the requirements given in the prose"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = simplify(text);
                    re.captures(&text)?;
                    assert_eq!(element.name, "noscript");

                    tracing::debug!(element.name, text, "😍");
                    Some(())
                },
            ),
            (
                // TODO: Read the prose for <ruby>
                &[r"see prose"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = simplify(text);
                    re.captures(&text)?;
                    assert_eq!(element.name, "ruby");

                    tracing::debug!(element.name, text, "😍");
                    Some(())
                },
            ),
        ]
    }

    fn tag_omission_parsers() -> &'static [(&'static [&'static str], Parse)] {
        &[
            (
                &[
                    r"neither tag is omissible",
                    r"an? \S+ element's start|end tag can be omitted if .*",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = simplify(text);
                    re.captures(&text)?;
                    let end_tag = true;

                    tracing::debug!(element.name, end_tag, text, "😍");
                    element.end_tag = end_tag;
                    Some(())
                },
            ),
            (
                &[r"no end tag"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = simplify(text);
                    re.captures(&text)?;
                    let end_tag = false;

                    tracing::debug!(element.name, end_tag, text, "😍");
                    element.end_tag = end_tag;
                    Some(())
                },
            ),
        ]
    }

    fn attribute_parsers() -> &'static [(&'static [&'static str], Parse)] {
        &[
            (
                &[r"global attributes"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = simplify(text);
                    re.captures(&text)?;

                    tracing::debug!(element.name, text, "😍");
                    Some(())
                },
            ),
            (
                &[
                    r"(\S+)",
                    r"(\S+) — (.*)",
                    r"(\S+) \(in \S+\) — (.*)",
                    r"(\S+) \(in \S+ or \S+\) — (.*)",
                    r"if the element is not a child of an \S+ or \S+ element: (\S+) — (.*)",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = simplify(text);
                    let captures = re.captures(&text)?;
                    let name = captures.get(1).unwrap().as_str().to_string();
                    let description = captures
                        .get(2)
                        .map(|description| description.as_str())
                        .unwrap_or_default()
                        .to_string();

                    tracing::debug!(element.name, name, description, text, "😍");
                    element.attributes.insert(name, description);
                    Some(())
                },
            ),
            (
                &[
                    r"also, the \S+ attribute has special semantics on this element: .*",
                    r"also, the \S+ global attribute has special semantics on this element",
                    r"any other attribute that has no namespace \(see prose\)",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = simplify(text);
                    re.captures(&text)?;

                    tracing::debug!(element.name, text, "😍");
                    Some(())
                },
            ),
        ]
    }

    fn idl_parsers() -> &'static [(&'static [&'static str], Parse)] {
        &[
            (
                &[
                    r"Uses? (\S+)\.",
                    r"Uses (\S+), as defined for \S+ elements\.",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    // We don't simplify here to capture case.
                    let captures = re.captures(&text)?;
                    let uses = captures[1].to_string();

                    tracing::debug!(element.name, uses, text, "😍");
                    element.interface = uses;
                    Some(())
                },
            ),
            (
                &[r"\[exposed=window.*", r"typedef .*"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    // We only simplify the haystack here, parsing needs original text.
                    re.captures(&simplify(text))?;

                    tracing::debug!(element.name, "😍");
                    element.interface = ParsedIdl::parse(text)
                        .interfaces
                        .values()
                        .next()
                        .unwrap()
                        .name
                        .clone();
                    Some(())
                },
            ),
        ]
    }
}

#[derive(Debug)]
pub struct ParsedElement {
    pub name: String,
    pub categories: BTreeSet<String>,
    pub contexts: BTreeSet<String>,
    pub contents: BTreeSet<String>,
    pub end_tag: bool,
    pub attributes: BTreeMap</* name: */ String, /* description: */ String>,
    pub interface: String,
}

#[derive(Clone, Debug)]
pub struct ParsedInterface {
    pub name: String,
    pub inherits: Option<String>,
    pub attributes: BTreeMap</* name: */ String, AttributeType>,
}

#[derive(Debug)]
pub struct ParsedIdl {
    pub interfaces: BTreeMap</* name: */ String, ParsedInterface>,
    pub mixins: BTreeMap</* name: */ String, ParsedInterface>,
    pub includes: BTreeMap</* left: */ String, /* right: */ BTreeSet<String>>,
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

        // Are we complete?
        // Fow now, we only miss interfaces/mixins defined in other specifcations.
        for interface in interfaces.values() {
            let name = &interface.name;

            if let Some(includes) = includes.get(&interface.name) {
                for include in includes {
                    if !mixins.contains_key(include) {
                        tracing::trace!("😓 {name} includes {include}");
                    }
                }
            }

            if let Some(inherits) = interface.inherits.as_ref() {
                if !interfaces.contains_key(inherits) {
                    tracing::trace!("😓 {name} : {inherits}");
                }
            }
        }

        Self {
            interfaces,
            mixins,
            includes,
        }
    }

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
                        AttributeType::try_from(&attribute.type_.type_).ok()?,
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
                        AttributeType::try_from(&attribute.type_.type_).ok()?,
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

    pub fn resolve(
        &self,
        name: &str,
    ) -> BTreeMap</* flat_case_name: */ String, (/* original_name: */ String, AttributeType)> {
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
                        .insert(flat_case(name), (name.clone(), *ty))
                        .is_none());
                }
                attributes
            })
    }
}
