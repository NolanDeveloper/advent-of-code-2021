fn read_input() -> std::io::Result<Vec<i64>> {
    let mut line = String::new();
    let n = std::io::stdin().read_line(&mut line)?;
    assert_ne!(n, 0);
    Ok(line.split(",").map(|x| x.trim().parse::<i64>().unwrap()).collect())
}

fn display(data: &Vec<i64>) -> String {
    data.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
}

fn main() {
    let mut data = read_input().unwrap();
    println!("Initial state: {}", display(&data));
    let mut counts = [0; 9];
    for x in data {
        counts[x as usize] += 1i64;
    }
    for day in 1..257i64 {
        let zero = counts[0];
        for i in 0..8 {
            counts[i] = counts[i + 1];
        }
        counts[8] = zero;
        counts[6] += zero;
        println!("After {:2} day{:1}: {}", day, if day != 1 { "s" } else { "" }, counts.iter().sum::<i64>());
    }
}
