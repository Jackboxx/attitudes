#![allow(dead_code)]

mod emoticons;

use clap::Parser;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use emoticons::DEFAULT_EMOTICONS;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Name of the emoticon to output (case insensitive)
    name: Option<String>,

    /// If a name is provided as an argument or via stdin and there are multiple icons with that
    /// name use the first icon found
    #[arg(short, long, default_value_t = true)]
    pick_first: bool,
}

fn main() -> std::io::Result<()> {
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
        possible_choices[0].icon // TODO: don't die if no match
    } else {
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&possible_choices)
            .interact_on_opt(&Term::stderr())?;

        selection
            .map(|idx| possible_choices[idx].icon)
            .unwrap_or("")
    };

    print!("{res}");
    Ok(())
}
