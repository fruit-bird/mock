//! # MoCkiNg
//!
//! A CLI that converts text into the SpongeBob mocking case

// (done) use rand to choose which chars to convert
// (done) have it copy the output to clipboard
// (done) maybe even input from clipboard
// (done) env var that decides wether or not to output result to clipboard or stdout
// (pain) convert String to &str in the end
// (good) make this CLI friendly
//     - if command takes no args          --> clipboard copy
//     - if commands takes arg (string)    --> output to terminal
//     + remove need for env var
//     + takes -c flag to copy the output to the clipboard

use anyhow::Result;
use arboard::Clipboard;
use clap::{arg, command, ArgMatches};
use rand::Rng;
use std::fmt::Display;

pub struct Mocker(String);

impl Mocker {
    pub fn new(input: impl AsRef<str>) -> Result<Self> {
        let output = Self::mocking_spongebob_case(input);
        Ok(Self(output))
    }

    /// Copies the input from the clipboard and pastes it back into the clipboard
    pub fn new_from_clipboard() -> Result<Self> {
        let mut clipboard = Clipboard::new()?;

        let input = clipboard.get_text().unwrap_or_default();
        let output = Self::mocking_spongebob_case(input);
        clipboard.set_text(&output)?;

        Ok(Self(output))
    }

    fn mocking_spongebob_case(input: impl AsRef<str>) -> String {
        let mut rng = rand::thread_rng();

        input
            .as_ref()
            .chars()
            .map(|c| {
                if rng.gen_bool(0.55) {
                    c.to_uppercase()
                        .to_string()
                        .parse::<char>()
                        .expect("Parsing error")
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Display for Mocker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn cli_builder() -> ArgMatches {
    let matches = command!()
        .author("fruit-bird")
        .about("Converts text into MoCkiNg case")
        .arg(arg!(<STRING> "Text to convert. Must be in double quotes").required(false)) // this is optional when -c is used, unless i change it to do clipboard id not given text
        .arg(arg!(-i --input <STRING> "Text to convert. Must be in double quotes").required(false)) // this is optional when -c is used, unless i change it to do clipboard id not given text
        .arg(arg!(-c --clipboard "Copies conversion to clipboard").required(false))
        // .arg(arg!(-r --reverse "Converts text back to original state").required(false)) // this should copy og text into some file somewhere
        .get_matches();
    matches
}

pub fn run() -> Result<()> {
    let matches = cli_builder();

    let output = match matches.get_one::<String>("input") {
        Some(text) => Mocker::new(text)?,
        None => Mocker::new_from_clipboard()?,
    };

    if matches.get_flag("clipboard") {
        Clipboard::new()?.set_text(&output.0)?;
        println!("Your MoCkiNg text has been copied into your clipboard!\n");
    }

    println!("{}", output);

    Ok(())
}
