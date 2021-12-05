use std::io::BufRead;

fn main() {
    let mut n = 0;
    let mut counts = vec!();
    loop {
        let mut line = String::new();
        let n_chars = std::io::stdin().lock().read_line(&mut line).unwrap();
        if n_chars == 0 {
            break;
        }
        for (i, c) in line.chars().enumerate() {
            if counts.len() <= i {
                counts.resize(i + 1, 0);
            }
            if c == '1' {
                counts[i] += 1;
            }
        }
        n += 1;
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    counts.resize(counts.len() - 2, 0);
    for c in counts {
        gamma = gamma * 2;
        epsilon = epsilon * 2;
        if c > n / 2 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    println!("{}", gamma * epsilon);
}
