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
    let mut left = *crab_coordinates.first().unwrap();
    let mut right = *crab_coordinates.last().unwrap() + 1;
    let mut least_fuel = i32::MAX;
    while (right - left) > 2 {
        let coordinate_a = left + (right - left) / 3;
        let coordinate_b = left + (right - left) * 2 / 3;
        let mut fuel_a = 0;
        let mut fuel_b = 0;
        for crab_coordinate in &crab_coordinates {
            fuel_a += required_fuel((coordinate_a - crab_coordinate).abs());
            fuel_b += required_fuel((coordinate_b - crab_coordinate).abs());
        }
        if fuel_a < fuel_b {
            right = coordinate_b;
            least_fuel = i32::min(least_fuel, fuel_a);
        } else {
            left = coordinate_a;
            least_fuel = i32::min(least_fuel, fuel_b);
        }
    }
    dbg!(least_fuel);
}
