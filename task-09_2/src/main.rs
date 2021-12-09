use std::collections::HashMap;
use std::io::BufRead;
use itertools::Itertools;
use ndarray::{Array2};


fn main() {
    let lines: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| String::from(line.unwrap_or(String::from("")).trim()))
        .take_while(|line| !line.is_empty())
        .collect();

    let shape = (lines.len(), lines[0].len());
    let heights = lines.iter()
        .flat_map(|line| line.trim().chars().map(|c| c.to_digit(10).unwrap()))
        .collect();
    let height_map = Array2::from_shape_vec(shape, heights).unwrap();
    let mut basin_map = Array2::<Option<(usize, usize)>>::default(height_map.dim());
    for (index @ (row, column), height) in height_map.indexed_iter() {
        if *height == 9 {
            continue;
        }
        // flood fill
        let mut stack = Vec::new();
        stack.push((row as i32, column as i32));
        while !stack.is_empty() {
            let (row, column) = stack.pop().unwrap();
            let basin_index = (row as usize, column as usize);
            if basin_map.get(basin_index).unwrap_or(&Some((0, 0))).is_some()
                || height_map[basin_index] == 9 {
                continue;
            }
            basin_map[basin_index] = Some(index);
            let up = (row as i32 - 1, column as i32);
            let down = (row as i32 + 1, column as i32);
            let left = (row as i32, column as i32 - 1);
            let right = (row as i32, column as i32 + 1);
            for near_index in [up, right, down, left] {
                stack.push(near_index);
            }
        }
    }
    let mut basin_sizes = HashMap::new();
    for x in basin_map.iter().flatten() {
        *basin_sizes.entry(*x).or_insert(0) += 1;
    }
    dbg!(basin_sizes.values().sorted_unstable().rev().take(3).product::<i32>());
}
