// Taken from file plugins/http/src/scope.rs in repository tauri-apps/plugins-workspace

use std::sync::Arc;

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

#[derive(Debug)]
pub struct Scope<'a> {
    allowed: Vec<&'a Arc<Entry>>,
    denied: Vec<&'a Arc<Entry>>,
}

impl<'a> Scope<'a> {
    /// Creates a new scope from the scope configuration.
    pub(crate) fn new(allowed: Vec<&'a Arc<Entry>>, denied: Vec<&'a Arc<Entry>>) -> Self {
        Self { allowed, denied }
    }

    /// Determines if the given URL is allowed on this scope.
    pub fn is_allowed(&self, url: &Url) -> bool {
        let denied = self.denied.iter().any(|entry| {
            entry
                .url
                .test(UrlPatternMatchInput::Url(url.clone()))
                .unwrap_or_default()
        });
        if denied {
            false
        } else {
            self.allowed.iter().any(|entry| {
                entry
                    .url
                    .test(UrlPatternMatchInput::Url(url.clone()))
                    .unwrap_or_default()
            })
        }
    }
}