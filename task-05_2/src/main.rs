use std::io::BufRead;

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
        for _ in 0..1001 {
            empty_row.push(0);
        }
        for _ in 0..1001 {
            map.push(empty_row.clone());
        }
    }
    for (x1, y1, x2, y2) in data {
        let dx = i32::signum((x2 as i32) - (x1 as i32));
        let dy = i32::signum((y2 as i32) - (y1 as i32));
        let (mut x, mut y) = (x1, y1);
        while !(x == x2 && y == y2) {
            map[x][y] += 1;
            x = ((x as i32) + dx) as usize;
            y = ((y as i32) + dy) as usize;
        }
        map[x][y] += 1;
    }
    let count_dangerous = map.iter().flat_map(|x| x.iter()).filter(|x| **x >= 2).count();
    println!("{}", count_dangerous);
}
