use crate::parser::{ParsedElement, ParsedIdl, ParsedInterface, Parser, UsesOrParsedInterface};
use scraper::{ElementRef, Html};
use std::{
    collections::{HashMap, HashSet},
    fs::{create_dir_all, read_to_string, write},
};

macro_rules! selector {
    ($($arg:tt)*) => {{
        let selector = format!($($arg)*);
        ::scraper::Selector::parse(&selector)
            .unwrap_or_else(|err| panic!("`{selector}`: {err:?}"))
    }};
}

// TODO
// - Void elements: https://html.spec.whatwg.org/multipage/syntax.html#elements-2
// - global attributes and event handlers: https://html.spec.whatwg.org/multipage/dom.html#global-attributes
// - attributes: https://html.spec.whatwg.org/multipage/syntax.html#attributes-2
// - HTMLElement IDL: https://html.spec.whatwg.org/multipage/dom.html#elements-in-the-dom

#[derive(Debug)]
pub enum Spec {}

impl Spec {
    pub const URL: &'static str = "https://html.spec.whatwg.org";
    pub const RESOURCES: &'static str = "resources";
    pub const FETCHED: &'static str = "_specs.html";

    pub async fn fetch() {
        tracing::info!("FETCHING");

        let url = Self::URL;
        let request = reqwest::get(url).await.unwrap();
        let response = request.text().await.unwrap();

        Self::write_fetched(&response);
    }

    pub async fn prepare() {
        tracing::info!("PREPARING");

        let html = Self::read_fetched();

        let elements = Self::parse_elements(&html).await;
        let interfaces = Self::parse_interfaces(&html).await;
    }
}

/// Private.
impl Spec {
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
                    h4.value().attr("id")?,
                    h4.select(&selector!("dfn[data-dfn-type=\"element\"] > code"))
                        .map(|code| code.inner_html())
                        .collect::<Vec<_>>(),
                    h4.next_siblings()
                        .flat_map(ElementRef::wrap)
                        .find(|element| selector!("dl.element").matches(element))?,
                ))
            })
            .flat_map(|(id, names, definition)| {
                names
                    .into_iter()
                    .map(move |name| (id.to_owned(), name, definition))
            })
            .map(|(id, name, definition)| {
                #[derive(Copy, Clone, Debug)]
                enum Section {
                    Categories,
                    Contexts,
                    ContentModel,
                    TagOmission,
                    Attributes,
                    Accessibility,
                    DOM,
                }

                let mut section = None;
                let mut element = ParsedElement {
                    id: id.to_owned(),
                    name: name.to_owned(),
                    categories: HashSet::new(),
                    contexts: HashSet::new(),
                    contents: HashSet::new(),
                    end_tag: true,
                    global_attributes: true,
                    attributes: HashMap::new(),
                    idl: UsesOrParsedInterface::Uses(String::from("__INVALID__")),
                };

                for d in definition.select(&selector!("dt, dd")) {
                    let text = d.text().collect::<String>();

                    match d.value().name() {
                        "dt" => {
                            let str = d.select(&selector!("a")).next().unwrap().inner_html();
                            section = Some(match str.to_lowercase().replace(' ', "").as_str() {
                                "categories" => Section::Categories,
                                "contextsinwhichthiselementcanbeused" => Section::Contexts,
                                "contentmodel" => Section::ContentModel,
                                "tagomissionintext/html" => Section::TagOmission,
                                "contentattributes" => Section::Attributes,
                                "accessibilityconsiderations" => Section::Accessibility,
                                "dominterface" => Section::DOM,
                                _ => panic!(),
                            });
                        }
                        "dd" => match section.unwrap() {
                            Section::Categories => {
                                parser.category(&text, &mut element);
                            }
                            Section::Contexts => {
                                parser.context(&text, &mut element);
                            }
                            Section::ContentModel => {
                                parser.content(&text, &mut element);
                            }
                            Section::TagOmission => {
                                parser.tag_omission(&text, &mut element);
                            }
                            Section::Attributes => {
                                parser.attribute(&text, &mut element);
                            }
                            Section::Accessibility => {}
                            Section::DOM => {
                                let text = d
                                    .select(&selector!("pre"))
                                    .next()
                                    .map(|pre| pre.text().collect::<String>())
                                    .unwrap_or(text);
                                parser.idl(&text, &mut element);
                            }
                        },
                        _ => panic!(),
                    }
                }

                element
            })
            .collect();

        parser.errors();
        elements
    }

    async fn parse_interfaces(html: &Html) -> Vec<ParsedInterface> {
        let idl = html
            .select(&selector!("code.idl"))
            .map(|idl| {
                idl.text()
                    .collect::<String>()
                    .trim_start_matches("[Exposed=*]") // `weedle` chokes on this
                    .to_owned()
            })
            .collect::<String>();

        let idl = ParsedIdl::parse(&idl);
        dbg!(&idl);

        todo!("OK")
    }
}

/// Utils.
impl Spec {
    fn fetched_file() -> String {
        format!("{}/{}", Self::RESOURCES, Self::FETCHED)
    }

    pub fn write_fetched(specs: &str) {
        create_dir_all(Self::RESOURCES).unwrap();
        write(Self::fetched_file(), specs).unwrap();
    }

    pub fn read_fetched() -> Html {
        let html = Html::parse_document(&read_to_string(Self::fetched_file()).unwrap());
        assert!(html.errors.is_empty());
        html
    }
}
