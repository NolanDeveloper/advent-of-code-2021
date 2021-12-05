use std::io::{BufRead, stdin};

#[derive(Debug)]
struct Data {
    numbers: Vec<i32>,
    cards: Vec<Vec<Vec<i32>>>,
}

fn read_input() -> std::io::Result<Data> {
    let mut line = String::new();
    stdin().lock().read_line(&mut line)?;
    line = line.trim().to_string();
    let numbers = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let mut cards = Vec::new();
    loop {
        let n = stdin().lock().read_line(&mut line)?;
        if n == 0 {
            break;
        }
        let mut card = Vec::new();
        for _ in 0..5 {
            line = String::new();
            stdin().lock().read_line(&mut line)?;
            line = line.trim().to_string();
            let row: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
            if row.len() == 0 {
                break;
            }
            assert_eq!(row.len(), 5);
            card.push(row);
        }
        assert_eq!(card.len(), 5);
        cards.push(card);
    }
    Ok(Data { numbers, cards })
}

fn main() {
    let data = read_input().unwrap();
    let mut permutation: Vec<(i32, usize)> = data.numbers.iter().enumerate().map(|x| (*x.1, x.0)).collect();
    permutation.sort();
    let permutation: Vec<i32> = permutation.iter().map(|x| x.1 as i32).collect();
    let (n_turns, card_index): (i32, usize) = data.cards.iter().enumerate().map(|(i, card)| {
        let rows = card.iter().map(|x| x.iter().map(|i| permutation[*i as usize]).collect());
        let columns = (0..5).map(|column| card.iter().map(|row| permutation[row[column] as usize]).collect());
        (rows.chain(columns).map(|x: Vec<i32>| *x.iter().max().unwrap()).min().unwrap(), i)
    }).max().unwrap();
    let sum: i32 = data.cards[card_index]
        .iter()
        .flat_map(|x| x.iter())
        .filter(|x| data.numbers
            .iter()
            .take((n_turns + 1) as usize)
            .find(|y| **y == **x)
            .is_none())
        .sum();
    println!("{}: {} * {} = {}", card_index, data.numbers[n_turns as usize], sum, data.numbers[n_turns as usize] * sum);
}
