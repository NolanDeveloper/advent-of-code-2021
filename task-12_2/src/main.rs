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
    let edges: Vec<(String, String)> = read_lines_until_empty()
        .iter()
        .map(|line| line.split("-").map(String::from).collect_tuple().unwrap())
        .collect();
    let caves: Vec<String> = edges.iter()
        .flat_map(|(from, to)| [from.as_str(), to.as_str()])
        .unique()
        .sorted_unstable()
        .map(|cave| String::from(cave))
        .collect();
    let index_of_start_cave = caves.iter().position(|cave| *cave == "start").unwrap();
    let index_of_end_cave = caves.iter().position(|cave| *cave == "end").unwrap();
    let adjacency_list: Vec<Vec<usize>> = caves
        .iter()
        .cloned()
        .map(|cave| edges
            .iter()
            .flat_map(|(from, to)|
                if cave == from.as_str() {
                    Some(caves.iter().position(|cave| *cave == to.as_str()).unwrap())
                } else if cave == to.as_str() {
                    Some(caves.iter().position(|cave| *cave == from.as_str()).unwrap())
                } else {
                    None
                }
            ).collect())
        .collect();
    let mut path = Vec::new();
    let answer = count_paths_from(
        &caves,
        &mut path,
        &adjacency_list,
        index_of_end_cave,
        index_of_start_cave,
    );
    dbg!(answer);
}

fn count_paths_from(
    caves: &Vec<String>,
    path: &mut Vec<usize>,
    adjacency_list: &Vec<Vec<usize>>,
    index_of_end_cave: usize,
    index_of_current_cave: usize,
) -> u32 {
    if index_of_end_cave == index_of_current_cave {
        return 1;
    }
    let is_small = caves[index_of_current_cave].chars().all(char::is_lowercase);
    let is_already_visited = path.iter().contains(&index_of_current_cave);
    if is_small && is_already_visited {
        let is_some_small_cave_already_visited_twice =
            path
                .iter()
                .filter(|&&i|
                    caves[i]
                        .chars()
                        .all(char::is_lowercase)
                )
                .counts()
                .values()
                .any(|&c| c == 2);
        if is_some_small_cave_already_visited_twice || caves[index_of_current_cave] == "start" {
            return 0;
        }
    }
    path.push(index_of_current_cave);
    let mut number_of_paths = 0;
    for &index_of_adjacent_cave in &adjacency_list[index_of_current_cave] {
        number_of_paths += count_paths_from(
            caves,
            path,
            adjacency_list,
            index_of_end_cave,
            index_of_adjacent_cave,
        );
    }
    path.pop();
    number_of_paths
}
