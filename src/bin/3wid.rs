use human_id::{custom_id_in_language, Language};
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "3wid", about = "Generate 3-word human identifiers.")]
pub struct CommandLine {
    #[structopt(short, long)]
    /// The number of identifiers to generate, the default is one.
    ///
    /// This is a useful option for creating batches of identifiers with a
    /// common format. The generated identifiers are output one per line.
    ///
    count: Option<u16>,

    #[structopt(short, long)]
    /// The separator string to use between words in the identifier, the
    /// default is none.
    ///
    /// The separator appears between the 3 words, such that a separator
    /// string `"/"` will create identifiers of the form `Olive/Lamps/Offer`.
    ///
    separator: Option<String>,

    #[structopt(short, long)]
    /// Turn off capitalization of words.
    ///
    /// By default the three words comprising the identifier will have their
    /// first character capitalized. This flag turns off this feature and
    /// generates all lowercase identifiers.
    ///
    no_capitalize: bool,

    #[structopt(short, long)]
    /// The language to choose words from, the default is 'en'.
    ///
    /// The set of words can be chosen from any language although the library
    /// only has a small set of chosen languages. The string is the ISO
    /// 2-character language code in either all lowercase `"en"` or all
    /// uppercase `"EN"` characters.
    ///
    language: Option<String>,
}

pub fn main() {
    let cmd_line = CommandLine::from_args();

    let separator = cmd_line
        .separator
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or_default();

    let capitalize_words = !cmd_line.no_capitalize;

    let language = if let Some(language) = cmd_line.language.as_ref() {
        match Language::from_str(&language) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        Language::default()
    };

    (0..cmd_line.count.unwrap_or(1)).for_each(|_| {
        println!(
            "{}",
            custom_id_in_language(&separator, capitalize_words, language)
        )
    })
}
