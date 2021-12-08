use std::io::BufRead;
use itertools::Itertools;

fn main() {
    let data =
        std::io::stdin()
            .lock()
            .lines()
            .flatten()
            .map(|x|
                x.split("|").map(|y| y.split_whitespace().map(String::from).collect()).collect_tuple()
            ).take_while(Option::is_some)
            .map(Option::unwrap)
            .collect::<Vec<(Vec<String>, Vec<String>)>>();
    let n = data
        .iter()
        .flat_map(|x| x.1.iter())
        .filter(|x| match x.len() { 2 | 3 | 4 | 7 => true, _ => false })
        .count();
    dbg!(n);
}
