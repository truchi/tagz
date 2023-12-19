use crate::{parser::Parser, Category, CategoryOrElement};
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::{create_dir_all, read_dir, read_to_string, write},
    ops::Range,
    process::Command,
    str::FromStr,
};

macro_rules! selector {
    ($($arg:tt)*) => {{
        let selector = format!($($arg)*);
        ::scraper::Selector::parse(&selector)
            .unwrap_or_else(|err| panic!("`{selector}`: {err:?}"))
    }};
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedElement {
    pub id: String,
    pub name: String,
    pub categories: HashSet<Category>,
    pub contexts: HashSet<CategoryOrElement>,
    pub contents: HashSet<CategoryOrElement>,
}

#[derive(Debug)]
pub enum Spec {}

impl Spec {
    pub const URL: &'static str = "https://html.spec.whatwg.org";
    pub const RESOURCES: &'static str = "resources";
    pub const FETCHED: &'static str = "1-fetched.html";
    pub const SCRAPED: &'static str = "2-scraped";
    pub const PARSED: &'static str = "3-parsed";

    pub async fn fetch() {
        tracing::info!("FETCHING");

        Self::write_fetched(&slim(
            fetch(Self::URL).await,
            selector!("#semantics")..selector!("#microdata"),
        ));
    }

    pub async fn scrape() {
        tracing::info!("SCRAPING");

        let html = Self::read_fetched();
        Self::scrape_elements(&html).await;
    }

    pub async fn parse() {
        tracing::info!("PARSING");

        let scrapeds = {
            let mut scraped = Self::read_scraped();
            scraped.sort_by_key(|(_, name, _)| name.clone());
            scraped
        };
        let mut parser = Parser::new();

        for (id, name, definition) in scrapeds {
            Self::parse_element(&id, &name, &definition, &mut parser).await;
        }

        parser.errors();
    }
}

/// Private.
impl Spec {
    async fn scrape_elements(html: &Html) {
        let selector = selector!("h4");
        let elements = html
            .select(&selector)
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
            });

        for (id, name, definition) in elements {
            Self::write_scraped(&id, &name, definition);
        }
    }

    async fn parse_element(id: &str, name: &str, definition: &Html, parser: &mut Parser) {
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

        impl FromStr for Section {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_lowercase().replace(' ', "").as_str() {
                    "categories" => Ok(Self::Categories),
                    "contextsinwhichthiselementcanbeused" => Ok(Self::Contexts),
                    "contentmodel" => Ok(Self::ContentModel),
                    "tagomissionintext/html" => Ok(Self::TagOmission),
                    "contentattributes" => Ok(Self::Attributes),
                    "accessibilityconsiderations" => Ok(Self::Accessibility),
                    "dominterface" => Ok(Self::DOM),
                    _ => Err(()),
                }
            }
        }

        let mut element = ParsedElement {
            id: id.to_owned(),
            name: name.to_owned(),
            categories: HashSet::new(),
            contexts: HashSet::new(),
            contents: HashSet::new(),
        };
        let mut section = Option::<Section>::None;

        for d in definition.select(&selector!("dt, dd")) {
            let text = d.text().collect::<String>();

            match d.value().name() {
                "dt" => {
                    let str = d.select(&selector!("a")).next().unwrap().inner_html();
                    section = Some(Section::from_str(&str).unwrap());
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
                    Section::TagOmission => {}
                    Section::Attributes => {}
                    Section::Accessibility => {}
                    Section::DOM => {}
                },
                _ => panic!(),
            }
        }

        Self::write_parsed(&element);
    }
}

/// Utils.
impl Spec {
    fn fetched_file() -> String {
        format!("{}/{}", Self::RESOURCES, Self::FETCHED)
    }

    fn scraped_dir() -> String {
        format!("{}/{}", Self::RESOURCES, Self::SCRAPED)
    }

    fn scraped_file(name: &str) -> String {
        format!("{}/{}.html", Self::scraped_dir(), name)
    }

    fn parsed_dir() -> String {
        format!("{}/{}", Self::RESOURCES, Self::PARSED)
    }

