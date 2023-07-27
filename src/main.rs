#![allow(dead_code)]

mod emoticons;

use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use emoticons::DEFAULT_EMOTICONS;

fn main() -> std::io::Result<()> {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&DEFAULT_EMOTICONS)
        .interact_on_opt(&Term::stderr())?;

    let res = selection
        .map(|idx| DEFAULT_EMOTICONS[idx].icon)
        .unwrap_or("");

    print!("{res}");
    Ok(())
}
