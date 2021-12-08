use std::collections::HashMap;
use std::io::BufRead;
use itertools::Itertools;

fn resolve(inputs: &Vec<String>, display: &Vec<String>) -> u32 {
    let digits = HashMap::from([
        (String::from("abcefg"), 0),
        (String::from("cf"), 1),
        (String::from("acdeg"), 2),
        (String::from("acdfg"), 3),
        (String::from("bcdf"), 4),
        (String::from("abdfg"), 5),
        (String::from("abdefg"), 6),
        (String::from("acf"), 7),
        (String::from("abcdefg"), 8),
        (String::from("abcdfg"), 9),
    ]);
    let ps = "abcdefg".chars().permutations(7);
    // let ps = Option::from("deafgbc".chars().collect::<Vec<char>>());
    let permutation =
        ps
            .map(|p| HashMap::from_iter(p.iter().cloned().zip("abcdefg".chars()).collect::<Vec<(char, char)>>()))
            .filter(|permutation: &HashMap<char, char>| {
                inputs.iter().all(|input| {
                    let mask: String = input.chars().map(|c| String::from(*permutation.get(&c).unwrap())).sorted().collect();
                    digits.contains_key(mask.as_str())
                })
            }).nth(0).unwrap();
    let mut result = 0;
    for number in display {
        result *= 10;
        result += digits[&number.chars().map(|c| String::from(permutation[&c])).sorted().collect::<String>()];
    }
    result
}

fn main() {
    let input_lines: Vec<String> = std::io::stdin().lock().lines()
        .map(|line| line.ok().filter(|line| !line.is_empty()))
        .take_while(Option::is_some)
        .map(Option::unwrap)
        .collect();
    let data: Vec<(Vec<String>, Vec<String>)> = input_lines.iter()
        .map(|line|
            line.split("|").map(|part| part.split_whitespace().map(String::from).collect()).collect_tuple().unwrap()
        ).collect();
    let n: u32 = data.iter().map(|(a, b)| resolve(a, b)).sum();
    dbg!(n);
}
