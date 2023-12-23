macro_rules! selector {
    ($($arg:tt)*) => {{
        let selector = format!($($arg)*);
        ::scraper::Selector::parse(&selector)
            .unwrap_or_else(|err| panic!("`{selector}`: {err:?}"))
    }};
}

macro_rules! ident {
    ($($arg:tt)*) => {
        ::quote::format_ident!($($arg)*)
    };
}

mod parsers {
    pub mod element;
    pub mod idl;
}
mod generators {
    pub mod builder;
    pub mod child;
    pub mod element;
}
mod renames;

use convert_case::{Case, Casing};
use parsers::{
    element::{ParsedElement, Parser},
    idl::ParsedIdl,
};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use regex::Regex;
use scraper::{ElementRef, Html};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

// Quick links:
// - https://html.spec.whatwg.org/multipage/syntax.html#elements-2
// - https://html.spec.whatwg.org/multipage/syntax.html#attributes-2
// - https://html.spec.whatwg.org/multipage/dom.html#global-attributes
// - https://html.spec.whatwg.org/multipage/dom.html#content-models
// - https://html.spec.whatwg.org/multipage/dom.html#elements-in-the-dom
// - https://html.spec.whatwg.org/multipage/custom-elements.html#custom-elements
// - https://html.spec.whatwg.org/entities.json

// TODO
// - Void elements ("Tag omission in text/html" is non-normative)
//
// Ideas:
// - enumerated attribute values?
// - typed link/input types?
// - character references?
// - htmx?
// - css?
// - aria?
// - svg?
// - mathml?

const MDN: &'static str = "https://developer.mozilla.org/en-US/docs/Web/HTML/Element";
const URL: &'static str = "https://html.spec.whatwg.org";
const SPEC: &'static str = "spec.html";
const OUTPUT: &'static str = "src/generated";

mod text {
    use super::*;

    pub fn simplify(text: &str) -> String {
        Regex::new(r"\s+") // Collapse whitespace
            .unwrap()
            .replace_all(&text, " ")
            .trim()
            .trim_end_matches('.')
            .to_lowercase()
    }

    pub fn flat(s: &str) -> String {
        s.to_case(Case::Flat)
    }

    pub fn snake(s: &str) -> Ident {
        ident!(
            "{}",
            match s.to_case(Case::Snake).as_str() {
                // Rust keywords
                "as" => "as_",
                "async" => "async_",
                "for" => "for_",
                "id" => "id_",
                "loop" => "loop_",
                "type" => "type_",
                s => s,
            }
        )
    }

    pub fn pascal(s: &str) -> Ident {
        ident!("{}", s.to_case(Case::Pascal))
    }

    pub fn attribute(s: &str) -> Ident {
        match snake(s).to_string().as_str() {
            // Conflicts with `data-*`
            "data" => ident!("data_"),
            s => ident!("{s}"),
        }
    }

    pub fn child(s: &str) -> Ident {
        ident!("{}Child", s.to_case(Case::Pascal))
    }

    pub fn builder(s: &str) -> Ident {
        ident!("{}Builder", s.to_case(Case::Pascal))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Element {
    tag: String,
    name: String,
    attributes: Vec<(String, AttributeType)>,
    children: BTreeSet<String>,
    text: bool,
    end_tag: bool,
    custom: bool,
}

impl Element {
    fn has_children(&self) -> bool {
        self.text || !self.children.is_empty()
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug)]
enum AttributeType {
    #[default]
    String,
    Bool,
    I16,
    U16,
    I32,
    U32,
    F32,
    I64,
    U64,
    F64,
    BoolOrF64OrString,
}

impl AttributeType {
    fn collapse(mut types: Vec<Self>) -> Option<Self> {
        if types.is_empty() {
            return None;
        }

        types.sort();

        let mut bool_or_f64_or_string = vec![
            AttributeType::Bool,
            AttributeType::F64,
            AttributeType::String,
        ];
        bool_or_f64_or_string.sort();

        Some(match types {
            types if types == [AttributeType::String] => AttributeType::String,
            types if types == bool_or_f64_or_string => AttributeType::BoolOrF64OrString,
            _ => unreachable!(),
        })
    }
}

impl ToTokens for AttributeType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            AttributeType::Bool => quote!(bool),
            AttributeType::I16 => quote!(i16),
            AttributeType::U16 => quote!(u16),
            AttributeType::I32 => quote!(i32),
            AttributeType::U32 => quote!(u32),
            AttributeType::F32 => quote!(f32),
            AttributeType::I64 => quote!(i64),
            AttributeType::U64 => quote!(u64),
            AttributeType::F64 => quote!(f64),
            AttributeType::String => quote!(CowStr),
            AttributeType::BoolOrF64OrString => quote!(BoolOrF64OrString),
        }
        .to_tokens(tokens)
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    async fn fetch() {
        tracing::info!("🚀 FETCHING");

        let request = reqwest::get(URL).await.unwrap();
        let response = request.text().await.unwrap();

        std::fs::write(format!("{SPEC}"), response).unwrap();
    }

