use phf::phf_map;

use crate::btrie::{BTrie, Go};

pub static LETTER_TO_MORSE: phf::Map<char, &'static str> = phf_map! {
    'A' => ".-",
    'B' => "-...",
    'C' => "-.-.",
    'D' => "-..",
    'E' => ".",
    'F' => "..-.",
    'G' => "--.",
    'H' => "....",
    'I' => "..",
    'J' => ".---",
    'K' => "-.-",
    'L' => ".-..",
    'M' => "--",
    'N' => "-.",
    'O' => "---",
    'P' => ".--.",
    'Q' => "--.-",
    'R' => ".-.",
    'S' => "...",
    'T' => "-",
    'U' => "..-",
    'V' => "...-",
    'W' => ".--",
    'X' => "-..-",
    'Y' => "-.--",
    'Z' => "--..",

    '1' => ".----",
    '2' => "..---",
    '3' => "...--",
    '4' => "....-",
    '5' => ".....",
    '6' => "-....",
    '7' => "--...",
    '8' => "---..",
    '9' => "----.",
    '0' => "-----",

    // whitespace is special
    ' ' => "/",
    '\t' => "/",
    '\n' => "/",

    // perhaps add more characters (international alphabets) later
};

pub fn str_to_go(morse: &str) -> Vec<Go> {
    Go::collect_str(&morse.replace(".", "L").replace("-", "R"))
}

lazy_static! {
    pub static ref MORSE_TO_LETTER: BTrie<char> = {
        let mut root = BTrie::default();

        for (k, v) in LETTER_TO_MORSE
            .entries()
            .filter(|(k, _v)| !k.is_ascii_whitespace())
        {
            root.insert(&str_to_go(v), *k);
        }
        root
    };
}
