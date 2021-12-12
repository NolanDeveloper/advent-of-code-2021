use std::collections::VecDeque;
use std::fmt::Write;
use std::io::BufRead;
use ndarray::Array2;

fn step(energies: &mut Array2<u32>) {
    let mut need_to_highlight: VecDeque<(usize, usize)> = VecDeque::new();
    for (index, e) in energies.indexed_iter_mut() {
        *e += 1;
        if *e > 9 {
            need_to_highlight.push_back(index);
        }
    }
    while let Some(index @ (row, column)) = need_to_highlight.pop_front() {
        if energies[index] == 0 {
            continue;
        }
        energies[index] = 0;
        for neighbor_row in (row as i32 - 1)..(row as i32 + 2) {
            for neighbor_column in (column as i32 - 1)..(column as i32 + 2) {
                if (neighbor_row, neighbor_column) == (row as i32, column as i32) {
                    continue;
                }
                let neighbor_index = (neighbor_row as usize, neighbor_column as usize);
                for e in energies.get_mut(neighbor_index) {
                    if *e != 0 {
                        *e += 1;
                        if *e > 9 {
                            need_to_highlight.push_back(neighbor_index)
                        }
                    }
                }
            }
        }
    }
}

fn read_lines_until_empty() -> Vec<String> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| String::from(line.unwrap_or(String::from("")).trim()))
        .take_while(|line| !line.is_empty())
        .collect()
}

fn display(array: &Array2<u32>) -> String {
    let mut result = String::new();
    for row in array.rows() {
        for value in row {
            write!(&mut result, "{}", value).expect("not written");
        }
        writeln!(&mut result).expect("not written");
    }
    result
}

fn main() {
    let input = read_lines_until_empty();
    let number_of_rows = input.len();
    let number_of_columns = input[0].len();
    let mut data: Array2<u32> = Array2::zeros((number_of_rows, number_of_columns));
    for (row, line) in input.iter().enumerate() {
        for (column, char) in line.chars().enumerate() {
            data[(row, column)] = char.to_digit(10).unwrap();
        }
    }
    println!("Before any steps:");
    println!("{}", display(&data));
    let mut number_of_flashes = 0;
    for i in 0..100 {
        step(&mut data);
        number_of_flashes += data.iter().filter(|e| **e == 0).count();
        println!("After step {}", i + 1);
        println!("{}", display(&data));
    }
    dbg!(number_of_flashes);
}