    fn generate() {
        tracing::info!("🚀 GENERATING");

        let html = Html::parse_document(&std::fs::read_to_string(format!("{SPEC}")).unwrap());
        assert!(html.errors.is_empty());

        generate_files(resolve(
            parse_global_attributes(&html),
            parse_idl(&html),
            parse_elements(&html),
        ));
    }

    match std::env::args().nth(1) {
        None => {
            fetch().await;
            generate();
        }
        Some(arg) if arg == "fetch" => {
            fetch().await;
        }
        Some(arg) if arg == "generate" => {
            generate();
        }
        command => {
            tracing::error!(command, "🚨 Unknown command");
            std::process::exit(1);
        }
    }
}

fn parse_idl(html: &Html) -> ParsedIdl {
    let idl = html
        .select(&selector!("code.idl"))
        .map(|idl| {
            idl.text()
                .collect::<String>()
                // `weedle` chokes on this:
                .trim_start_matches("[Exposed=*]")
                .to_owned()
        })
        .collect::<String>();

    ParsedIdl::parse(&idl).check()
}

fn parse_global_attributes(html: &Html) -> Vec<String> {
    const ATTRIBUTES: &str = concat!(
        "the following attributes are common to ",
        "and may be specified on all html elements ",
        "(even those not defined in this specification):",
    );
    const HANDLERS: &str = concat!(
        "the following event handler content attributes ",
        "may be specified on any html element:",
    );

    let list = |needle: &str| {
        html.select(&selector!("body > p"))
            .map(|p| (p, p.text().collect::<String>()))
            .find(|(_, text)| text::simplify(text) == needle)
            .map(|(p, _)| p)
            .unwrap()
            .next_siblings()
            .flat_map(ElementRef::wrap)
            .find(|element| selector!("ul.brief").matches(element))
            .unwrap()
            .select(&selector!("a"))
            .map(|a| a.text().collect::<String>())
            .collect::<Vec<_>>()
    };
    let additional = {
        let selector = selector!("body > p");
        let ps = html
            .select(&selector)
            .map(|p| text::simplify(&p.text().collect::<String>()));
        let re = regex::Regex::new(
            r"^the (\S+), (\S+), and (\S+) attributes may be specified on all html elements$",
        )
        .unwrap();

        let mut additional = None;
        for text in ps {
            if let Some(captures) = re.captures(&text) {
                additional = Some([
                    captures[1].to_owned(),
                    captures[2].to_owned(),
                    captures[3].to_owned(),
                ]);
                break;
            }
        }

        additional.unwrap()
    };

    list(ATTRIBUTES)
        .into_iter()
        .chain(additional)
        .chain(list(HANDLERS).into_iter().map(|handler| {
            assert!(handler.starts_with("on"));
            format!("on_{}", handler.trim_start_matches("on"))
        }))
        .collect()
}

