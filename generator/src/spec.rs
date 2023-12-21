use crate::{
    parser::{ParsedElement, ParsedIdl, Parser},
    AttributeType,
};
use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use scraper::{ElementRef, Html};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{create_dir_all, read_to_string, write},
};

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

// TODO
// - Void elements: https://html.spec.whatwg.org/multipage/syntax.html#elements-2
// - global attributes and event handlers: https://html.spec.whatwg.org/multipage/dom.html#global-attributes
// - attributes: https://html.spec.whatwg.org/multipage/syntax.html#attributes-2
// - HTMLElement IDL: https://html.spec.whatwg.org/multipage/dom.html#elements-in-the-dom
// - Categories: https://html.spec.whatwg.org/multipage/dom.html#content-models
// - Custom elements: https://html.spec.whatwg.org/multipage/custom-elements.html#custom-elements

#[derive(Serialize, Deserialize, Debug)]
pub struct Element {
    pub name: String,
    pub global_attributes: bool,
    pub attributes: BTreeMap</* name: */ String, (AttributeType, /* description: */ String)>,
    pub children: BTreeSet<String>,
    pub text: bool,
    pub end_tag: bool,
}

pub enum Spec {}

impl Spec {
    pub const URL: &'static str = "https://html.spec.whatwg.org";
    pub const RESOURCES: &'static str = "resources";
    pub const FETCHED: &'static str = "_specs.html";
    pub const OUTPUT: &'static str = "src/generated";

    pub async fn fetch() {
        tracing::info!("FETCHING");

        let request = reqwest::get(Self::URL).await.unwrap();
        let response = request.text().await.unwrap();

        Self::write_fetched(&response);
    }

    pub async fn prepare() {
        tracing::info!("PREPARING");

        let html = Self::read_fetched();
        let idl = Self::parse_idl(&html).await;
        let elements = Self::parse_elements(&html).await;
        let elements = Self::resolve(idl, elements);

        Self::generate(elements);
    }
}

/// Private.
impl Spec {
    fn fetched_file() -> String {
        format!("{}/{}", Self::RESOURCES, Self::FETCHED)
    }

    pub fn write_fetched(specs: &str) {
        create_dir_all(Self::RESOURCES).unwrap();
        write(Self::fetched_file(), specs).unwrap();
    }

    fn read_fetched() -> Html {
        let html = Html::parse_document(&read_to_string(Self::fetched_file()).unwrap());
        assert!(html.errors.is_empty());
        html
    }

    async fn parse_idl(html: &Html) -> ParsedIdl {
        let idl = html
            .select(&selector!("code.idl"))
            .map(|idl| {
                idl.text()
                    .collect::<String>()
                    .trim_start_matches("[Exposed=*]") // `weedle` chokes on this
                    .to_owned()
            })
            .collect::<String>();

        ParsedIdl::parse(&idl)
    }

