pub mod btrie;
pub mod lookup;

#[macro_use]
extern crate lazy_static;

pub fn convert_to_morse(input: &str) -> String {
    let mut results: Vec<&str> = vec![];
    for character in input.to_uppercase().chars() {
        if let Some(result) = lookup::LETTER_TO_MORSE.get(&character) {
            results.push(result);
            results.push(" ");
        }
    }

    results.pop(); // remove last space
    results.join("")
}

pub fn convert_from_morse(input: &str) -> String {
    let mut results: Vec<_> = vec![];
    for word in input.split(&['/', '\\'][..]) {
        for character in word.split_whitespace() {
            results.push(lookup::MORSE_TO_LETTER.get(&lookup::str_to_go(character)));
        }
        results.push(Some(&' '));
    }

    results.pop(); // remove last space
    results.into_iter().filter_map(|x| x).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to() {
        assert_eq!(convert_to_morse("ABC"), ".- -... -.-.");
    }

    #[test]
    fn convert_to_words() {
        assert_eq!(convert_to_morse("abcdefg hijklmnop qrs tuv wx yz"), ".- -... -.-. -.. . ..-. --. / .... .. .--- -.- .-.. -- -. --- .--. / --.- .-. ... / - ..- ...- / .-- -..- / -.-- --..");
    }

    #[test]
    fn convert_from() {
        assert_eq!(convert_from_morse("."), "E");
    }

    #[test]
    fn convert_from_words() {
        assert_eq!(convert_from_morse(".- -... -.-. -.. . ..-. --. / .... .. .--- -.- .-.. -- -. --- .--. / --.- .-. ... / - ..- ...- / .-- -..- / -.-- --.."), "ABCDEFG HIJKLMNOP QRS TUV WX YZ");
    }
}
