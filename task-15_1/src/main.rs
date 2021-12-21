use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use std::io::BufRead;
use std::usize;
use nalgebra::{DMatrix};

fn read_lines_until_empty() -> Vec<String> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| String::from(line.unwrap_or(String::from("")).trim()))
        .take_while(|line| !line.is_empty())
        .collect()
}

fn read_input() -> DMatrix<u32> {
    let lines = read_lines_until_empty();
    let (number_of_rows, number_of_columns) = (lines.len(), lines[0].len());
    DMatrix::from_iterator(
        number_of_rows,
        number_of_columns,
        lines
            .iter()
            .flat_map(|line| {
                line.chars()
                    .map(move |c| (c.to_digit(10).unwrap()))
            })).transpose()
}

struct Entry {
    potential_cost: u32,
    previous_index: (usize, usize),
    current_index: (usize, usize),
}

impl Entry {
    fn new(potential_cost: u32,
           previous_index: (usize, usize),
           current_index: (usize, usize)) -> Entry {
        Entry {
            potential_cost,
            previous_index,
            current_index,
        }
    }
}

impl Eq for Entry {}

impl PartialEq<Self> for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.potential_cost == other.potential_cost
    }
}

impl PartialOrd<Self> for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.potential_cost.partial_cmp(&self.potential_cost)
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.potential_cost.cmp(&self.potential_cost)
    }
}

const DELTAS: [(usize, usize); 4] = [
    (0_usize.wrapping_sub(1), 0),
    (0, 0 + 1),
    (0 + 1, 0),
    (0, 0_usize.wrapping_sub(1)),
];

fn main() {
    let numbers = read_input();
    let mut cost = DMatrix::<Option<u32>>::repeat(numbers.nrows(), numbers.ncols(), None);
    let mut prev = cost.map(|_| None);
    let mut q = BinaryHeap::new();
    q.push(Entry::new(0, (0, 0), (0, 0)));
    while let Some(entry) = q.pop() {
        let Entry {
            potential_cost: c,
            previous_index: p,
            current_index: index @ (row, column),
        } = entry;
        if cost[index].is_some() {
            continue;
        }
        cost[index] = Some(c);
        prev[index] = Some(p);
        let neighbors: Vec<(usize, usize)> = DELTAS
            .iter()
            .copied()
            .map(|(delta_row, delta_column)| (row.wrapping_add(delta_row), column.wrapping_add(delta_column)))
            .filter(|i|
                cost.get(*i)
                    .map(|c| c.is_none())
                    .unwrap_or(false))
            .collect();
        let c = cost.get(index).unwrap().unwrap();
        for i in neighbors {
            let t = numbers.get(i).unwrap();
            q.push(Entry::new(c + t, index, i));
        }
    }
    let mut path = Vec::new();
    let mut current = (numbers.nrows() - 1, numbers.ncols() - 1);
    let mut answer = 0;
    while current != (0, 0) {
        path.push(current);
        answer += numbers[current];
        current = prev[current].unwrap();
    }

    // for row in 0..numbers.nrows() {
    //     for col in 0..numbers.ncols() {
    //         let t = cost[(row, col)].unwrap();
    //         if path.contains(&(row, col)) {
    //             print!("{:2}* ", t);
    //         } else {
    //             print!("{:2}  ", t);
    //         }
    //     }
    //     println!();
    // }

    dbg!(answer);
}
