use std::cmp::{max, min};
use std::io::BufRead;
use itertools::Itertools;

struct BitStream<'a> {
    data: &'a [bool],
    current_bit: usize,
}

impl BitStream<'_> {
    fn new(data: &[bool]) -> BitStream {
        BitStream {
            data,
            current_bit: 0,
        }
    }

    fn extract_bit(&mut self) -> Option<bool> {
        if self.current_bit >= self.data.len() {
            return None;
        }
        let result = Some(self.data[self.current_bit]);
        self.current_bit += 1;
        return result;
    }

    fn extract(&mut self, number_of_bits: usize) -> Option<u64> {
        assert!(number_of_bits <= 64);
        (0..number_of_bits).map(|_| self.extract_bit()).fold_options(0, |acc, x| acc * 2 + x as u64)
    }

    fn substream(&mut self, number_of_bits: usize) -> Option<BitStream> {
        let is_ok = self.current_bit + number_of_bits <= self.data.len();
        if !is_ok {
            println!();
        }
        assert!(is_ok);
        is_ok.then(|| {
            let result = BitStream::new(&self.data[self.current_bit..self.current_bit + number_of_bits]);
            self.current_bit += number_of_bits;
            result
        })
    }
    // sums all version numbers contained inside the packet
    fn parse_packet(&mut self) -> Option<u64> {

        let operations: Vec<fn(u64, u64) -> u64> = vec!(
            |a, b| a + b,
            |a, b| a * b,
            |a, b| min(a, b),
            |a, b| max(a, b),
            |_, _| panic!(),
            |a, b| (a > b) as u64,
            |a, b| (a < b) as u64,
            |a, b| (a == b) as u64,
        );

        let _version = self.extract(3)?;
        let type_id = self.extract(3)?;
        return if type_id == 4 { // literal value
            let mut result = 0;
            loop {
                let portion = self.extract(5)?;
                result = result << 4 | portion & 0b1111;
                if (portion >> 4) & 1 == 0 {
                    break;
                }
            }
            Some(result)
        } else { // operator
            let mut result = None;
            let length_type_id = self.extract_bit()?;
            if !length_type_id {
                let total_length_in_bits = self.extract(15)? as usize;
                let mut substream = self.substream(total_length_in_bits)?;
                loop {
                    match substream.parse_packet() {
                        None => { break; }
                        Some(s) => {
                            match result {
                                None => {
                                    result = Some(s);
                                }
                                Some(t) => {
                                    result = Some(operations[type_id as usize](t, s));
                                }
                            }
                        }
                    }
                }
            } else {
                let number_of_subpackets = self.extract(11)?;
                for _ in 0..number_of_subpackets {
                    let s = self.parse_packet()?;
                    match result {
                        None => {
                            result = Some(s);
                        }
                        Some(t) => {
                            result = Some(operations[type_id as usize](t, s));
                        }
                    }
                }
            }
            result
        }
    }
}

fn main() {
    let input = std::io::stdin().lock().lines().nth(0).unwrap().unwrap();
    let bits = input.chars()
        .flat_map(|c| {
            let digit = c.to_digit(16).unwrap();
            format!("{:04b}", digit).chars().map(|c| c == '1').collect::<Vec<bool>>()
        })
        .collect::<Vec<bool>>();
    let mut bit_stream = BitStream::new(&bits[..]);
    let n = bit_stream.parse_packet().unwrap();
    println!();
    dbg!(n);
}
