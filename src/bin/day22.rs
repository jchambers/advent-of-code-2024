use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let monkey_market = MonkeyMarket {
            secret_numbers: BufReader::new(File::open(path)?)
                .lines()
                .map_while(Result::ok)
                .map(|line| line.parse())
                .collect::<Result<Vec<_>, _>>()?,
        };

        println!(
            "Secret number sum after 2000 iterations: {}",
            monkey_market.secret_number_sum(2000)
        );

        println!(
            "Max bananas after 2000 iterations: {}",
            monkey_market.max_bananas(2000)
        );

        Ok(())
    } else {
        Err("Usage: day22 INPUT_FILE_PATH".into())
    }
}

struct MonkeyMarket {
    secret_numbers: Vec<u64>,
}

impl MonkeyMarket {
    pub fn secret_number_sum(&self, iterations: usize) -> u64 {
        self.secret_numbers
            .iter()
            .map(|&seed| {
                SecretNumbers::new(seed)
                    .skip(1)
                    .take(iterations)
                    .last()
                    .unwrap()
            })
            .sum()
    }

    pub fn max_bananas(&self, iterations: usize) -> u32 {
        const SEQUENCE_LENGTH: usize = 4;
        const DISTINCT_SEQUENCES: usize = 19usize.pow(SEQUENCE_LENGTH as u32);

        let mut max_bananas_by_sequence = vec![0; DISTINCT_SEQUENCES];

        self.secret_numbers
            .iter()
            .map(|&seed| {
                SecretNumbers::new(seed)
                    .take(iterations + 1)
                    .map(|n| (n % 10) as i8)
                    .collect::<Vec<i8>>()
            })
            .for_each(|prices| {
                let deltas: Vec<i8> = prices
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect();

                let mut encountered_sequences = vec![false; DISTINCT_SEQUENCES];

                deltas
                    .windows(SEQUENCE_LENGTH)
                    .zip(prices.iter().skip(SEQUENCE_LENGTH))
                    .for_each(|(sequence, &price)| {
                        let packed_sequence = Self::pack_sequence(sequence) as usize;

                        if !encountered_sequences[packed_sequence] {
                            encountered_sequences[packed_sequence] = true;
                            max_bananas_by_sequence[packed_sequence] += price as u32;
                        }
                    });
            });

        *max_bananas_by_sequence.iter().max().unwrap()
    }

    fn pack_sequence(sequence: &[i8]) -> u32 {
        sequence
            .iter()
            .fold(0u32, |acc, &i| acc * 19 + ((i + 9) as u32))
    }
}

struct SecretNumbers {
    n: u64,
}

impl SecretNumbers {
    fn new(n: u64) -> SecretNumbers {
        SecretNumbers { n }
    }
}

impl Iterator for SecretNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.n;

        self.n ^= self.n << 6;
        self.n &= 0xffffff;
        self.n ^= self.n >> 5;
        self.n &= 0xffffff;
        self.n ^= self.n << 11;
        self.n &= 0xffffff;

        Some(next)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_secret_numbers() {
        assert_eq!(
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254
            ],
            SecretNumbers::new(123)
                .skip(1)
                .take(10)
                .collect::<Vec<u64>>()
        );
    }

    #[test]
    fn test_secret_number_sum() {
        let monkey_market = MonkeyMarket {
            secret_numbers: vec![1, 10, 100, 2024],
        };
        assert_eq!(37327623, monkey_market.secret_number_sum(2000));
    }

    #[test]
    fn test_max_bananas() {
        let monkey_market = MonkeyMarket {
            secret_numbers: vec![1, 2, 3, 2024],
        };
        assert_eq!(23, monkey_market.max_bananas(2000));
    }
}
