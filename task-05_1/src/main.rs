use std::io::BufRead;
use std::mem::swap;

fn read_input() -> std::io::Result<Vec<(usize, usize, usize, usize)>> {
    let mut result = Vec::new();
    loop {
        let mut line = String::new();
        let n = std::io::stdin().lock().read_line(&mut line)?;
        if n == 0 {
            break;
        }
        let parts: Vec<usize> = line
            .split("->")
            .flat_map(|x| x.split(","))
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect();
        let x1 = parts[0];
        let y1 = parts[1];
        let x2 = parts[2];
        let y2 = parts[3];
        result.push((x1, y1, x2, y2));
    }
    Ok(result)
}


fn main() {
    let data = read_input().unwrap();
    let mut map = Vec::new();
    {
        let mut empty_row = Vec::new();
        for _ in 0..1000 {
            empty_row.push(0);
        }
        for _ in 0..1000 {
            map.push(empty_row.clone());
        }
    }
    for (x1, y1, x2, y2) in data {
        if x1 == x2 {
            let mut y1 = y1;
            let mut y2 = y2;
            if y2 < y1 {
                swap(&mut y1, &mut y2);
            }
            for i in y1..(y2 + 1) {
                map[x1][i] += 1;
            }
        } else if y1 == y2 {
            let mut x1 = x1;
            let mut x2 = x2;
            if x2 < x1 {
                swap(&mut x1, &mut x2);
            }
            for i in x1..(x2 + 1) {
                map[i][y1] += 1;
            }
        }
    }
    // let most_dangerous = map.iter().flat_map(|x| x.iter()).max().unwrap();
    let count_dangerous = map.iter().flat_map(|x| x.iter()).filter(|x| **x >= 2).count();
    println!("{}", count_dangerous);
}
