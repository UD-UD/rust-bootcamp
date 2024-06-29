// Rewrite the find_pos function.
// The return type must be Result<usize, Error>.




use std::{fs, io};
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyTextOrPattern, // either text or pattern (or both) is empty string
    TextLenSmall,       // text.len() < pattern.len()
    PatternNotFound,    // pattern not a substring of text
}

// below function finds the starting index of `pattern` in `text`
// if `pattern` is not found, it returns text.len()
pub fn find_pos(text: &str, pattern: &str) -> Result<usize, Error> {
    let pattern_len = pattern.len();
    if (text.is_empty() || pattern.is_empty()) {
        return Err(Error::EmptyTextOrPattern);
    }
    if (text.len() < pattern_len) {
        return Err(Error::TextLenSmall);
    }
    for start in 0..text.len() - pattern_len + 1 {
        if &text[start..start + pattern_len] == pattern {
            return Ok(start);
        }
    }
    return Err(Error::PatternNotFound);
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut strings = String::new();
    File::open(filename)?.read_to_string(&mut strings);
    Ok(strings)
}

fn read_first_line(filename: &str) -> Option<String> {
    fs::read_to_string(filename).ok().and_then(|s| {
        s.lines().next().map(|s| s.to_owned())
    })
}

fn fetch_last<T>(list: &mut Vec<T>) -> Result<T, String> {
    list.pop().ok_or("Empty list".to_owned())
}

fn main2() {
    let mut my_nums = Vec::<i32>::new();
    match fetch_last(&mut my_nums) {
        Ok(ele) => println!("Last element: {ele}"),
        Err(error) => {
            println!("Error: {error}");
            assert_eq!(error, "Empty list".to_owned());
        }
    }
}

// ok() converts a Result type to Option Type
// ok_or() convert Option to Result

fn add(num1: &str, num2: &str) -> Option<u8> {
    // TODO: only modify the 2 statements below
    let num1 = num1.parse::<u8>();
    let num2 = num2.parse::<u8>();
    num1.ok()?.checked_add(num2.ok()?)
}

fn main3() {
    let (num1, num2) = ("4", "5");
    if let Some(sum) = add("4", "5") {
        println!("{num1} + {num2} = {sum}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_strings() {
        assert!(matches!(
            find_pos("", "pattern"),
            Err(Error::EmptyTextOrPattern)
        ));
        assert!(matches!(
            find_pos("text", ""),
            Err(Error::EmptyTextOrPattern)
        ));
        assert!(matches!(find_pos("", ""), Err(Error::EmptyTextOrPattern)));
    }

    #[test]
    fn small_text() {
        assert!(matches!(
            find_pos("hello", "hello there"),
            Err(Error::TextLenSmall)
        ));
    }

    #[test]
    fn pattern_not_present() {
        assert!(matches!(
            find_pos("hello", "bye"),
            Err(Error::PatternNotFound)
        ));
    }

    #[test]
    fn pattern_present() {
        assert!(matches!(find_pos("I luv Rust", "uv"), Ok(3)));
    }
}