    fn parsed_file(name: &str) -> String {
        format!("{}/{}.json", Self::parsed_dir(), name)
    }

    pub fn write_fetched(html: &Html) {
        tracing::debug!("WRITING {}", Self::fetched_file());

        create_dir_all(Self::RESOURCES).unwrap();
        write(Self::fetched_file(), html.root_element().html()).unwrap();
    }

    pub fn read_fetched() -> Html {
        tracing::debug!("READING {}", Self::fetched_file());

        Html::parse_document(&read_to_string(Self::fetched_file()).unwrap())
    }

    pub fn write_scraped(id: &str, name: &str, definition: ElementRef) {
        tracing::debug!("WRITING {}", Self::scraped_file(name));

        create_dir_all(Self::scraped_dir()).unwrap();
        write(
            Self::scraped_file(name),
            format!("<!-- {id} -->\n{}", definition.inner_html()),
        )
        .unwrap();
        prettier(&Self::scraped_file(name));
    }

    pub fn read_scraped() -> Vec<(
        /* id: */ String,
        /* name: */ String,
        /* definition: */ Html,
    )> {
        tracing::debug!("READING {}", Self::scraped_dir());

        read_dir(Self::scraped_dir())
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.unwrap();
                let contents = read_to_string(entry.path()).unwrap();
                let (id, html) = contents.split_once("\n").unwrap();

                Some((
                    id.trim_start_matches("<!-- ")
                        .trim_end_matches(" -->")
                        .to_string(),
                    entry
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_string_lossy()
                        .to_string()
                        .to_owned(),
                    Html::parse_document(&html),
                ))
            })
            .collect()
    }

    pub fn write_parsed(element: &ParsedElement) {
        tracing::debug!("WRITING {}", Self::parsed_file(&element.name));

        create_dir_all(Self::parsed_dir()).unwrap();
        write(
            Self::parsed_file(&element.name),
            serde_json::to_string_pretty(element).unwrap(),
        )
        .unwrap();
    }
}

async fn fetch(url: &str) -> Html {
    tracing::debug!("FETCHING {}", url);

    let request = reqwest::get(url).await.unwrap();
    let response = request.text().await.unwrap();
    let html = Html::parse_document(&response);

    if html.errors.len() > 0 {
        panic!("HTML errors: {:?}", html.errors);
    } else {
        html
    }
}

fn prettier(file: &str) {
    tracing::debug!("PRETTIER {}", file);

    _ = Command::new("prettier")
        .args(&["-w", file, "--ignore-path", "false", "--log-level", "warn"])
        .spawn()
        .map(|mut child| {
            child
                .wait()
                .map_err(|err| tracing::warn!(?err, "Prettier failed"))
        })
        .map_err(|err| tracing::warn!(?err, "Prettier failed"));
}

/// This specification must go on a diet!
fn slim(mut html: Html, range: Range<Selector>) -> Html {
    let Range { start, end } = range;

    #[derive(Copy, Clone, Debug)]
    enum State {
        Before,
        Inside,
        After,
    }

    let body = html.select(&selector!("body")).next().unwrap();
    let (state, node_ids_to_remove) =
        body.children()
            .fold((State::Before, Vec::new()), |(state, mut remove), node| {
                if let Some(element) = ElementRef::wrap(node) {
                    return (
                        match state {
                            State::Before => {
                                if start.matches(&element) {
                                    State::Inside
                                } else {
                                    remove.push(node.id());
                                    state
                                }
                            }
                            State::Inside => {
                                if end.matches(&element) {
                                    remove.push(node.id());
                                    State::After
                                } else {
                                    state
                                }
                            }
                            State::After => {
                                remove.push(node.id());
                                state
                            }
                        },
                        remove,
                    );
                } else {
                    match state {
                        State::Before | State::After => remove.push(node.id()),
                        State::Inside => {}
                    }
                    return (state, remove);
                };
            });

    match state {
        State::Before => tracing::warn!(?state),
        State::Inside => tracing::warn!(?state),
        State::After => {}
    }

    for id in node_ids_to_remove {
        html.tree.get_mut(id).unwrap().detach();
    }

    html
}
