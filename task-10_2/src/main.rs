use std::collections::HashMap;
use std::io::BufRead;
use bimap::{BiMap};
use itertools::Itertools;

enum SyntaxError {
    Mismatch { expected: char, found: char },
    Incomplete { not_matched: Vec<char> },
}

/// returns (expected, found)
fn check_syntax(line: &String) -> Option<SyntaxError> {
    let brackets = BiMap::from_iter(vec![
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);
    let mut not_matched = Vec::new();
    for c in line.chars() {
        if brackets.contains_left(&c) {
            not_matched.push(c);
        } else {
            let last_not_matched = not_matched.pop().unwrap();
            let expected = *brackets.get_by_left(&last_not_matched).unwrap();
            if expected != c {
                return Some(SyntaxError::Mismatch { expected, found: c });
            }
        }
    }
    if !not_matched.is_empty() {
        return Some(SyntaxError::Incomplete { not_matched });
    }
    None
}

fn read_lines_until_empty() -> Vec<String> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| String::from(line.unwrap_or(String::from("")).trim()))
        .take_while(|line| !line.is_empty())
        .collect()
}

fn main() {
    let table = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);
    let input = read_lines_until_empty();
    let scores: Vec<i64> = input.iter().map(|line| {
        let syntax_error = check_syntax(line);
        match syntax_error {
            Some(SyntaxError::Incomplete { not_matched }) => {
                not_matched.iter().rev().map(|c| { table.get(c).unwrap() }).fold(0, |c, x| c * 5 + x)
            }
            _ => { 0 }
        }
    }).filter(|s| *s != 0).sorted_unstable().collect();
    dbg!(scores[scores.len() / 2]);
}
