/*!
Generate human readable identifier strings by chaning common (short) words of
the english language.

This approach to the generation of identifiers is most commoly associated with
the company [what3words](https://what3words.com/) who have generated
fine-grained location identifiers that allow for very precise geo-location
based on 3 common (native language) words.

# Example

The following is a combination of the three functions provided and how they
allow progressively more customization of the generated identifier.

```
use human_id::{
    NO_SEPARATOR, Language, id, custom_id, custom_id_in_language
};

id();                    // HungryDucksListen

custom_id("-", false);   // tame-lions-retire
custom_id("", true);     // ChattyWombatsCare

custom_id_in_language(
    "-",
    false,
    Language::En);       // tame-lions-retire
custom_id_in_language(
    NO_SEPARATOR,
    true,
    Default::default()); // ChattyWombatsCare
```

# Command-Line Tool

This package also provides a command-line tool, `3wid` (for _three word
identifier_) which can be used for simple identifier creation.

```bash
$ 3wid --help
3wid 0.1.0
Generate 3-word human identifiers.

USAGE:
    3wid [FLAGS] [OPTIONS]

FLAGS:
    -h, --help
            Prints help information

    -n, --no-capitalize
            Turn off capitalization of words.

            By default the three words comprising the identifier will have
            their first character capitalized. This flag turns off this
            feature and generates all lowercase identifiers.
    -V, --version
            Prints version information


OPTIONS:
    -c, --count <count>
            The number of identifiers to generate, the default is one.

            This is a useful option for creating batches of identifiers
            with a common format. The generated identifiers are output
            one per line.
    -l, --language <language>
            The language to choose words from, the default is 'en'.

            The set of words can be chosen from any language although
            the library only has a small set of chosen languages. The
            string is the ISO 2-character language code in either all
            lowercase `"en"` or all uppercase `"EN"` characters.
    -s, --separator <separator>
            The separator string to use between words in the identifier,
            the default is none.

            The separator appears between the 3 words, such that a
            separator string `"/"` will create identifiers of the
            form `Olive/Lamps/Offer`.
```

The count (`-c` or `--count`) option allows the creation of batches of
identifiers, in the following fashion.

```bash
$ 3wid -c 5 -s "-" -n
tangy-flies-hide
khaki-aliens-repair
tangy-friends-relate
orange-boxes-repair
tidy-turkeys-occur
```

# Acknowledgements

This work is based on the [human id](https://crates.io/crates/human_id) crate
by [acheronfail](https://github.com/acheronfail) which in turn was based
upon the [Human-Readable
Identifiers](https://github.com/RienNeVaPlus/human-id) Typescript package.

 */

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::Display;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The value to use when no separator is required between words.
///
pub const NO_SEPARATOR: &str = "";

#[derive(Clone, Copy, Debug)]
///
/// The language to select words from.
///
pub enum Language {
    /// English language words.
    En,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Generates a "human id".
///
/// This uses the default configuration of word capitalization with no
/// separator string. The default language is used for word choices.
///
/// ```
/// use human_id::id;
///
/// id(); // HungryDucksListen
/// ```
///
pub fn id() -> String {
    custom_id(NO_SEPARATOR, true)
}

/// Generates a custom "human id".
///
/// This form allows:
///
/// * the insertion of a sepatator string between words,
/// * the choice of whether to capitalize the chosen words.
///
/// The default language is used for word choices.
///
/// ```
/// use human_id::custom_id;
///
/// custom_id("-", false); // tame-lions-retire
/// custom_id("", true);   // ChattyWombatsCare
/// ```
///
pub fn custom_id<S>(separator: S, should_capitalize: bool) -> String
where
    S: Into<String>,
{
    custom_id_in_language(separator, should_capitalize, Default::default())
}

///
/// Generates a custom "human id" in a chosen language.
///
/// This form allows for full customization of the generated identifier:
///
/// * the insertion of a sepatator string between words,
/// * the choice of whether to capitalize the chosen words,
/// * the choice of language for word choices.
///
/// ```
/// use human_id::{NO_SEPARATOR, custom_id_in_language};
///
/// custom_id_in_language(
///     "-",
///     false,
///     Language::En);         // tame-lions-retire
/// custom_id_in_language(
///     NO_SEPARATOR,
///     true,
///     Default::default());   // ChattyWombatsCare
/// ```
///
pub fn custom_id_in_language<S>(separator: S, should_capitalize: bool, language: Language) -> String
where
    S: Into<String>,
{
    let mut rng = thread_rng();

    let may_capitalize = |x: &&str| {
        if should_capitalize {
            capitalize(*x)
        } else {
            x.to_string()
        }
    };

    let chosen_words: (&[&str], &[&str], &[&str]) = match language {
        Language::En => (&words::en::ADJECTIVES, &words::en::NOUNS, &words::en::VERBS),
    };

    [
        chosen_words
            .0
            .choose(&mut rng)
            .map(may_capitalize)
            .unwrap()
            .to_string(),
        chosen_words
            .1
            .choose(&mut rng)
            .map(may_capitalize)
            .unwrap()
            .to_string(),
        chosen_words
            .2
            .choose(&mut rng)
            .map(may_capitalize)
            .unwrap()
            .to_string(),
    ]
    .join(&separator.into())
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn capitalize<S>(input: S) -> String
where
    S: Into<String>,
{
    let string = input.into();
    let mut chars = string.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Language {
    fn default() -> Self {
        Self::En
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::En => "en",
            }
        )
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" | "EN" => Ok(Self::En),
            _ => Err(format!(
                "The string '{}' is not a valid identifier, or supported language",
                s
            )),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod words;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use crate::custom_id;
    use crate::words::en::{ADJECTIVES, NOUNS, VERBS};

    fn capitals<S>(s: S) -> usize
    where
        S: Into<String>,
    {
        s.into().chars().filter(|c| c.is_uppercase()).count()
    }

    #[test]
    fn it_works_with_separator_and_no_capitalize() {
        let the_id = custom_id("-", false);

        let capitals = capitals(&the_id);
        assert_eq!(capitals, 0);

        let parts = the_id.split("-").collect::<Vec<&str>>();
        assert_eq!(parts.len(), 3);
        assert!(ADJECTIVES.contains(&parts[0]));
        assert!(NOUNS.contains(&parts[1]));
        assert!(VERBS.contains(&parts[2]));
    }

    #[test]
    fn it_works_without_separator_and_capitalized() {
        let the_id = custom_id("", true);

        let capitals = capitals(&the_id);
        assert_eq!(capitals, 3);
    }
}
