use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // input string
    slug_in: String,
}

fn slugify(s: &str) -> String {
    let mut result = String::new();
    let mut previous_char= ' ';

    for letter in s.chars() {
        let converted_char = conv(letter);

        if !(previous_char == '-' && converted_char == '-') {
            result.push(converted_char);
        }

        previous_char = converted_char;
    }

    /*
     * If the result has more than one character and ends with '-', remove it.
     * If, on the other hand, the only character is '-', leave it.
     */

    if result.len() > 1 && result.ends_with('-') {
        result.pop();
    }

    result
}

fn conv(c: char) -> char {
    const SUBS_I : &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O : &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

    let chars_i: Vec<char> = SUBS_I.chars().collect();
    let chars_o: Vec<char> = SUBS_O.chars().collect();

    // looking for the position of c in SUBS_I
    if let Some(i) = chars_i.iter().position(|&x| x == c) {
        /*
         * If we find it, we extract the corresponding character from SUBS_O.
         * (nth() requires a char iterator, so we use SUBS_O.chars())
         */
        chars_o[i]
    }
    else if !c.is_ascii_alphabetic() && !c.is_ascii_digit() {
        // if c is not in SUBS_I, it's already ok, so we can return it
        '-'
    }
    else {
        c.to_ascii_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accented_letter() {
        // for example, check that 'à' is converted to 'a'
        let input = 'à';
        let expected = 'a';
        let output = conv(input);

        assert_eq!(output, expected, "Accented character not converted correctly");
    }

    #[test]
    fn test_nonaccented_letter() {
        let input = 'b';
        let expected = 'b';
        let output = conv(input);

        assert_eq!(output, expected, "Unaccented character should not be edited");
    }

    #[test]
    fn test_unknown_non_allowed_char() {
        let input = '#';
        // if '#' is not in SUBS_I and is not alphanumeric, our conv() function converts it to '-'
        let expected = '-';
        let output = conv(input);

        assert_eq!(output, expected, "Unknown disallowed character is not converted correctly to '-'");
    }

    #[test]
    fn test_accented_letter_not_in_subs_i () {
        let input = 'ῶ';
        // greek letter accented, not present in SUBS_I

        /* Since c is alphabetic and does not appear in SUBS_I, the conv(c) function will return
         * c.to_ascii_lowercase()
         * For 'ῶ', the ASCII conversion does not change it and will remain unchanged.
         */
        let expected = '-';
        let output = conv(input);

        assert_eq!(output, expected, "Accented letter not present in SUBS_I is not ");
    }

    #[test]
    fn test_spaces_in_string() {
        let input = "Hello World Rust";
        /*
         * According to the rules, impermissible spaces are replaced by '-', and letters are
         * converted to lower case, avoiding double '-'.
         */

        let expected = "hello-world-rust";
        let output = slugify(input);

        assert_eq!(output, expected, "Conversion of a string with multiple space-separated words is incorrect");
    }

    #[test]
    fn test_string_with_accented_chars() {
        let input = "Caffè Espressö à la française";
        /*
         * Conversion example:
         * "Coffee" -> "coffee"
         * "Espressö" -> "espresso"
         * "à" -> "a"
         * spaces -> "-"
         * If there is any character not expected in SUBS_I, we replace it with '-'
         * So for example we might expect: "caffe-espresso-a-la-francaise"
         */
        let expected = "caffe-espresso-a-la-francaise";
        let output = slugify(input);

        assert_eq!(output, expected, "Conversion of a string with accented characters is incorrect");
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        /*
         * If the string is empty, we can expect slugify to return ""
         */
        let expected = "";
        let output = slugify(input);

        assert_eq!(output, expected, "Empty string not handled correctly")
    }

    #[test]
    fn test_multiple_spaces() {
        let input = "Hello   World   Rust";
        /*
         * According to the rules:
         * - Each space is converted to '-'
         * - We avoid double '-'. If multiple consecutive spaces would create "--", the code reduces it to '-'
         * So we expect multiple consecutive spaces to become one
         */
        let expected = "hello-world-rust";
        let output = slugify(input);

        assert_eq!(output, expected, "String with multiple consecutive spaces was not handled correctly");
    }

    #[test]
    fn test_multiple_invalid_chars() {
        let input = "Hello@@##!!World";
        /*
         * Every invalid character (neither alphanumeric nor in SUBS_I) becomes '-'
         * If there are more than one consecutive ones, the function avoids generating "--" but
         * reduces to a single '-'
         * So we expect:
         * "Hello@@##!!World" -> "hello-world"
         */
        let expected = "hello-world";
        let output = slugify(input);

        assert_eq!(output, expected, "String with multiple consecutive invalid chars was not handled correctly");
    }

    #[test]
    fn test_only_invalid_chars() {
        let input = "@@@@####!!!!!";
        /*
         * According to the rules:
         * All these invalid characters are converted to '-'
         * If the function avoids double '-'s, the final string should become a single '-'
         * But if the code keeps a hyphen for each one, combining them then into one, it remains '-'
         */
        let expected = "-";
        let output = slugify(input);

        assert_eq!(output, expected, "All-invalid string was not handled correctly")
    }

    #[test]
    fn test_trailing_space() {
        let input = "Hello ";
        /*
         * According to the rules:
         * - Space is replaced by '-'
         * - If the final string has more than one character and ends with '-', we remove itì
         * "Hello " -> "hello-" -> "hello" (because we remove the final hyphen, being > 1 chars)
         */
        let expected = "hello";
        let output = slugify(input);

        assert_eq!(output, expected, "Trailing space not handled correctly")
    }

    #[test]
    fn test_multiple_invalid_chars_at_the_end() {
        let input = "Hello!!!???";
        /*
         * - All invalid characters are converted to '-'
         * - Consecutive dashes are reduced to one
         * - If the resulting string has more than one character and ends with '-', we remove it
         * So, starting with "hello!!!??" -> "hello--??" -> "hello-" -> then, with the rule, "hello"
         * (removing the final hyphen if the length > 1)
         */
        let expected = "hello";
        let output = slugify(input);

        assert_eq!(output, expected, "Trailing invalid characters not handled correctly");
    }
}

fn main() {
    let args = Args::parse();

    print!("String to slugify: {}\n", args.slug_in);

    let result = slugify(&args.slug_in);

    print!("Slugified string: {}", result);
}
