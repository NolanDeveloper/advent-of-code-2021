use std::io::Read;
use ndarray::arr1;
use nom;

#[derive(Clone)]
enum Direction {
    Forward,
    Up,
    Down,
}

struct Command {
    direction: Direction,
    x: i32,
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Command>> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{i32, multispace1, newline};
    use nom::combinator::{map, value};
    use nom::multi::separated_list0;
    use nom::sequence::tuple;
    let forward = value(Direction::Forward, tag("forward"));
    let up = value(Direction::Up, tag("up"));
    let down = value(Direction::Down, tag("down"));
    let direction = alt((forward, up, down));
    let line = map(tuple((direction, multispace1, i32)), |x| Command { direction: x.0, x: x.2 });
    let mut parser = separated_list0(newline, line);
    parser(input)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (_, result) = parse(input.as_str()).unwrap();
    let mut position = arr1(&[0, 0, 0]);
    let mut aim = 0;
    for command in result {
        match command.direction {
            Direction::Forward => {
                position += &arr1(&[command.x, 0, command.x * aim]);
            }
            Direction::Up => {
                aim -= command.x;
            }
            Direction::Down => {
                aim += command.x;
            }
        }
    }
    println!("{}", position[0] * position[2]);
}
