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

// NOTE
// Quick links:
// - https://html.spec.whatwg.org/multipage/syntax.html#elements-2
// - https://html.spec.whatwg.org/multipage/syntax.html#attributes-2
// - https://html.spec.whatwg.org/multipage/dom.html#global-attributes
// - https://html.spec.whatwg.org/multipage/dom.html#content-models
// - https://html.spec.whatwg.org/multipage/dom.html#elements-in-the-dom
// - https://html.spec.whatwg.org/multipage/custom-elements.html#custom-elements

// TODO
// - Void elements ("Tag omission in text/html" is non-normative)
// - Attributes run-time validation (debug or release)
// - Autonomous custom element
// - HTMX support

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
        let s = s.to_case(Case::Snake);
        // Rust keywords
        let s = match s.as_str() {
            "as" => "as_",
            "async" => "async_",
            "for" => "for_",
            "id" => "id_",
            "loop" => "loop_",
            "type" => "type_",
            s => s,
        };
        ident!("{s}")
    }

    pub fn pascal(s: &str) -> Ident {
        let s = s.to_case(Case::Pascal);
        ident!("{s}")
    }

    pub fn attribute(s: &str) -> Ident {
        match snake(s).to_string().as_str() {
            "data" => ident!("data_"), // Conflicts with `data-*`
            s => ident!("{s}"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Element {
    name: String,
    attributes: BTreeMap<String, AttributeType>,
    children: BTreeSet<String>,
    text: bool,
    end_tag: bool,
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
    I64,
    U64,
    F32,
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
            AttributeType::I64 => quote!(i64),
            AttributeType::U64 => quote!(u64),
            AttributeType::F32 => quote!(f32),
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
        .inspect(|attribute| assert!(!attribute.starts_with("on")))
        .chain(list(HANDLERS))
        .collect()
}

fn parse_elements(html: &Html) -> Vec<ParsedElement> {
    let mut parser = Parser::new();
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
        .flat_map(|(names, definition)| names.into_iter().map(move |name| (name, definition)))
        .map(|(name, definition)| {
            let mut section = None;
            let mut element = ParsedElement {
                name: name.to_owned(),
                categories: BTreeSet::new(),
                contents: BTreeSet::new(),
                end_tag: true,
                attributes: BTreeSet::new(),
                interface: String::from("__INVALID__"),
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
            let resolved = idl.resolve(&element.interface);

            Element {
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
                text: element.contents.contains("text") || element.contents.contains("phrasing"),
                end_tag: element.end_tag,
            }
        })
        .collect()
}

fn generate_files(elements: Vec<Element>) {
    const MDN: &'static str = "https://developer.mozilla.org/en-US/docs/Web/HTML/Element";

    fn child_name(s: &str) -> Ident {
        let s = s.to_case(Case::Pascal);
        ident!("{s}Child")
    }

    fn builder_name(s: &str) -> Ident {
        let s = s.to_case(Case::Pascal);
        ident!("{s}Builder")
    }

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

    let elements = {
        let mut elements = elements;
        elements.iter_mut().for_each(|element| {
            // We handle those attributes manually.
            assert!(element.attributes.remove("id").is_some());
            assert!(element.attributes.remove("class").is_some());
        });
        elements
    };

    let sort_attributes = |attributes: &BTreeMap<String, AttributeType>| {
        let mut attributes = attributes
            .iter()
            .map(|(name, ty)| (name.clone(), *ty))
            .collect::<Vec<_>>();
        attributes.sort_by(
            |(a, _), (b, _)| match (a.starts_with("on"), b.starts_with("on")) {
                (true, true) => a.cmp(&b),
                (true, false) => std::cmp::Ordering::Greater,
                (false, true) => std::cmp::Ordering::Less,
                (false, false) => a.cmp(&b),
            },
        );
        attributes
    };

    write(None, "mod", {
        let names = elements
            .iter()
            .map(|element| text::flat(&element.name))
            .map(|name| ident!("{name}"))
            .map(|name| quote! { mod #name; pub use #name::*; });
        let children = elements
            .iter()
            .filter(|element| !element.children.is_empty())
            .map(|element| text::flat(&element.name))
            .map(|name| ident!("{name}"))
            .map(|name| quote! { mod #name; pub use #name::*; });
        let builders = names.clone();

        quote! {
            pub use elements::*;
            pub mod elements { #(#names)* }
            pub mod children { #(#children)* }
            pub mod builders { #(#builders)* }
        }
    });

    // Elements.
    elements.iter().for_each(|element| {
        let name = text::pascal(&element.name);
        let child = child_name(&element.name);
        let builder = builder_name(&element.name);
        let child_fn = (!element.children.is_empty()).then_some(quote! {
            pub fn child<T: Into<children::#child>>(child: T) -> builders::#builder {
                <builders::#builder as Default>::default().child(child)
            }
        });
        let attributes = sort_attributes(&element.attributes)
            .into_iter()
            .map(|(name, ty)| (text::attribute(&name), ty))
            .map(|(name, ty)| quote! { pub #name: std::option::Option<#ty>, });
        let children = (!element.children.is_empty()).then_some(quote! {
            pub children: Vec<children::#child>,
        });
        let attribute_builders = sort_attributes(&element.attributes)
            .into_iter()
            .map(|(name, ty)| (text::attribute(&name), ty.to_token_stream()))
            .map(|(name, ty)| {
                quote! {
                    pub fn #name<T: Into<#ty>>(#name: T) -> builders::#builder {
                        <builders::#builder as Default>::default().#name(#name)
                    }
                }
            });
        let child_builders = element
            .children
            .iter()
            .map(|name| (text::snake(name), text::pascal(name)))
            .map(|(snake, pascal)| {
                quote! {
                    #[allow(non_snake_case)]
                    pub fn #pascal<T: Into<#pascal>>(#snake: T) -> builders::#builder {
                        <builders::#builder as Default>::default().#pascal(#snake)
                    }
                }
            });
        let description = format!(" The `<{}>` element.", element.name);
        let link = format!(" [`MDN`]({MDN}/{})", element.name);

        write(
            "elements",
            text::flat(&element.name),
            quote! {
                use crate::*;

                #[doc = #description]
                ///
                #[doc = #link]
                #[derive(Clone, Default, Debug)]
                pub struct #name {
                    pub id: StdOption<CowStr>,
                    pub classes: HashSet<CowStr>,
                    pub data: HashMap<CowStr, CowStr>,
                    #(#attributes)*
                    #children
                }

                impl #name {
                    pub fn id<T: Into<CowStr>>(id: T) -> builders::#builder {
                        <builders::#builder as Default>::default().id(id)
                    }

                    pub fn class<T: Into<CowStr>>(class: T) -> builders::#builder {
                        <builders::#builder as Default>::default().class(class)
                    }

                    pub fn data<K: Into<CowStr>, V: Into<CowStr>>(key: K, value: V) -> builders::#builder {
                        <builders::#builder as Default>::default().data(key, value)
                    }

                    #child_fn
                    #(#attribute_builders)*
                    #(#child_builders)*
                }

                impl From<builders::#builder> for #name {
                    fn from(builder: builders::#builder) -> Self {
                        builder.element
                    }
                }
            },
        );
    });

    // Children.
    elements.iter().for_each(|element| {
        if element.children.is_empty() {
            return;
        }

        let child = child_name(&element.name);
        let name = element.children.iter().map(|name| text::pascal(name));
        let text = element.text.then_some(quote! { Text(CowStr) });
        let str_froms = element.text.then_some(quote! {
            impl From<&'static str> for #child {
                fn from(s: &'static str) -> Self {
                    #child::Text(s.into())
                }
            }

            impl From<String> for #child {
                fn from(s: String) -> Self {
                    #child::Text(s.into())
                }
            }
        });
        let froms = element
            .children
            .iter()
            .map(|name| (text::pascal(name), builder_name(name)))
            .map(|(name, builder)| {
                quote! {
                    impl From<#name> for #child {
                        fn from(child: #name) -> Self {
                            #child::#name(child)
                        }
                    }

                    impl From<builders::#builder> for #child {
                        fn from(builder: builders::#builder) -> Self {
                            #child::#name(builder.build())
                        }
                    }
                }
            });
        let description = format!(" The `<{}>` element's children.", element.name);

        write(
            "children",
            text::flat(&element.name),
            quote! {
                use crate::*;

                #[doc = #description]
                #[derive(Clone, Debug)]
                pub enum #child {
                    #(#name(#name),)*
                    #text
                }

                #(#froms)*
                #str_froms
            },
        );
    });

    // Builders.
    elements.iter().for_each(|element| {
        let name = text::pascal(&element.name);
        let child = child_name(&element.name);
        let builder = builder_name(&element.name);
        let child_fn = (!element.children.is_empty()).then_some(quote! {
            pub fn child<T: Into<children::#child>>(mut self, child: T) -> Self {
                self.element.children.push(child.into());
                self
            }
        });
        let attributes = sort_attributes(&element.attributes)
            .into_iter()
            .map(|(name, ty)| (text::attribute(&name), ty.to_token_stream()))
            .map(|(name, ty)| {
                quote! {
                    pub fn #name<T: Into<#ty>>(mut self, #name: T) -> Self {
                        self.element.#name = Some(#name.into());
                        self
                    }
                }
            });
        let children = element
            .children
            .iter()
            .map(|name| (text::snake(name), text::pascal(name)))
            .map(|(snake, pascal)| {
                quote! {
                    #[allow(non_snake_case)]
                    pub fn #pascal<T: Into<#pascal>>(mut self, #snake: T) -> Self {
                        self.element.children.push(children::#child::#pascal(#snake.into()));
                        self
                    }
                }
            });
        let description = format!(" The `<{}>` element's builder.", element.name);

        write(
            "builders",
            text::flat(&element.name),
            quote! {
                use crate::*;

                #[doc = #description]
                #[derive(Clone, Default, Debug)]
                pub struct #builder {
                    pub element: #name,
                }

                impl #builder {
                    pub fn build(self) -> #name {
                        self.element
                    }

                    pub fn id<T: Into<CowStr>>(mut self, id: T) -> Self {
                        self.element.id = Some(id.into());
                        self
                    }

                    pub fn class<T: Into<CowStr>>(mut self, class: T) -> Self {
                        self.element.classes.insert(class.into());
                        self
                    }

                    pub fn data<K: Into<CowStr>, V: Into<CowStr>>(mut self, key: K, value: V) -> Self {
                        self.element.data.insert(key.into(), value.into());
                        self
                    }

                    #child_fn
                    #(#attributes)*
                    #(#children)*
                }

                impl From<#name> for #builder {
                    fn from(element: #name) -> Self {
                        Self { element }
                    }
                }
            },
        );
    });
}
