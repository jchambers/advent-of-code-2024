use std::error::Error;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let onsen = Onsen::from_str(fs::read_to_string(path)?.as_str())?;

        println!("Possible patterns: {}", onsen.possible_patterns());

        println!(
            "Total possible arrangements: {}",
            onsen.total_possible_arrangements()
        );

        Ok(())
    } else {
        Err("Usage: day19 INPUT_FILE_PATH".into())
    }
}

struct Onsen {
    towels: Vec<String>,
    patterns: Vec<String>,
}

impl Onsen {
    pub fn possible_patterns(&self) -> usize {
        self.patterns
            .iter()
            .filter(|pattern| Self::possible_arrangements(pattern, &self.towels) > 0)
            .count()
    }

    pub fn total_possible_arrangements(&self) -> u64 {
        self.patterns
            .iter()
            .map(|pattern| Self::possible_arrangements(pattern, &self.towels))
            .sum()
    }

    fn possible_arrangements<T: AsRef<str>>(pattern: &str, towels: &[T]) -> u64 {
        let mut paths = vec![0u64; pattern.len() + 1];
        paths[0] = 1;

        for i in 1..=pattern.len() {
            for towel in towels {
                let towel = towel.as_ref();

                // If there isn't enough space for this towel, just keep moving
                if i < towel.len() {
                    continue;
                }

                if &pattern[i - towel.len()..i] == towel {
                    paths[i] += paths[i - towel.len()];
                }
            }
        }

        paths[paths.len() - 1]
    }
}

impl FromStr for Onsen {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((towels, patterns)) = s.split_once("\n\n") {
            let towels: Vec<String> = towels.split(", ").map(String::from).collect();
            let patterns = patterns.lines().map(String::from).collect();

            Ok(Onsen { towels, patterns })
        } else {
            Err("Could not parse towels".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_ONSEN: &str = indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    "};

    #[test]
    fn test_possible_arrangements() {
        let onsen = Onsen::from_str(TEST_ONSEN).unwrap();
        let expected_possible_arrangements = vec![2, 1, 4, 6, 0, 1, 2, 0];

        for i in 0..expected_possible_arrangements.len() {
            assert_eq!(
                expected_possible_arrangements[i],
                Onsen::possible_arrangements(&onsen.patterns[i], &onsen.towels)
            );
        }
    }
}
