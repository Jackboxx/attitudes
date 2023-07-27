// I am to lazy to make this work with 1 struct so fuck it

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Emoticon<'a> {
    pub name: &'a str,
    pub icon: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OwnableEmoticon {
    pub name: String,
    pub icon: String,
}

impl Display for OwnableEmoticon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{name:20}\t {icon}", name = self.name, icon = self.icon)
    }
}

impl<'a> From<Emoticon<'a>> for OwnableEmoticon {
    fn from(value: Emoticon<'a>) -> Self {
        Self {
            name: value.name.to_owned(),
            icon: value.icon.to_owned(),
        }
    }
}

impl<'a> Emoticon<'a> {
    pub const FALLBACK_ICON: &str = "¯\\_(ツ)_/¯";
}

pub const DEFAULT_EMOTICONS: [Emoticon; 3] = [
    Emoticon {
        name: "Flower Girl",
        icon: "(◕‿◕✿)",
    },
    Emoticon {
        name: "Cat",
        icon: "ฅ(^•ﻌ•^ฅ)",
    },
    Emoticon {
        name: "Cat",
        icon: "(=ʘᆽʘ=)∫",
    },
];
