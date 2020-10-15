extern crate moroselib;

use clap::{App, AppSettings, Arg};
use moroselib::{convert_from_morse, convert_to_morse};
use std::io;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");

const AFTER_HELP: &str = r#"A note on conversions from Morse:
Morose tries to be liberal in what it accepts. It will ignore characters that are not in the set [.-/ ]."#;

fn main() -> Result<(), io::Error> {
    let matches = App::new("Morose")
        .version(VERSION.unwrap_or("unknown"))
        .author(AUTHORS.unwrap_or("unknown"))
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::with_name("to")
                .short("t")
                .long("to")
                .conflicts_with("from")
                .help("Convert to Morse code (conflicts with --from)."),
        )
        .arg(
            Arg::with_name("from")
                .short("f")
                .long("from")
                .conflicts_with("to")
                .help("Convert from Morse code (conflicts with --to)."),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("String to convert to or from Morse Code.")
                .multiple(true),
        )
        .after_help(AFTER_HELP)
        .get_matches();

    let input = match matches.values_of("INPUT") {
        Some(values) => values.collect::<Vec<&str>>().join(" "),
        None => "".to_owned(),
    };

    // our default is "to"
    let convert_to = !matches.is_present("from");

    let result = if convert_to {
        convert_to_morse(&input)
    } else {
        convert_from_morse(&input)
    };

    if !result.is_empty() {
        println!("{}", result);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use std::str;

    #[test]
    fn morse_input() {
        let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(&["--from", "."])
            .output()
            .expect("Failed to execute process.");
        assert_eq!(
            str::from_utf8(&output.stdout).expect("Failed to convert output into a &str"),
            "E\n"
        );
    }

    #[test]
    fn string_input() {
        let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(&["--to", "c"])
            .output()
            .expect("Failed to execute process.");
        assert_eq!(
            str::from_utf8(&output.stdout).expect("Failed to convert output into a &str."),
            "-.-.\n"
        );
    }

    #[test]
    fn no_input() {
        let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .output()
            .expect("Failed to execute process.");
        assert_eq!(
            str::from_utf8(&output.stdout).expect("Failed to convert output into a &str."),
            ""
        );
    }

    #[test]
    fn no_args() {
        let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .arg("v")
            .output()
            .expect("Failed to execute process.");
        assert_eq!(
            str::from_utf8(&output.stdout).expect("Failed to convert output into a &str."),
            "...-\n"
        );
    }

    #[test]
    fn unconvertable_input() {
        let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .arg(".")
            .output()
            .expect("Failed to execute process.");
        assert_eq!(
            str::from_utf8(&output.stdout).expect("Failed to convert output into a &str."),
            ""
        );
    }
}
