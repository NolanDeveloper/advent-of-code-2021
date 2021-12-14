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

fn display(mut stars: Vec<(i32, i32)>) {
    let left_most = stars.iter().map(|x| x.0).min().unwrap();
    let top_most = stars.iter().map(|x| x.1).min().unwrap();
    let right_most = stars.iter().map(|x| x.0).max().unwrap();
    let bottom_most = stars.iter().map(|x| x.1).max().unwrap();
    for (x, y) in stars.iter_mut() {
        *x -= left_most;
        *y -= top_most;
    }
    let stars = HashSet::<(i32, i32)>::from_iter(stars);
    (0..=(bottom_most - top_most))
        .map(|row| {
            (0..=(right_most - left_most))
                .map(|column| if stars.contains(&(column, row)) { '#' } else { '.' })
                .collect()
        })
        .for_each(|line: String| println!("{}", line));
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
    for command in commands {
        match command {
            Fold::AlongX(d) => {
                for (x, _) in stars.iter_mut() {
                    *x = fold(*x, d);
                }
            }
            Fold::AlongY(d) => {
                for (_, y) in stars.iter_mut() {
                    *y = fold(*y, d);
                }
            }
        }
    }
    display(stars.clone());
}
