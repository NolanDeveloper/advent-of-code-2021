use std::collections::HashMap;
use std::io::BufRead;
use itertools::Itertools;

fn read_lines_until_empty() -> Vec<String> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| String::from(line.unwrap_or(String::from("")).trim()))
        .take_while(|line| !line.is_empty())
        .collect()
}

fn main() {
    let polymer_template = read_lines_until_empty().remove(0);
    let rules =
        HashMap::<(char, char), char>::from_iter(
            read_lines_until_empty()
                .iter()
                .map(|line| {
                    let (left, right) = line
                        .split(" -> ")
                        .map(str::trim)
                        .collect_tuple()
                        .unwrap();
                    let a = left.chars().nth(0).unwrap();
                    let b = left.chars().nth(1).unwrap();
                    let t = right.chars().nth(0).unwrap();
                    ((a, b), t)
                })
        );
    let mut pairs =
        polymer_template
            .chars()
            .zip(polymer_template.chars().skip(1))
            .counts();
    for _ in 0..40 {
        let mut new_pairs = HashMap::<(char, char), usize>::new();
        for ((a, b), n) in pairs {
            if let Some(&t) = rules.get(&(a, b)) {
                *new_pairs.entry((a, t)).or_insert(0) += n;
                *new_pairs.entry((t, b)).or_insert(0) += n;
            }
        }
        pairs = new_pairs;
    }
    let mut letter_counts = HashMap::new();
    for ((a, b), n) in pairs {
        *letter_counts.entry(a).or_insert(0) += n;
        *letter_counts.entry(b).or_insert(0) += n;
    }
    for (_, n) in letter_counts.iter_mut() {
        *n = (*n + 1) / 2;
    }
    let most_common = letter_counts.values().max().unwrap();
    let least_common = letter_counts.values().min().unwrap();
    let answer = most_common - least_common;
    dbg!(most_common);
    dbg!(least_common);
    dbg!(answer);
}
