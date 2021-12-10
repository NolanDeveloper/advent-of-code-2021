use std::collections::HashMap;
use std::io::BufRead;
use bimap::{BiMap};

/// returns (expected, found)
fn first_mismatch(line: &String) -> Option<(char, char)> {
    let brackets = BiMap::from_iter(vec![
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);
    let mut stack = Vec::new();
    for c in line.chars() {
        if brackets.contains_left(&c) {
            stack.push(*brackets.get_by_left(&c).unwrap());
        } else {
            let t = stack.pop().unwrap_or('_');
            if t != c {
                return Some((*brackets.get_by_right(&t).unwrap(), c));
            }
        }
    }
    stack.pop().map(|c| (c, '$'))
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
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    let input = read_lines_until_empty();
    let score: i32 = input.iter().map(|line| {
        let mismatch = first_mismatch(line);
        table.get(&mismatch.unwrap().1).unwrap_or(&0)
    }).sum();
    dbg!(score);
}
