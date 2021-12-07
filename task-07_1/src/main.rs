use std::io::BufRead;

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
    let leftmost = *crab_coordinates.iter().min().unwrap();
    let n = crab_coordinates.len();
    let mut least_fuel = i32::MAX;
    let mut fuel = crab_coordinates.iter().map(|x| *x - leftmost).sum::<i32>();
    for i in 1..n {
        let delta = crab_coordinates[i] - crab_coordinates[i - 1];
        fuel += delta * (2 * (i as i32) - n as i32);
        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }
    dbg!(least_fuel);
}
