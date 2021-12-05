use std::io::BufRead;

fn read_input() -> std::io::Result<Vec<String>> {
    let mut result = Vec::new();
    loop {
        let mut line = String::new();
        let n = std::io::stdin().lock().read_line(&mut line)?;
        if n == 0 {
            break;
        }
        result.push(line.chars().filter(|x| *x == '0' || *x == '1').collect());
    }
    Ok(result)
}

enum BitCriteria {
    Oxygen,
    Co2,
}

fn rating(mut data: Vec<String>, bit_criteria: BitCriteria) -> String {
    let n = data[0].len();
    for i in 0..n {
        let number_of_ones = data.iter().filter(|x| x.chars().nth(i).unwrap() == '1').count();
        let number_of_zeros = data.len() - number_of_ones;
        let digit_to_keep = match bit_criteria {
            BitCriteria::Oxygen => {
                if number_of_zeros <= number_of_ones { '1' } else { '0' }
            }
            BitCriteria::Co2 => {
                if number_of_zeros <= number_of_ones { '0' } else { '1' }
            }
        };
        data = data.iter().filter(|x| x.chars().nth(i).unwrap() == digit_to_keep).cloned().collect();
        assert_ne!(data.len(), 0);
        if data.len() == 1 {
            break;
        }
    }
    assert_eq!(data.len(), 1);
    data.remove(0)
}

fn main() {
    let data = read_input().unwrap();
    let oxygen_rating = rating(data.clone(), BitCriteria::Oxygen);
    let co2_rating = rating(data, BitCriteria::Co2);
    let oxygen_rating_number = i32::from_str_radix(oxygen_rating.as_str(), 2).unwrap();
    let co2_rating_number = i32::from_str_radix(co2_rating.as_str(), 2).unwrap();
    println!("{}", oxygen_rating_number * co2_rating_number);
}
