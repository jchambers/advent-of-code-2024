use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let seeds: Vec<u64> = BufReader::new(File::open(path)?)
            .lines()
            .map_while(Result::ok)
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?;

        println!(
            "Secret number sum after 2000 iterations: {}",
            secret_number_sum(&seeds, 2000)
        );

        Ok(())
    } else {
        Err("Usage: day22 INPUT_FILE_PATH".into())
    }
}

fn secret_number_sum(seeds: &[u64], iterations: usize) -> u64 {
    seeds
        .iter()
        .map(|&seed| SecretNumbers::new(seed).take(iterations).last().unwrap())
        .sum()
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
        self.n ^= self.n << 6;
        self.n &= 0xffffff;
        self.n ^= self.n >> 5;
        self.n &= 0xffffff;
        self.n ^= self.n << 11;
        self.n &= 0xffffff;

        Some(self.n)
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
            SecretNumbers::new(123).take(10).collect::<Vec<u64>>()
        );
    }

    #[test]
    fn test_secret_number_sum() {
        assert_eq!(37327623, secret_number_sum(&[1, 10, 100, 2024], 2000));
    }
}
