use std::collections::HashSet;
use std::io::BufRead;
use itertools::Itertools;

enum Fold {
    AlongX(i32),
    AlongY(i32),
}

fn read_lines_until_empty() -> Vec<String> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| String::from(line.unwrap_or(String::from("")).trim()))
        .take_while(|line| !line.is_empty())
        .collect()
}

fn fold(n: i32, d: i32) -> i32 {
    if n < d {
        n
    } else {
        2 * d - n
    }
}

fn main() {
    let mut stars: Vec<(i32, i32)> =
        read_lines_until_empty()
            .iter()
            .map(|line| line
                .split(",")
                .map(|part| part.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
            )
            .collect();
    let commands: Vec<Fold> =
        read_lines_until_empty()
            .iter()
            .map(|line| {
                let (axis, d) = line
                    .trim_start_matches("fold along ")
                    .split("=")
                    .collect_tuple()
                    .unwrap();
                let d = d.parse::<i32>().unwrap();
                match axis.trim() {
                    "x" => { Fold::AlongX(d) }
                    "y" => { Fold::AlongY(d) }
                    _ => panic!("Illegal axis")
                }
            })
            .collect();
    for command in commands.iter().take(1) {
        match command {
            Fold::AlongX(d) => {
                for (x, _) in stars.iter_mut() {
                    *x = fold(*x, *d);
                }
            }
            Fold::AlongY(d) => {
                for (_, y) in stars.iter_mut() {
                    *y = fold(*y, *d);
                }
            }
        }
    }
    let result = HashSet::<(i32, i32)>::from_iter(stars);
    dbg!(result);
}
