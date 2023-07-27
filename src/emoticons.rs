use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Emoticon {
    pub tags: Vec<String>,
    pub icon: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmoticonJson {
    pub new_tags: Vec<String>,
    pub original_tags: Vec<String>,
}

impl Emoticon {
    pub fn from_json_data(data: HashMap<String, EmoticonJson>) -> Vec<Emoticon> {
        data.into_iter()
            .map(|(k, v)| {
                let mut tags = v.new_tags;
                let mut org = v.original_tags;

                tags.append(&mut org);
                let icon = k;
                Emoticon { icon, tags }
            })
            .collect()
    }
}

impl Display for Emoticon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{icon:20}\t tags: {tags}",
            icon = self.icon,
            tags = self.tags.join(", ")
        )
    }
}

impl Emoticon {
    pub const FALLBACK_ICON: &str = "¯\\_(ツ)_/¯";
}
