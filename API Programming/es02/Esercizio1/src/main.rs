// paste this file into main.rs

use std::{env, fs};

fn stats(text: &str) -> [u32; 26] {
    let mut counts = [0u32; 26];
    let mut c;

    for character in text.chars() {
        c = character.to_ascii_lowercase();

        if c >= 'a' && c <= 'z' {
            counts[c as usize - 'a' as usize] += 1;
        }
    }

    if is_pangram(&counts) {
        println!("{} is pangram", text);
    }
    else {
        println!("{} is not pangram", text);
    }

    let mut i = 0;
    for count in counts {
        println!("{} {}", (b'a' + i) as char, count);
        i += 1;
    }

    counts
}

fn is_pangram(counts: &[u32]) -> bool {
    if counts.len() != 26 {
        return false;
    }

    for &count in counts {
        if count == 0 {
            return false;
        }
    }
    true
}

// call this function from main
// load here the contents of the file
pub fn run_pangram() {
    // retrieve arguments from command line
    let args: Vec<String> = env::args().collect();

    // checks that at least one argument (the file name) is passed
    if args.len() < 2 {
        eprint!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];

    // reads all the contents of the file into a String
    let contents = fs::read_to_string(filename).unwrap();

    stats(&contents);
    return;
}


// please note, code has been split in simple functions in order to make testing easier

#[cfg(test)] // this is a test module
mod tests
{
    // tests are separated modules, you must import the code you are testing
    use super::*;

    #[test]
    fn test_all_ones() {
        let counts = [1; 26];
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_some_zeros() {
        let mut counts = [0; 26];
        counts[0] = 0;
        counts[1] = 0;
        assert!(!is_pangram(&counts));
    }

    #[test]
    fn test_increasing_counts() {
        let mut counts = [0; 26];
        for i in 0..26 {
            counts[i] = i as u32 + 1;
        }
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_wrong_size()  {
        let counts = [1; 25];
        assert!(!is_pangram(&counts));
    }

    #[test]
    fn test_stats_on_full_alphabet() {
        let counts = stats("abcdefghijklmnopqrstuvwxyz");
        for c in counts {
            assert!(c == 1);
        }
    }

    #[test]
    fn test_stats_on_empty_string() {
        let counts = stats("");
        for c in counts {
            assert!(c == 0);
        }
    }

    #[test]
    fn test_stats_missing_char() {
        let counts = stats("abcdefghijklmnopqrstuvwxy");
        for c in counts.iter().take(25) {
            assert!(*c == 1);
        }
        assert!(counts[25] == 0);

    }

    #[test]
    fn test_stats_on_full_tring() {
        let contents = "The quick brown fox jumps over the lazy dog";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_stats_with_punctuation() {
        let contents = "The quick brown fox jumps over the lazy dog!";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_missing_char_on_full_string() {
        let contents = "The quick brown fox jumps over the laz* dog";
        let counts = stats(contents);
        println!("{:?}", counts);
        for (i, c) in counts.iter().enumerate() {
            if i == 24 {
                assert!(*c == 0);
            } else {
                assert!(*c > 0);
            }

        }
    }

    #[test]
    fn test_is_pangram() {
        let counts = stats("The quick brown fox jumps over the lazy dog");
        assert!(is_pangram(&counts));
    }
}

fn main() {
    run_pangram();
}

