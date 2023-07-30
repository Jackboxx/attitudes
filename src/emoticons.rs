use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Emoticon {
    pub tags: Vec<String>,
    pub icon: String,
}

impl Display for Emoticon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{icon:40}\t tags: {tags}",
            icon = self.icon,
            tags = self.tags.join(", ")
        )
    }
}

impl Emoticon {
    pub const FALLBACK_ICON: &str = "¯\\_(ツ)_/¯";
}
