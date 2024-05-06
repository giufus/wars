use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ALPHABET: HashSet<char> = ('a'..='z').collect();
}

macro_rules! init_alphabet {
    () => {
        {
            let mut alphabet: HashSet<char> = HashSet::new();
            for c in 'a'..='z' {
                alphabet.insert(c);
            }
            alphabet
        }
    };
}

pub fn is_pangram(s: &str) -> bool {

    let mut alphabet = ALPHABET.clone();

    s.to_lowercase().chars().for_each(|c| {
        alphabet.remove(&c);
    });

    alphabet.is_empty()
}



#[cfg(test)]
mod tests {
    use super::is_pangram;

    fn dotest(s: &str, expected: bool) {
        let actual = is_pangram(s);
        assert!(
            actual == expected,
            "Test failed with s = \"{s}\"\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest("The quick, brown fox jumps over the lazy dog!", true);
        dotest("Cwm fjord bank glyphs vext quiz", true);
        dotest("Pack my box with five dozen liquor jugs.", true);
        dotest("How quickly daft jumping zebras vex.", true);
        dotest("ABCD45EFGH,IJK,LMNOPQR56STUVW3XYZ", true);
        dotest("This isn't a pangram!", false);
        dotest("abcdefghijklmopqrstuvwxyz", false);
        dotest("Aacdefghijklmnopqrstuvwxyz", false);
    }
}
