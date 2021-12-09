use std::io::BufRead;
use ndarray::Array2;

fn main() {
    let lines: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap_or(String::from("")))
        .take_while(|line| !line.is_empty())
        .collect();

    let shape = (lines.len(), lines[0].len());
    let heights = lines.iter()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();
    let height_map = Array2::from_shape_vec(shape, heights).unwrap();

    let mut risk_level = 0;
    for row in 0..height_map.nrows() as i32 {
        for column in 0..height_map.ncols() as i32 {
            let height = height_map.get((row as usize, column as usize)).cloned().unwrap();
            let mut is_low_point = true;
            'outer: for near_row in row - 1..row + 2 {
                for near_column in column - 1..column + 2 {
                    if (near_row, near_column) == (row, column) {
                        continue;
                    }
                    let index = (near_row as usize, near_column as usize);
                    let near_height = height_map.get(index).cloned().unwrap_or(u32::MAX);
                    if near_height <= height {
                        is_low_point = false;
                        break 'outer;
                    }
                }
            }
            if is_low_point {
                risk_level += height + 1;
            }
        }
    }
    dbg!(risk_level);
}
