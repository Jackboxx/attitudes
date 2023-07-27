#![allow(dead_code)]

mod emoticons;

use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use emoticons::{Emoticon, OwnableEmoticon, DEFAULT_EMOTICONS};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Tag of the emoticon to output (case insensitive)
    tag: Option<String>,

    /// If a tag is provided as an argument or via stdin and there are multiple icons with that
    /// name use the first icon found
    #[arg(short, long, default_value_t = true)]
    pick_first: bool,

    /// Copy icon to clipboard instead of printing it
    #[arg(short, long, default_value_t = false)]
    copy_to_clipboard: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    extra_emoticons: Option<Vec<OwnableEmoticon>>,
}

// example config
impl Default for Config {
    fn default() -> Self {
        Self {
            extra_emoticons: Some(vec![
                OwnableEmoticon {
                    icon: ":D".to_owned(),
                    tags: vec!["smile".to_owned()],
                },
                OwnableEmoticon {
                    icon: "-_-".to_owned(),
                    tags: vec!["meh".to_owned(), "uh".to_owned()],
                },
            ]),
        }
    }
}

fn main() {
    let args = CliArgs::parse();
    let cfg: Config = confy::load(clap::crate_name!(), "config")
        .expect("Config file is malformed or doesn't exist");

    let emoticons = {
        let mut extra = cfg.extra_emoticons.unwrap_or(vec![]);
        let mut emoticons: Vec<OwnableEmoticon> = DEFAULT_EMOTICONS.map(|emo| emo.into()).to_vec();

        emoticons.append(&mut extra);
        emoticons
    };

    let tag = args.tag;

    let possible_choices: Vec<_> = match &tag {
        Some(tag) => emoticons
            .into_iter()
            .filter(|emo| emo.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
            .collect(),
        None => emoticons,
    };

    let res = if tag.is_some() && args.pick_first {
        possible_choices
            .get(0)
            .map(|emo| emo.icon.clone())
            .unwrap_or(Emoticon::FALLBACK_ICON.to_owned())
    } else {
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&possible_choices)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap_or(None);

        selection
            .map(|idx| possible_choices[idx].icon.clone())
            .unwrap_or(Emoticon::FALLBACK_ICON.to_owned())
    };

    // ¯\_(ツ)_/¯
    if !args.copy_to_clipboard
        || (|| {
            let mut ctx: ClipboardContext = ClipboardProvider::new()?;
            ctx.set_contents(res.to_owned())?;
            Ok::<(), Box<dyn std::error::Error>>(())
        })()
        .is_err()
    {
        print!("{res}");
    }
}
