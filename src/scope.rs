// Taken from file plugins/http/src/scope.rs in repository tauri-apps/plugins-workspace

use serde::{Deserialize, Deserializer};
use url::Url;
use urlpattern::{UrlPattern, UrlPatternMatchInput};

#[allow(rustdoc::bare_urls)]
#[derive(Debug)]
pub struct Entry {
    pub url: UrlPattern,
}

fn parse_url_pattern(s: &str) -> Result<UrlPattern, urlpattern::quirks::Error> {
    let mut init = urlpattern::UrlPatternInit::parse_constructor_string::<regex::Regex>(s, None)?;
    if init.search.as_ref().map(|p| p.is_empty()).unwrap_or(true) {
        init.search.replace("*".to_string());
    }
    if init.hash.as_ref().map(|p| p.is_empty()).unwrap_or(true) {
        init.hash.replace("*".to_string());
    }
    if init
        .pathname
        .as_ref()
        .map(|p| p.is_empty() || p == "/")
        .unwrap_or(true)
    {
        init.pathname.replace("*".to_string());
    }
    UrlPattern::parse(init, Default::default())
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum EntryRaw {
    Value(String),
    Object { url: String },
}

impl<'de> Deserialize<'de> for Entry {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        EntryRaw::deserialize(deserializer).and_then(|raw| {
            let url = match raw {
                EntryRaw::Value(url) => url,
                EntryRaw::Object { url } => url,
            };
            Ok(Entry {
                url: parse_url_pattern(&url).map_err(|e| {
                    serde::de::Error::custom(format!("`{url}` is not a valid URL pattern: {e}"))
                })?,
            })
        })
    }
}
