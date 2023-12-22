use crate::{parsers::idl::ParsedIdl, text};
use regex::Regex;
use std::collections::{BTreeSet, HashMap};

type Parse = fn(&Regex, &str, &mut ParsedElement) -> Option<()>;
type Counts = HashMap<(usize, Parse), usize>;

#[derive(Debug)]
pub struct ParsedElement {
    pub name: String,
    pub categories: BTreeSet<String>,
    pub contents: BTreeSet<String>,
    pub end_tag: bool,
    pub attributes: BTreeSet<String>,
    pub interface: Option<String>,
    pub custom: bool,
}

#[derive(Debug)]
pub struct Parser {
    category: Counts,
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
                        "🤔 No matches (Parser.{name}())",
                    );
                }
            }
        };

        errors("categories", &self.category);
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

        tracing::warn!(element.name, section, text, "😓 No matches");
    }

    fn category_parsers() -> &'static [(&'static [&'static str], Parse)] {
        &[(
            &[
                r"(\S+) content",
                r"(\S+) element",
                r"(\S+) (\S+) element",
                r"(\S+) and (\S+) (\S+) element",
                r"(\S+), (\S+), (\S+), and (\S+) (\S+) element",
                r"(\S+), (\S+), (\S+), (\S+), and (\S+) (\S+) element",
                r"if the element is allowed in the body: (\S+) content",
                r"if the element has an? \S+ attribute: (\S+) content",
                r"if the element's children include at least one \S+ element: (\S+) content",
                r"if the element's children include at least one \S+ group: (\S+) content",
                r"if the \S+ attribute is present: (\S+) content",
                r"if the \S+ attribute is in the \S+ state: (\S+), (\S+), (\S+), and (\S+) (\S+) element",
                r"if the \S+ attribute is not in the \S+ state: (\S+) content",
                r"if the \S+ attribute is not in the \S+ state: (\S+), (\S+), (\S+), (\S+), and (\S+) (\S+) element",
                r"for \S+ custom elements: (\S+), (\S+), (\S+), and (\S+) (\S+) element",
                r"none",
            ],
            |re: &Regex, text: &str, element: &mut ParsedElement| {
                let text = text::simplify(text);
                let categories = re
                    .captures(&text)?
                    .iter()
                    .skip(1)
                    .map(|capture| capture.unwrap().as_str())
                    .collect::<Vec<_>>();

                tracing::debug!(element.name, ?categories, text, "🥳");
                for category in categories {
                    element.categories.insert(category.to_owned());
                }

                Some(())
            },
        )]
    }

    // TODO ..., but with no ... descendants. (Is this even possible?) (Pass that to next stage?)
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
                    let text = text::simplify(text);
                    let contents = re
                        .captures(&text)?
                        .iter()
                        .skip(1)
                        .map(|capture| capture.unwrap().as_str())
                        .collect::<Vec<_>>();

                    tracing::debug!(element.name, ?contents, text, "🥳");
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
                    let text = text::simplify(text);
                    re.captures(&text)?;
                    assert_eq!(element.name, "script");

                    tracing::debug!(element.name, text, "🥳");
                    Some(())
                },
            ),
            (
                &[r"otherwise: text that conforms to the requirements given in the prose"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = text::simplify(text);
                    re.captures(&text)?;
                    assert_eq!(element.name, "noscript");

                    tracing::debug!(element.name, text, "🥳");
                    Some(())
                },
            ),
            (
                // TODO: Read the prose for <ruby>
                &[r"see prose"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = text::simplify(text);
                    re.captures(&text)?;
                    assert_eq!(element.name, "ruby");

                    tracing::debug!(element.name, text, "🥳");
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
                    let text = text::simplify(text);
                    re.captures(&text)?;
                    let end_tag = true;

                    tracing::debug!(element.name, end_tag, text, "🥳");
                    element.end_tag = end_tag;
                    Some(())
                },
            ),
            (
                &[r"no end tag"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = text::simplify(text);
                    re.captures(&text)?;
                    let end_tag = false;

                    tracing::debug!(element.name, end_tag, text, "🥳");
                    element.end_tag = end_tag;
                    Some(())
                },
            ),
        ]
    }

    fn attribute_parsers() -> &'static [(&'static [&'static str], Parse)] {
        &[
            (
                &[
                    r"(\S+)",
                    r"(\S+) — .*",
                    r"(\S+) \(in \S+\) — .*",
                    r"(\S+) \(in \S+ or \S+\) — .*",
                    r"(\S+), for \S+ custom elements — .*",
                    r"if the element is not a child of an \S+ or \S+ element: (\S+) — .*",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = text::simplify(text);
                    let captures = re.captures(&text)?;
                    let name = captures.get(1).unwrap().as_str().to_string();

                    tracing::debug!(element.name, name, text, "🥳");
                    element.attributes.insert(name);
                    Some(())
                },
            ),
            (
                &[
                    r"global attributes",
                    r"global attributes, except the \S+ attribute",
                    r"also, the \S+ attribute has special semantics on this element: .*",
                    r"also, the \S+ global attribute has special semantics on this element",
                    r"any other attribute that has no namespace \(see prose\)",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let text = text::simplify(text);
                    re.captures(&text)?;

                    tracing::debug!(element.name, text, "🥳");
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
                    r"Supplied by the element's author \(inherits from (\S+)\)",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    // We don't simplify here to capture case.
                    let captures = re.captures(&text)?;
                    let uses = captures[1].to_string();

                    tracing::debug!(element.name, uses, text, "🥳");
                    element.interface = Some(uses);
                    Some(())
                },
            ),
            (
                &[r"\[exposed=window.*", r"typedef .*"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    // We only simplify the haystack here, parsing needs original text.
                    re.captures(&text::simplify(text))?;

                    tracing::debug!(element.name, "🥳");
                    element.interface = Some(
                        ParsedIdl::parse(text)
                            .interfaces
                            .values()
                            .next()
                            .unwrap()
                            .name
                            .clone(),
                    );
                    Some(())
                },
            ),
        ]
    }
}
