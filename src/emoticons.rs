use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Emoticon<'a> {
    pub name: &'a str,
    pub icon: &'a str,
}

impl<'a> Display for Emoticon<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{name:20}\t {icon}", name = self.name, icon = self.icon)
    }
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
