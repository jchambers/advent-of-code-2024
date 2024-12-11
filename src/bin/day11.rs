use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let mut pebble_line = PebbleLine::from_str(fs::read_to_string(path)?.as_str())?;

        println!(
            "Pebbles after 25 blinks: {}",
            pebble_line.total_pebbles_after_blinks(25)
        );

        println!(
            "Pebbles after 75 blinks: {}",
            pebble_line.total_pebbles_after_blinks(75)
        );

        Ok(())
    } else {
        Err("Usage: day11 INPUT_FILE_PATH".into())
    }
}

#[derive(Clone)]
struct PebbleLine {
    pebbles: Vec<u64>,
    cache: HashMap<(u64, usize), u64>,
}

impl PebbleLine {
    pub fn total_pebbles_after_blinks(&mut self, blinks: usize) -> u64 {
        self.pebbles
            .clone()
            .iter()
            .map(|&pebble| self.pebbles_after_blinks(pebble, blinks))
            .sum()
    }

    fn pebbles_after_blinks(&mut self, pebble: u64, blinks: usize) -> u64 {
        if blinks == 0 {
            1
        } else if let Some(&pebbles) = self.cache.get(&(pebble, blinks)) {
            pebbles
        } else {
            let pebbles = if pebble == 0 {
                self.pebbles_after_blinks(1, blinks - 1)
            } else if Self::has_even_decimal_digits(pebble) {
                let (left, right) = Self::split_pebble(pebble);

                self.pebbles_after_blinks(left, blinks - 1)
                    + self.pebbles_after_blinks(right, blinks - 1)
            } else {
                self.pebbles_after_blinks(pebble * 2024, blinks - 1)
            };

            self.cache.insert((pebble, blinks), pebbles);

            pebbles
        }
    }

    fn has_even_decimal_digits(pebble: u64) -> bool {
        pebble.ilog10() % 2 != 0
    }

    fn split_pebble(pebble: u64) -> (u64, u64) {
        let mask = 10u64.pow((pebble.ilog10() + 1) / 2);

        let left = pebble / mask;
        let right = pebble % (left * mask);

        (left, right)
    }
}

impl FromStr for PebbleLine {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pebbles = s
            .split_whitespace()
            .map(|p| p.parse::<u64>())
            .collect::<Result<_, _>>()?;

        Ok(PebbleLine {
            pebbles,
            cache: HashMap::new(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_PEBBLE_LINE: &str = "125 17";

    #[test]
    fn test_has_even_decimal_digits() {
        assert!(!PebbleLine::has_even_decimal_digits(1));
        assert!(PebbleLine::has_even_decimal_digits(12));
        assert!(!PebbleLine::has_even_decimal_digits(123));
    }

    #[test]
    fn test_split_pebble() {
        assert_eq!((1, 2), PebbleLine::split_pebble(12));
        assert_eq!((123, 456), PebbleLine::split_pebble(123456));
    }

    #[test]
    fn test_total_pebbles_after_blinks() {
        let mut pebble_line = PebbleLine::from_str(TEST_PEBBLE_LINE).unwrap();

        assert_eq!(22, pebble_line.total_pebbles_after_blinks(6));
        assert_eq!(55312, pebble_line.total_pebbles_after_blinks(25));
    }
}