    async fn parse_elements(html: &Html) -> Vec<ParsedElement> {
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
                    contexts: BTreeSet::new(),
                    contents: BTreeSet::new(),
                    end_tag: true,
                    global_attributes: true,
                    attributes: BTreeMap::new(),
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
                            "contextsinwhichthiselementcanbeused" => {
                                parser.context(&text, &mut element);
                            }
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

    fn resolve(_idl: ParsedIdl, elements: Vec<ParsedElement>) -> Vec<Element> {
        let names = elements
            .iter()
            .map(|element| element.name.clone())
            .collect::<BTreeSet<_>>();
        let categories = elements.iter().fold(
            BTreeMap::<String, BTreeSet<String>>::new(),
            |mut categories, element| {
                for category in &element.categories {
                    if category == "text" || category == "transparent" || category == "nothing" {
                        dbg!(&category);
                    }
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
            .map(|element| Element {
                name: element.name.clone(),
                global_attributes: element.global_attributes,
                attributes: element
                    .attributes
                    .into_iter()
                    .map(|(name, description)| (name, (AttributeType::String, description)))
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
            })
            .collect()
    }

    fn generate(elements: Vec<Element>) {
        fn flat_case(s: &str) -> String {
            s.to_case(Case::Flat)
        }

        fn snake_case(s: &str) -> Ident {
            let s = s.to_case(Case::Snake);
            let s = match s.as_str() {
                "as" => "as_",
                "type" => "type_",
                "loop" => "loop_",
                "for" => "for_",
                "async" => "async_",
                s => s,
            };
            ident!("{s}")
        }

        fn pascal_case(s: &str) -> Ident {
            let s = s.to_case(Case::Pascal);
            ident!("{s}")
        }

        fn child_name(s: &str) -> Ident {
            let s = s.to_case(Case::Pascal);
            ident!("{s}Child")
        }

        fn builder_name(s: &str) -> Ident {
            let s = s.to_case(Case::Pascal);
            ident!("{s}Builder")
        }

        fn write<'a>(dir: impl Into<Option<&'a str>>, file: impl AsRef<str>, tokens: TokenStream) {
            let out = Spec::OUTPUT;
            let file = file.as_ref();
            let (dir, file) = if let Some(dir) = dir.into() {
                (format!("{out}/{dir}"), format!("{out}/{dir}/{file}.rs"))
            } else {
                (format!("{out}"), format!("{out}/{file}.rs"))
            };
            let content = prettyplease::unparse(&syn::parse_file(&tokens.to_string()).unwrap());

            create_dir_all(dir).unwrap();
            std::fs::write(file, content).unwrap();
        }

        write(None, "mod", {
            let names = elements
                .iter()
                .map(|element| flat_case(&element.name))
                .map(|name| ident!("{name}"))
                .map(|name| quote! { mod #name; pub use #name::*; });
            let names2 = names.clone();
            let names3 = names.clone();

            quote! {
                pub mod elements { #(#names)* }
                pub mod children { #(#names2)* }
                pub mod builders { #(#names3)* }

                pub use elements::*;
                use children::*;
                use builders::*;
            }
        });

        elements.iter().for_each(|element| {
            let name = pascal_case(&element.name);
            let child = child_name(&element.name);
            let builder = builder_name(&element.name);
            let attributes = element
                .attributes
                .iter()
                .map(|(name, (ty, description))| (snake_case(name), (ty, description)))
                .map(|(name, (ty, description))| {
                    quote! {
                        #[doc = #description]
                        pub #name: std::option::Option<#ty>,
                    }
                });
            let description = format!(" The `<{}>` element.", element.name);

            write(
                "elements",
                flat_case(&element.name),
                quote! {
                    use crate::generated::*;

                    #[doc = #description]
                    #[derive(Clone, Default, Debug)]
                    pub struct #name {
                        #(#attributes)*
                        pub children: Vec<#child>,
                    }

                    #[doc = #description]
                    #[allow(non_snake_case)]
                    pub fn #name() -> #builder {
                        #builder {
                            element: #name::default(),
                        }
                    }

                    impl From<#builder> for #name {
                        fn from(builder: #builder) -> Self {
                            builder.element
                        }
                    }
                },
            );
        });

        elements.iter().for_each(|element| {
            let child = child_name(&element.name);
            let name = element.children.iter().map(|name| pascal_case(name));
            let description = format!(" The `<{}>` element's children.", element.name);

            write(
                "children",
                flat_case(&element.name),
                quote! {
                    use crate::generated::*;

                    #[doc = #description]
                    #[derive(Clone, Debug)]
                    pub enum #child {
                        #(#name(#name)),*
                    }
                },
            );
        });

        elements.iter().for_each(|element| {
            let name = pascal_case(&element.name);
            let child = child_name(&element.name);
            let builder = builder_name(&element.name);
            let attributes = element
                .attributes
                .iter()
                .map(|(name, (ty, _))| (snake_case(name), ty.to_token_stream()))
                .map(|(name, ty)| {
                    quote! {
                        pub fn #name(mut self, #name: #ty) -> Self {
                            self.element.#name = Some(#name);
                            self
                        }
                    }
                });
            let children = element
                .children
                .iter()
                .map(|name| (snake_case(name), pascal_case(name)))
                .map(|(snake, pascal)| {
                    quote! {
                        #[allow(non_snake_case)]
                        pub fn #pascal(mut self, #snake: #pascal) -> Self {
                            self.element.children.push(#child::#pascal(#snake));
                            self
                        }
                    }
                });
            let description = format!(" The `<{}>` element's builder.", element.name);

            write(
                "builders",
                flat_case(&element.name),
                quote! {
                    use crate::generated::*;

                    #[doc = #description]
                    #[derive(Clone, Default, Debug)]
                    pub struct #builder {
                        pub(crate) element: #name,
                    }

                    impl #builder {
                        #(#attributes)*

                        pub fn child(mut self, child: #child) -> Self {
                            self.element.children.push(child);
                            self
                        }

                        #(#children)*

                        pub fn build(self) -> #name {
                            self.element
                        }
                    }
                },
            );
        });
    }
}

/// Utils.
impl Spec {}
