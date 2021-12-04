use std::io::Read;
use nom;
use ndarray::{arr1, Array1};

fn parse(input: &str) -> nom::IResult<&str, Vec<Array1<i32>>> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{i32, multispace1, newline};
    use nom::combinator::{map, value};
    use nom::multi::separated_list0;
    use nom::sequence::tuple;
    let forward = value(ndarray::arr1(&[1, 0, 0]), tag("forward"));
    let up = value(ndarray::arr1(&[0, 0, -1]), tag("up"));
    let down = value(ndarray::arr1(&[0, 0, 1]), tag("down"));
    let direction = alt((forward, up, down));
    let line = map(tuple((direction, multispace1, i32)), |x| { x.0 * x.2 });
    let mut parser = separated_list0(newline, line);
    parser(input)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (_, result) = parse(input.as_str()).unwrap();
    let mut position = arr1(&[0, 0, 0]);
    for dir in result {
        position += &dir;
    }
    println!("{}", position[0] * position[2]);
}