fn parse_elements(html: &Html) -> Vec<ParsedElement> {
    let mut parser = Parser::new();
    let custom = html
        .select(&selector!("body > p"))
        .map(|p| (p, p.text().collect::<String>()))
        .find(|(_, text)| {
            text::simplify(text)
                == "autonomous custom elements have the following element definition:"
        })
        .map(|(p, _)| p)
        .unwrap()
        .next_siblings()
        .flat_map(ElementRef::wrap)
        .find(|element| selector!("dl.element").matches(element))
        .map(|definition| (true, "custom".to_owned(), definition));
    let elements = html
        .select(&(selector!("h4")))
        .filter_map(|h4| {
            let secno = h4.select(&selector!("span.secno")).next()?.inner_html();
            if !secno.starts_with("4.") {
                return None;
            }

            Some((
                h4.select(&selector!("dfn[data-dfn-type=\"element\"] > code"))
                    .map(|code| code.inner_html())
                    .collect::<Vec<_>>(),
                h4.next_siblings()
                    .flat_map(ElementRef::wrap)
                    .find(|element| selector!("dl.element").matches(element))?,
            ))
        })
        .flat_map(|(names, definition)| {
            assert!(!names.contains(&"custom".to_string()));
            names.into_iter().map(move |name| (false, name, definition))
        })
        .chain(custom)
        .map(|(custom, name, definition)| {
            let mut section = None;
            let mut element = ParsedElement {
                name: name.to_owned(),
                categories: BTreeSet::new(),
                contents: BTreeSet::new(),
                end_tag: true,
                attributes: BTreeSet::new(),
                interface: None,
                custom,
            };

            for d in definition.select(&selector!("dt, dd")) {
                let text = d.text().collect::<String>();

                match d.value().name() {
                    "dt" => {
                        let str = d.select(&selector!("a")).next().unwrap().inner_html();
                        section = Some(str.to_lowercase().replace(' ', ""));
                    }
                    "dd" => match section.as_ref().unwrap().as_str() {
                        "categories" => {
                            parser.category(&text, &mut element);
                        }
                        "contextsinwhichthiselementcanbeused" => {}
                        "contentmodel" => {
                            parser.content(&text, &mut element);
                        }
                        "tagomissionintext/html" => {
                            parser.tag_omission(&text, &mut element);
                        }
                        "contentattributes" => {
                            parser.attribute(&text, &mut element);
                        }
                        "accessibilityconsiderations" => {}
                        "dominterface" => {
                            let text = d
                                .select(&selector!("pre"))
                                .next()
                                .map(|pre| pre.text().collect::<String>())
                                .unwrap_or(text);
                            parser.idl(&text, &mut element);
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }

            element
        })
        .collect();

    parser.errors();
    elements
}

fn resolve(
    global_attributes: Vec<String>,
    idl: ParsedIdl,
    elements: Vec<ParsedElement>,
) -> Vec<Element> {
    let names = elements
        .iter()
        .map(|element| element.name.clone())
        .collect::<BTreeSet<_>>();
    let categories = elements.iter().fold(
        BTreeMap::<String, BTreeSet<String>>::new(),
        |mut categories, element| {
            for category in &element.categories {
                categories
                    .entry(category.clone())
                    .or_default()
                    .insert(element.name.clone());
            }
            categories
        },
    );

    elements
        .into_iter()
        .map(|element| {
            let resolved = idl.resolve(&element.interface.unwrap());

            Element {
                tag: element.name.clone(),
                name: element.name.clone(),
                attributes: element
                    .attributes
                    .into_iter()
                    .chain(global_attributes.clone())
                    .map(|name| {
                        (
                            name.to_owned(),
                            resolved
                                .get(&text::flat(&name))
                                .map(|(_, ty)| *ty)
                                .unwrap_or_default(),
                        )
                    })
                    .collect(),
                children: element
                    .contents
                    .iter()
                    .fold(BTreeSet::new(), |mut children, name| {
                        match name.as_str() {
                            // https://html.spec.whatwg.org/multipage/dom.html#the-nothing-content-model
                            // > When an element's content model is nothing,
                            // > the element must contain no Text nodes and no element nodes.
                            // NOTE:
                            // Elements might be in "nothing" under certain conditions,
                            // but have children under other conditions.
                            "nothing" => return children,

                            // https://html.spec.whatwg.org/multipage/dom.html#transparent-content-models
                            // > The content model of a transparent element is derived
                            // > from the content model of its parent element: [...].
                            // > When a transparent element has no parent, then its content model
                            // > must instead be treated as accepting any flow content.
                            // NOTE:
                            // Of course this is impossible to implement in a static way,
                            // and we don't want to restrict to flow content only.
                            "transparent" => return names.clone(),

                            // NOTE:
                            // We handle "text" on `Element::text` directly.
                            "text" => return children,

                            _ => {
                                if let Some(elements) = categories.get(name) {
                                    children.extend(elements.clone());
                                    return children;
                                }

                                if names.contains(name) {
                                    children.insert(name.clone());
                                    return children;
                                }

                                unreachable!();
                            }
                        }
                    }),
                // https://html.spec.whatwg.org/multipage/dom.html#text-content
                // > Text is sometimes used as a content model on its own,
                // > but is also phrasing content.
                // https://html.spec.whatwg.org/multipage/dom.html#flow-content
                // https://html.spec.whatwg.org/multipage/dom.html#phrasing-content
                // https://html.spec.whatwg.org/multipage/dom.html#palpable-content
                // > [...], text
                text: element.contents.contains("text")
                    || element.contents.contains("flow")
                    || element.contents.contains("phrasing")
                    || element.contents.contains("palpable"),
                end_tag: element.end_tag,
                custom: element.custom,
            }
        })
        .collect()
}

fn generate_files(elements: Vec<Element>) {
    fn write<'a>(dir: impl Into<Option<&'a str>>, file: impl AsRef<str>, tokens: TokenStream) {
        let out = OUTPUT;
        let file = file.as_ref();
        let (dir, file) = if let Some(dir) = dir.into() {
            (format!("{out}/{dir}"), format!("{out}/{dir}/{file}.rs"))
        } else {
            (format!("{out}"), format!("{out}/{file}.rs"))
        };
        let content = format!(
            "// 🤖 This file is generated!\n\n{}",
            prettyplease::unparse(&syn::parse_file(&tokens.to_string()).unwrap()),
        );

        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(file, content).unwrap();
    }

    let renames = renames::elements();
    let elements = elements
        .into_iter()
        .map(|mut element| {
            let attributes = element.attributes;

            // We handle those attributes manually
            assert!(attributes.iter().find(|(n, _)| n == "id").is_some());
            assert!(attributes.iter().find(|(n, _)| n == "class").is_some());
            element.attributes = attributes
                .into_iter()
                .filter(|(name, _)| name != "id" && name != "class")
                .collect();

            // Event handlers last
            element.attributes.sort_by(|(a, _), (b, _)| {
                match (a.starts_with("on_"), b.starts_with("on_")) {
                    (true, true) => a.cmp(&b),
                    (true, false) => std::cmp::Ordering::Greater,
                    (false, true) => std::cmp::Ordering::Less,
                    (false, false) => a.cmp(&b),
                }
            });

            // Rename elements
            element.name = renames[&element.name].to_owned();
            element.children = element
                .children
                .into_iter()
                .map(|name| renames[&name].to_owned())
                .collect();

            element
        })
        .collect::<Vec<_>>();

    write(None, "mod", {
        let tags = elements
            .iter()
            .map(|element| text::flat(&element.tag))
            .map(|tag| ident!("{tag}"))
            .map(|tag| quote! { mod #tag; pub use #tag::*; });
        let children = elements
            .iter()
            .filter(|element| element.has_children())
            .map(|element| text::flat(&element.tag))
            .map(|tag| ident!("{tag}"))
            .map(|tag| quote! { mod #tag; pub use #tag::*; });
        let builders = tags.clone();

        quote! {
            pub use elements::*;
            pub mod elements { #(#tags)* }
            pub mod children { #(#children)* }
            pub mod builders { #(#builders)* }
        }
    });

    for element in elements {
        write(
            "elements",
            text::flat(&element.tag),
            generators::element::generate(&element),
        );

        if element.has_children() {
            write(
                "children",
                text::flat(&element.tag),
                generators::child::generate(&element),
            );
        }

        write(
            "builders",
            text::flat(&element.tag),
            generators::builder::generate(&element),
        );
    }
}
