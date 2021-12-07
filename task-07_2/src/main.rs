use std::io::BufRead;

fn required_fuel(distance: i32) -> i32 {
    distance * (distance + 1) / 2
}

fn main() {
    let mut crab_coordinates =
        std::io::stdin()
            .lock()
            .lines()
            .nth(0)
            .unwrap()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
    crab_coordinates.sort();
    let left = *crab_coordinates.first().unwrap();
    let right = *crab_coordinates.last().unwrap();
    let mut least_fuel = i32::MAX;
    for coordinate in left..(right + 1) {
        let mut fuel = 0;
        for crab_coordinate in &crab_coordinates {
            fuel += required_fuel((coordinate - crab_coordinate).abs());
        }
        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }
    dbg!(least_fuel);
}
