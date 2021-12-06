fn read_input() -> std::io::Result<Vec<i32>> {
    let mut line = String::new();
    let n = std::io::stdin().read_line(&mut line)?;
    assert_ne!(n, 0);
    Ok(line.split(",").map(|x| x.trim().parse::<i32>().unwrap()).collect())
}

fn display(data: &Vec<i32>) -> String {
    data.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
}

fn main() {
    let mut data = read_input().unwrap();
    println!("Initial state: {}", display(&data));
    for day in 1..81 {
        let mut new_fish = 0;
        for a in data.iter_mut() {
            if *a == 0 {
                *a = 6;
                new_fish += 1;
            } else {
                *a -= 1;
            }
        }
        for _ in 0..new_fish {
            data.push(8);
        }
        println!("After {:2} day{:1}: {}", day, if day != 1 { "s" } else { "" }, data.len());
    }
}
