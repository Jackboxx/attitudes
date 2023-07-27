// I am to lazy to make this work with 1 struct so fuck it

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Emoticon<'a> {
    pub tags: &'a [&'a str],
    pub icon: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OwnableEmoticon {
    pub tags: Vec<String>,
    pub icon: String,
}

impl Display for OwnableEmoticon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{icon:20}\t tags: {tags}",
            icon = self.icon,
            tags = self.tags.join(", ")
        )
    }
}

impl<'a> From<Emoticon<'a>> for OwnableEmoticon {
    fn from(value: Emoticon<'a>) -> Self {
        Self {
            tags: value
                .tags
                .into_iter()
                .map(|t| t.to_owned().to_owned())
                .collect(),
            icon: value.icon.to_owned(),
        }
    }
}

impl<'a> Emoticon<'a> {
    pub const FALLBACK_ICON: &str = "¯\\_(ツ)_/¯";
}

pub const DEFAULT_EMOTICONS: [Emoticon; 0] = [];
