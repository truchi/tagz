use crate::spec::ParsedElement;
use regex::Regex;
use std::collections::HashMap;

type Parse = fn(&Regex, &str, &mut ParsedElement) -> Option<()>;
type Counts = HashMap<(usize, Parse), usize>;

#[derive(Debug)]
pub struct Parser {
    categories: Counts,
    context: Counts,
    content: Counts,
    regexes: Vec<Regex>,
}

impl Parser {
    pub fn new() -> Self {
        let mut parser = Self {
            categories: HashMap::new(),
            context: HashMap::new(),
            content: HashMap::new(),
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
            push(res, *f, &mut parser.categories);
        }
        for (res, f) in Self::context_parsers() {
            push(res, *f, &mut parser.context);
        }
        for (res, f) in Self::content_parsers() {
            push(res, *f, &mut parser.content);
        }

        parser
    }

    pub fn category(&mut self, text: &str, element: &mut ParsedElement) {
        Self::parse(
            "category",
            &mut self.categories,
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

        errors("categories", &self.categories);
        errors("context", &self.context);
        errors("content", &self.content);
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
        // Collapse whitespace.
        let re = Regex::new(r"\s+").unwrap();
        let text = re.replace_all(&text, " ");
        let text = text.trim().trim_end_matches('.');

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
                r"If the element is allowed in the body: (\S+) content",
                r"If the element has an? \S+ attribute: (\S+) content",
                r"If the element's children include at least one \S+ element: (\S+) content",
                r"If the element's children include at least one \S+ group: (\S+) content",
                r"If the \S+ attribute is present: (\S+) content",
                r"If the \S+ attribute is in the \S+ state: (\S+), (\S+), (\S+), and (\S+) (\S+) element",
                r"If the \S+ attribute is not in the \S+ state: (\S+) content",
                r"If the \S+ attribute is not in the \S+ state: (\S+), (\S+), (\S+), (\S+), and (\S+) (\S+) element",
                r"(\S+) and (\S+) (\S+) element",
                r"(\S+), (\S+), (\S+), and (\S+) (\S+) element",
                r"(\S+), (\S+), (\S+), (\S+), and (\S+) (\S+) element",
                r"None",
            ],
            |re: &Regex, text: &str, element: &mut ParsedElement| {
                let captures = re.captures(text)?;
                let categories = captures
                    .iter()
                    .skip(1)
                    .map(|capture| capture.unwrap().as_str())
                    .map(|capture| capture.parse().unwrap())
                    .collect::<Vec<_>>();
                tracing::trace!(element.name, ?categories, text, "😍");

                for category in categories {
                    element.categories.insert(category);
                }

                Some(())
            },
        )]
    }

    fn context_parsers() -> &'static [(&'static [&'static str], Parse)] {
        &[
            (
                &[
                    r"Inside (\S+) elements",
                    r"Where (\S+) elements are expected",
                    r"Where (\S+) content is expected",
                    r"Where (\S+) content is expected, but only if there is an? \S+ element ancestor",
                    r"Where (\S+) content is expected, but only if it is a hierarchically correct \S+ element",
                    r"Where (\S+) content is expected in HTML documents, if there are no ancestor \S+ elements",
                    r"As a child of an? (\S+) element",
                    r"As a child of an? (\S+) element that doesn't have an? \S+ attribute",
                    r"As a child of an? (\S+) element, before the \S+ element",
                    r"As a child of an? (\S+) element, before any \S+ content",
                    r"As a child of an? (\S+) element, before any \S+ content or \S+ elements",
                    r"As a child of an? (\S+) element, after all \S+ elements",
                    r"As a child of an? (\S+) element, after any \S+, \S+, and \S+ elements, but only if there are no \S+ elements that are children of the \S+ element",
                    r"As a child of an? (\S+) element, after any \S+, \S+, \S+, \S+, and \S+ elements, but only if there are no other \S+ elements that are children of the \S+ element",
                    r"As a child of an? (\S+) element, after any \S+ elements and before any \S+, \S+, \S+, and \S+ elements",
                    r"As a child of an? (\S+) element, after any \S+, and \S+ elements and before any \S+, \S+, and \S+ elements, but only if there are no other \S+ elements that are children of the \S+ element",
                    r"As a child of an? (\S+) element, either immediately before or immediately after an? \S+ element",
                    r"As the first child of an? (\S+) element",
                    r"As the first element in an? (\S+) element",
                    r"As the first element child of an? (\S+) element",
                    r"As the second element in an? (\S+) element",
                    r"As the first or last child of an? (\S+) element",
                    r"In an? (\S+) element that is a child of an? \S+ element",
                    r"In an? (\S+) element containing no other \S+ elements",
                    r"In an? (\S+) element of an HTML document, if there are no ancestor \S+ elements",
                    r"If the \S+ attribute is present: where (\S+) content is expected",
                    r"If the \S+ attribute is present but not in the \S+ declaration state: in an? (\S+) element",
                    r"If the \S+ attribute is present but not in the \S+ declaration state: in an? (\S+) element that is a child of an? \S+ element",
                    r"If the \S+ attribute is present, or if the element's \S+ attribute is in the \S+ declaration state: in an? (\S+) element",
                    r"If the element is allowed in the body: where (\S+) content is expected",
                    r"Before \S+ or \S+ elements inside (\S+) elements",
                    r"After \S+ or \S+ elements inside (\S+) elements",
                    r"Before \S+ or \S+ elements inside (\S+) elements that are children of an? \S+ element",
                    r"After \S+ or \S+ elements inside (\S+) elements that are children of an? \S+ element",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let captures = re.captures(text)?;
                    let contexts = captures
                        .iter()
                        .skip(1)
                        .map(|capture| capture.unwrap().as_str())
                        .map(|capture| capture.into())
                        .collect::<Vec<_>>();
                    tracing::trace!(element.name, ?contexts, text, "😍");

                    for context in contexts {
                        element.contexts.insert(context);
                    }

                    Some(())
                },
            ),
            (
                &[
                    r"As document's document element",
                    r"Wherever a subdocument fragment is allowed in a compound document",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    re.captures(text)?;
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
                    r"If the \S+ attribute is present: (\S+)",
                    r"If the \S+ attribute is absent: Zero or more (\S+) and (\S+) elements",
                    r"If the element is not a child of an? \S+ element: (\S+) content",
                    r"If the element has no \S+ attribute and is a child of an? \S+ element: (\S+)",
                    r"If the element has no \S+ attribute and is not a child of an? \S+ element: (\S+) that is not inter-element whitespace",
                    r"If the element has an? \S+ attribute: (\S+) content",
                    r"If the element has an? \S+ attribute but no \S+ attribute: (\S+)",
                    r"If the element has an? \S+ attribute: zero or more (\S+) elements, then (\S+), but with no \S+ element descendants",
                    r"If the element does not have an? \S+ attribute: zero or more (\S+) elements, then zero or more (\S+) elements, then (\S+), but with no \S+ element descendants",
                    r"If the element is a child of an? \S+ element: one or more (\S+) elements followed by one or more (\S+) elements, optionally intermixed with (\S+) elements",
                    r"If the element has an? \S+ attribute and an? \S+ attribute: (\S+)",
                    r"If the document is an? \S+ \S+ document or if \S+ information is available from a higher-level protocol: Zero or more elements of (\S+) content, of which no more than one is a (\S+) element and no more than one is a (\S+) element",
                    r"In this order: optionally an? (\S+) element, followed by zero or more (\S+) elements, followed optionally by an? (\S+) element, followed by either zero or more (\S+) elements or one or more (\S+) elements, followed optionally by a (\S+) element, optionally intermixed with one or more (\S+) elements",
                    r"A (\S+) element followed by a (\S+) element",
                    r"One (\S+) element followed by (\S+) content",
                    r"Optionally a (\S+) element, followed by (\S+) content",
                    r"Zero or more (\S+) and (\S+) elements",
                    r"Zero or more (\S+), (\S+), and (\S+) elements",
                    r"Zero or more (\S+), (\S+), (\S+), and (\S+) elements",
                    r"Zero or more (\S+) elements, followed by one (\S+) element, optionally intermixed with (\S+) elements",
                    r"Zero or more (\S+) elements, followed by one (\S+), (\S+), (\S+), (\S+), (\S+), or (\S+) element, followed by zero or more (\S+) elements, optionally intermixed with (\S+) elements",
                    r"Either: (\S+) content",
                    r"Either: one (\S+) element followed by (\S+) content",
                    r"Either: Zero or more groups each consisting of one or more (\S+) elements followed by one or more (\S+) elements, optionally intermixed with (\S+) elements",
                    r"Or: Zero or more (\S+) and (\S+) elements",
                    r"Or: (\S+) content",
                    r"Or: (\S+) content followed by one (\S+) element",
                    r"Or: One or more (\S+) elements, optionally intermixed with (\S+) elements",
                    r"Otherwise: (\S+), but must match requirements described in \S+ below",
                    r"Otherwise: One or more elements of (\S+) content, of which exactly one is a (\S+) element and no more than one is a (\S+) element",
                    r"When scripting is disabled, not in an? \S+ element: (\S+), but there must be no \S+ element descendants",
                    r"When scripting is disabled, in an? \S+ element: in any order, zero or more (\S+) elements, zero or more (\S+) elements, and zero or more (\S+) elements",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    let captures = re.captures(text)?;
                    let contents = captures
                        .iter()
                        .skip(1)
                        .map(|capture| capture.unwrap().as_str())
                        .map(|capture| capture.into())
                        .collect::<Vec<_>>();
                    tracing::trace!(element.name, ?contents, text, "😍");

                    for content in contents {
                        element.contents.insert(content);
                    }

                    Some(())
                },
            ),
            (
                &[
                    r"If there is no \S+ attribute, depends on the value of the \S+ attribute, but must match script content restrictions",
                    r"If there is an? \S+ attribute, the element must be either empty or contain only script documentation that also matches script content restrictions",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    re.captures(text)?;
                    assert_eq!(element.name, "script");
                    tracing::trace!(element.name, text, "😍");

                    Some(())
                },
            ),
            (
                &[r"Otherwise: text that conforms to the requirements given in the prose"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    re.captures(text)?;
                    assert_eq!(element.name, "noscript");
                    tracing::trace!(element.name, text, "😍");

                    Some(())
                },
            ),
            (
                &[r"See prose"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    re.captures(text)?;
                    assert_eq!(element.name, "ruby");
                    tracing::trace!(element.name, text, "😍");

                    Some(())
                },
            ),
        ]
    }
}
