mod idl;
mod parser;
mod spec;

use serde::{Deserialize, Serialize};
use spec::Spec;
use std::str::FromStr;

/// [https://html.spec.whatwg.org/multipage/dom.html#content-models]
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Category {
    EmbeddedContent,
    FlowContent,
    HeadingContent,
    InteractiveContent,
    MetadataContent,
    PalpableContent,
    PhrasingContent,
    SectioningContent,

    FormAssociated, // All the following are form-associated categories:
    Listed,
    Submittable,
    Resettable,
    AutocapitalizeInheriting,
    Labelable, // (also includes non-form-associated elements)

    Document,
    Media,            // <audio> and <video>
    ScriptSupporting, // <script> and <template>
    Text,
    Transparent, // SHIT!

    Nothing,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .trim()
            .to_lowercase()
            .replace(' ', "")
            .replace('-', "")
            .as_str()
        {
            "embedded" => Ok(Self::EmbeddedContent),
            "flow" => Ok(Self::FlowContent),
            "heading" => Ok(Self::HeadingContent),
            "interactive" => Ok(Self::InteractiveContent),
            "metadata" => Ok(Self::MetadataContent),
            "palpable" => Ok(Self::PalpableContent),
            "phrasing" => Ok(Self::PhrasingContent),
            "sectioning" => Ok(Self::SectioningContent),
            "formassociated" => Ok(Self::FormAssociated),
            "listed" => Ok(Self::Listed),
            "submittable" => Ok(Self::Submittable),
            "resettable" => Ok(Self::Resettable),
            "autocapitalizeinheriting" => Ok(Self::AutocapitalizeInheriting),
            "labelable" => Ok(Self::Labelable),
            "document" => Ok(Self::Document),
            "media" => Ok(Self::Media),
            "scriptsupporting" => Ok(Self::ScriptSupporting),
            "text" => Ok(Self::Text),
            "transparent" => Ok(Self::Transparent),
            "nothing" => Ok(Self::Nothing),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub enum CategoryOrElement {
    Category(Category),
    Element(String),
}

impl From<&str> for CategoryOrElement {
    fn from(s: &str) -> Self {
        match Category::from_str(s) {
            Ok(category) => Self::Category(category),
            Err(_) => Self::Element(s.to_string()),
        }
    }
}

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    let command = std::env::args().nth(1);
    match command {
        None => {
            Spec::fetch().await;
            Spec::scrape().await;
            Spec::parse().await;
        }
        Some(arg) if arg == "fetch" => {
            Spec::fetch().await;
        }
        Some(arg) if arg == "scrape" => {
            Spec::scrape().await;
        }
        Some(arg) if arg == "parse" => {
            Spec::parse().await;
        }
        command => {
            tracing::error!(command, "Unknown command");
            return;
        }
    }
}
