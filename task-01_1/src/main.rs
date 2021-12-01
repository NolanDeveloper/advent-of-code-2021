use std::io::{BufRead, stdin};

fn main()  {
    let mut prev = i32::MAX;
    let mut number_of_increases = 0;
    for n in stdin().lock().lines().map(|x| str::parse::<i32>(&x.unwrap()).unwrap()) {
        let increased = prev < n;
        if increased {
            number_of_increases += 1;
        }
        println!("{} ({})", n, if increased { "increased" } else { "decreased" });
        prev = n;
    }
    println!("{}", number_of_increases);
}
