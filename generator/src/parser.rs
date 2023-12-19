use crate::{spec::ParsedElement, Category, CategoryOrElement};
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

type Parse = fn(&Regex, &str, &mut ParsedElement) -> bool;
type Counts = HashMap<(usize, Parse), usize>;

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
                let regex = Regex::new(re).unwrap();

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
        for ((index, parse), count) in counts {
            if parse(&regexes[*index], text, element) {
                *count += 1;
                return;
            }
        }

        tracing::warn!(element.name, section, text, "😓");
    }

    fn category_parsers() -> &'static [(&'static [&'static str], Parse)] {
        &[
            (
                &[
                    r"^(\S+) content\.$",
                    r"^If the element has an? \S+ attribute: (\S+) content\.$",
                    r"^If the element's children include at least one \S+ element: (\S+) content\.$",
                    r"^If the element's children include at least one \S+ group: (\S+) content\.$",
                    r"^If the \S+ attribute is present: (\S+) content\.$",
                    r"^If the \S+ attribute is not in the \S+ state: (\S+) content\.$",
                    r"^If the element is allowed in the body: (\S+) content\.$",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, [category]) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);

                        element
                            .categories
                            .insert(Category::from_str(category).unwrap());

                        tracing::trace!(element.name, category, text, "😍");
                        return true;
                    }

                    false
                },
            ),
            (
                &[r"^(\S+) element\.$"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, [a]) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);

                        let categories = [Category::from_str(a).unwrap()];

                        for category in categories {
                            element.categories.insert(category);
                        }

                        tracing::trace!(element.name, ?categories, text, "😍");
                        return true;
                    }

                    false
                },
            ),
            (
                &[r"^(\S+) (\S+) element\.$"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, [a, b]) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);

                        let categories = [
                            Category::from_str(a).unwrap(),
                            Category::from_str(b).unwrap(),
                        ];

                        for category in categories {
                            element.categories.insert(category);
                        }

                        tracing::trace!(element.name, ?categories, text, "😍");
                        return true;
                    }

                    false
                },
            ),
            (
                &[r"^(\S+) and (\S+) (\S+) element\.$"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, [a, b, c]) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);

                        let categories = [
                            Category::from_str(a).unwrap(),
                            Category::from_str(b).unwrap(),
                            Category::from_str(c).unwrap(),
                        ];

                        for category in categories {
                            element.categories.insert(category);
                        }

                        tracing::trace!(element.name, ?categories, text, "😍");
                        return true;
                    }

                    false
                },
            ),
            (
                &[
                    r"^(\S+), (\S+), (\S+), and (\S+) (\S+) element\.$",
                    r"^If the \S+ attribute is in the \S+ state: (\S+), (\S+), (\S+), and (\S+) (\S+) element\.$",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, [a, b, c, d, e]) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);

                        let categories = [
                            Category::from_str(a).unwrap(),
                            Category::from_str(b).unwrap(),
                            Category::from_str(c).unwrap(),
                            Category::from_str(d).unwrap(),
                            Category::from_str(e).unwrap(),
                        ];

                        for category in categories {
                            element.categories.insert(category);
                        }

                        tracing::trace!(element.name, ?categories, text, "😍");
                        return true;
                    }

                    false
                },
            ),
            (
                &[
                    r"^(\S+), (\S+), (\S+), (\S+), and (\S+) (\S+) element\.$",
                    r"^If the \S+ attribute is not in the \S+ state: (\S+), (\S+), (\S+), (\S+), and (\S+) (\S+) element\.$",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, [a, b, c, d, e, f]) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);

                        let categories = [
                            Category::from_str(a).unwrap(),
                            Category::from_str(b).unwrap(),
                            Category::from_str(c).unwrap(),
                            Category::from_str(d).unwrap(),
                            Category::from_str(e).unwrap(),
                            Category::from_str(f).unwrap(),
                        ];

                        for category in categories {
                            element.categories.insert(category);
                        }

                        tracing::trace!(element.name, ?categories, text, "😍");
                        return true;
                    }

                    false
                },
            ),
            (
                &[r"^None\.$"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, []) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);

                        tracing::trace!(element.name, text, "😍");
                        return true;
                    }

                    false
                },
            ),
        ]
    }

    fn context_parsers() -> &'static [(&'static [&'static str], Parse)] {
        &[
            (
                &[
                    r"^Where (\S+) elements are expected\.$",
                    r"^Where (\S+) content is expected\.$",
                    r"^Where (\S+) content is expected, but only if there is an? \S+ element ancestor\.$",
                    r"^Where (\S+) content is expected, but only if it is a hierarchically correct \S+ element\.$",
                    r"^Where (\S+) content is expected in HTML documents, if there are no ancestor \S+ elements\.$",
                    r"^If the \S+ attribute is present: where (\S+) content is expected\.$",
                    r"^If the element is allowed in the body: where (\S+) content is expected\.$",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, [category]) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);

                        element.contexts.insert(CategoryOrElement::Category(
                            Category::from_str(category).unwrap(),
                        ));

                        tracing::trace!(element.name, category, text, "😍");
                        return true;
                    }

                    false
                },
            ),
            (
                &[
                    r"^Inside (\S+) elements\.$",
                    r"^As a child of an? (\S+) element\.$",
                    r"^As a child of an? (\S+) element that doesn't have an? \S+ attribute\.$",
                    r"^As a child of an? (\S+) element, before the \S+ element\.$",
                    r"^As a child of an? (\S+) element, before any \S+ content\.$",
                    r"^As a child of an? (\S+) element, before any \S+ content or \S+ elements\.$",
                    r"^As a child of an? (\S+) element, after all \S+ elements\.$",
                    r"^As a child of an? (\S+) element, after any \S+, \S+, and \S+ elements, but only if there are no \S+ elements that are children of the \S+ element\.$",
                    r"^As a child of an? (\S+) element, after any \S+, \S+, \S+, \S+, and \S+ elements, but only if there are no other \S+ elements that are children of the \S+ element\.$",
                    r"^As a child of an? (\S+) element, after any \S+ elements and before any \S+, \S+, \S+, and \S+ elements\.$",
                    r"^As a child of an? (\S+) element, after any \S+, and \S+ elements and before any \S+, \S+, and \S+ elements, but only if there are no other \S+ elements that are children of the \S+ element\.$",
                    r"^As a child of an? (\S+) element, either immediately before or immediately after an? \S+ element\.$",
                    r"^As the first child of an? (\S+) element\.$",
                    r"^As the first element in an? (\S+) element\.$",
                    r"^As the first element child of an? (\S+) element\.$",
                    r"^As the second element in an? (\S+) element\.$",
                    r"^As the first or last child of an? (\S+) element\.$",
                    r"^In an? (\S+) element that is a child of an? \S+ element\.$",
                    r"^In an? (\S+) element containing no other \S+ elements\.$",
                    r"^In an? (\S+) element of an HTML document, if there are no ancestor \S+ elements\.$",
                    r"^If the \S+ attribute is present but not in the \S+ declaration state: in an? (\S+) element\.$",
                    r"^If the \S+ attribute is present but not in the \S+ declaration state: in an? (\S+) element that is a child of an? \S+ element\.$",
                    r"^If the \S+ attribute is present, or if the element's \S+ attribute is in the \S+ declaration state: in an? (\S+) element\.$",
                    r"^Before \S+ or \S+ elements inside (\S+) elements\.$",
                    r"^After \S+ or \S+ elements inside (\S+) elements\.$",
                    r"^Before \S+ or \S+ elements inside (\S+) elements that are children of an? \S+ element\.$",
                    r"^After \S+ or \S+ elements inside (\S+) elements that are children of an? \S+ element\.$",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, [category]) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);

                        element
                            .contexts
                            .insert(CategoryOrElement::Element(category.to_string()));

                        tracing::trace!(element.name, category, text, "😍");
                        return true;
                    }

                    false
                },
            ),
            (
                &[
                    r"^As document's document element\.$",
                    r"^Wherever a subdocument fragment is allowed in a compound document\.$",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, []) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);
                        assert_eq!(element.name, "html");

                        tracing::trace!(element.name, text, "😍");
                        return true;
                    }

                    false
                },
            ),
        ]
    }

    fn content_parsers() -> &'static [(&'static [&'static str], Parse)] {
        // Either: one figcaption element followed by flow content.
        // Either: Zero or more groups each consisting of one or more dt elements followed by one or more dd elements, optionally intermixed with script-supporting elements.
        // Or: flow content.
        // Or: flow content followed by one figcaption element.
        // Or: One or more div elements, optionally intermixed with script-supporting elements.
        // Otherwise: One or more elements of metadata content, of which exactly one is a title element and no more than one is a base element.
        // Otherwise: text that conforms to the requirements given in the prose.
        //
        // Flow content, but with no header or footer element descendants.
        // Flow content, but with no form element descendants.
        // Flow content, but with no header or footer element descendants.
        // Phrasing content, but with no descendant labelable elements unless it is the element's labeled control, and no descendant label elements.
        //
        // Optionally a legend element, followed by flow content.
        // When scripting is disabled, in a head element: in any order, zero or more link elements, zero or more style elements, and zero or more meta elements.
        // When scripting is disabled, not in a head element: transparent, but there must be no noscript element descendants.
        // If the document is an iframe srcdoc document or if title information is available from a higher-level protocol: Zero or more elements of metadata content, of which no more than one is a title element and no more than one is a base element.
        // If the element has a label attribute and a value attribute: Nothing.
        // Zero or more p elements, followed by one h1, h2, h3, h4, h5, or h6 element, followed by zero or more p elements, optionally intermixed with script-supporting elements.
        // A head element followed by a body element.
        // See prose.
        &[
            //
            (
                &[
                    r"^(\S+)\.$",
                    r"^(\S+) content\.$",
                    r"^(\S+) \(for clarification, see example\)\.$",
                    r"^(\S+) that is not inter-element whitespace\.$",
                    r"^(\S+) that gives a conformant style sheet\.$",
                    r"^(\S+) content, optionally intermixed with \S+ content\.$",
                    r"^(\S+) content, but there must be no \S+ element descendants\.$",
                    r"^(\S+) content, but with no descendant \S+ elements\.$",
                    r"^(\S+) content, but with no \S+, \S+, \S+ content, or \S+ content descendants\.$",
                    r"^(\S+) content, but with no \S+ content descendants, no \S+ content descendants, and no \S+, \S+, or \S+ element descendants\.$",
                    r"^(\S+) content, but there must be no \S+ content descendant and no descendant with the \S+ attribute specified\.$",
                    r"^(\S+), but there must be no \S+ content descendant, \S+ element descendant, or descendant with the \S+ attribute specified\.$",
                    r"^(\S+), but with no \S+ content descendants except for \S+ elements, \S+ elements with \S+ attributes, \S+ elements, \S+ elements whose \S+ attribute are in the \S+ or \S+ \S+ states, \S+ elements that are \S+, and \S+ elements with a \S+ attribute or a \S+ \S+ greater than 1\.$",
                    r"^Either: (\S+) content\.$",
                    r"^If the \S+ attribute is present: (\S+)\.$",
                    r"^If the element is not a child of an? \S+ element: (\S+) content\.$",
                    r"^If the element has an? \S+ attribute: (\S+) content\.$",
                    r"^If the element has an? \S+ attribute but no \S+ attribute: (\S+)\.$",
                    r"^If the element has no \S+ attribute and is a child of an? \S+ element: (\S+)\.$",
                    r"^If the element has no \S+ attribute and is not a child of an? \S+ element: (\S+) that is not inter-element whitespace\.$",
                    r"^Otherwise: (\S+), but must match requirements described in \S+ below\.$",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, [category]) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);

                        element.contexts.insert(CategoryOrElement::Category(
                            Category::from_str(category).unwrap(),
                        ));

                        tracing::trace!(element.name, category, text, "😍");
                        return true;
                    }

                    false
                },
            ),
            (
                &[r"^Transparent$"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, []) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);

                        let category = "transparent";
                        element.contexts.insert(CategoryOrElement::Category(
                            Category::from_str(category).unwrap(),
                        ));

                        tracing::trace!(element.name, category, text, "😍");
                        return true;
                    }

                    false
                },
            ),
            (
                &[r"^If the span attribute is absent: Zero or more (\S+) and (\S+) elements\.$"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, [a, b]) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);

                        let categories = [
                            CategoryOrElement::Element(a.to_string()),
                            CategoryOrElement::Element(b.to_string()),
                        ];

                        for category in &categories {
                            element.contexts.insert(category.clone());
                        }

                        tracing::trace!(element.name, ?categories, text, "😍");
                        return true;
                    }

                    false
                },
            ),
            (
                &[
                    r"^Zero or more (\S+) and (\S+) elements\.$",
                    r"^Or: Zero or more (\S+) and (\S+) elements\.$",
                    r"^One (\S+) element followed by (\S+) content\.$",
                    r"^If the element has an? \S+ attribute: zero or more (\S+) elements, then (\S+), but with no \S+ element descendants\.$",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, [a, b]) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);

                        let categories = [
                            CategoryOrElement::Element(a.to_string()),
                            CategoryOrElement::Category(Category::from_str(b).unwrap()),
                        ];

                        for category in &categories {
                            element.contexts.insert(category.clone());
                        }

                        tracing::trace!(element.name, ?categories, text, "😍");
                        return true;
                    }

                    false
                },
            ),
            (
                &[
                    r"^Zero or more (\S+), (\S+), and (\S+) elements\.$",
                    r"^Zero or more (\S+) elements, followed by one (\S+) element, optionally intermixed with (\S+) elements\.$",
                    r"^If the element does not have an? \S+ attribute: zero or more (\S+) elements, then zero or more (\S+) elements, then (\S+), but with no \S+ element descendants\.$",
                    r"^If the element is a child of an? \S+ element: one or more (\S+) elements followed by one or more (\S+) elements, optionally intermixed with (\S+) elements\.$",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, [a, b, c]) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);

                        let categories = [
                            CategoryOrElement::Element(a.to_string()),
                            CategoryOrElement::Element(b.to_string()),
                            CategoryOrElement::Category(Category::from_str(c).unwrap()),
                        ];

                        for category in &categories {
                            element.contexts.insert(category.clone());
                        }

                        tracing::trace!(element.name, ?categories, text, "😍");
                        return true;
                    }

                    false
                },
            ),
            (
                &[r"^Zero or more (\S+), (\S+), (\S+), and (\S+) elements\.$"],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, [a, b, c, d]) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);

                        let categories = [
                            CategoryOrElement::Element(a.to_string()),
                            CategoryOrElement::Element(b.to_string()),
                            CategoryOrElement::Element(c.to_string()),
                            CategoryOrElement::Category(Category::from_str(d).unwrap()),
                        ];

                        for category in &categories {
                            element.contexts.insert(category.clone());
                        }

                        tracing::trace!(element.name, ?categories, text, "😍");
                        return true;
                    }

                    false
                },
            ),
            (
                &[
                    r"^In this order: optionally a (\S+) element, followed by zero or more (\S+) elements, followed optionally by an? (\S+) element, followed by either zero or more (\S+) elements or one or more (\S+) elements, followed optionally by a (\S+) element, optionally intermixed with one or more (\S+) elements\.$",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, [a, b, c, d, e, f, g]) in re.captures_iter(text).map(|c| c.extract())
                    {
                        assert_eq!(text, sub);

                        let categories = [
                            CategoryOrElement::Element(a.to_string()),
                            CategoryOrElement::Element(b.to_string()),
                            CategoryOrElement::Element(c.to_string()),
                            CategoryOrElement::Element(d.to_string()),
                            CategoryOrElement::Element(e.to_string()),
                            CategoryOrElement::Element(f.to_string()),
                            CategoryOrElement::Category(Category::from_str(g).unwrap()),
                        ];

                        for category in &categories {
                            element.contexts.insert(category.clone());
                        }

                        tracing::trace!(element.name, ?categories, text, "😍");
                        return true;
                    }

                    false
                },
            ),
            (
                &[
                    r"^If there is no src attribute, depends on the value of the type attribute, but must match script content restrictions\.$",
                    r"^If there is a src attribute, the element must be either empty or contain only script documentation that also matches script content restrictions\.$",
                ],
                |re: &Regex, text: &str, element: &mut ParsedElement| {
                    for (sub, []) in re.captures_iter(text).map(|c| c.extract()) {
                        assert_eq!(text, sub);
                        assert_eq!(element.name, "script");

                        tracing::trace!(element.name, text, "😍");
                        return true;
                    }

                    false
                },
            ),
        ]
    }
}
