use std::io::{BufRead, stdin};

fn main()  {
    let mut prev = [i32::MAX / 3; 3];
    let mut number_of_increases = 0;
    for n in stdin().lock().lines().map(|x| str::parse::<i32>(&x.unwrap()).unwrap()) {
        let prev_avg = prev[0] + prev[1] + prev[2];
        let cur_avg = prev[1] + prev[2] + n;
        let increased = prev_avg < cur_avg;
        if increased {
            number_of_increases += 1;
        }
        println!("{} ({})", n, if increased { "increased" } else { "decreased" });
        prev[0] = prev[1];
        prev[1] = prev[2];
        prev[2] = n;
    }
    println!("{}", number_of_increases);
}
