#![allow(dead_code)]

mod emoticons;

use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use emoticons::{Emoticon, DEFAULT_EMOTICONS};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Name of the emoticon to output (case insensitive)
    name: Option<String>,

    /// If a name is provided as an argument or via stdin and there are multiple icons with that
    /// name use the first icon found
    #[arg(short, long, default_value_t = true)]
    pick_first: bool,

    /// Copy icon to clipboard instead of printing it
    #[arg(short, long, default_value_t = false)]
    copy_to_clipboard: bool,
}

fn main() {
    let args = CliArgs::parse();

    let emoticons = DEFAULT_EMOTICONS.to_vec();
    let name = args.name;

    let possible_choices: Vec<_> = match &name {
        Some(name) => emoticons
            .into_iter()
            .filter(|emo| emo.name.eq_ignore_ascii_case(name))
            .collect(),
        None => emoticons,
    };

    let res = if name.is_some() && args.pick_first {
        possible_choices
            .get(0)
            .map(|emo| emo.icon)
            .unwrap_or(Emoticon::FALLBACK_ICON)
    } else {
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&possible_choices)
            .interact_on_opt(&Term::stderr())
            .unwrap_or(None);

        selection
            .map(|idx| possible_choices[idx].icon)
            .unwrap_or(Emoticon::FALLBACK_ICON)
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
