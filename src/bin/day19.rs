use std::error::Error;
use std::iter::repeat_with;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let onsen = Onsen::from_str(fs::read_to_string(path)?.as_str())?;
        let possible_arrangements = onsen.possible_arrangements_by_towel();

        println!(
            "Possible patterns: {}",
            possible_arrangements
                .iter()
                .filter(|&&arrangements| arrangements > 0)
                .count()
        );

        println!(
            "Total possible arrangements: {}",
            possible_arrangements.iter().sum::<u64>()
        );

        Ok(())
    } else {
        Err("Usage: day19 INPUT_FILE_PATH".into())
    }
}

struct Onsen {
    towels_by_length: Vec<Vec<String>>,
    min_towel_length: usize,

    patterns: Vec<String>,
}

impl Onsen {
    pub fn new(towels: Vec<String>, patterns: Vec<String>) -> Self {
        let min_towel_length = towels.iter().map(|t| t.len()).min().unwrap();
        let max_towel_length = towels.iter().map(|t| t.len()).max().unwrap();

        let mut towels_by_length: Vec<Vec<String>> =
            repeat_with(Vec::new).take(max_towel_length + 1).collect();

        towels
            .into_iter()
            .for_each(|towel| towels_by_length[towel.len()].push(towel));

        Onsen {
            towels_by_length,
            min_towel_length,

            patterns,
        }
    }

    pub fn possible_arrangements_by_towel(&self) -> Vec<u64> {
        self.patterns
            .iter()
            .map(|pattern| self.possible_arrangements(pattern))
            .collect()
    }

    fn possible_arrangements(&self, pattern: &str) -> u64 {
        let mut paths = vec![0u64; pattern.len() + 1];
        paths[0] = 1;

        for i in self.min_towel_length..=pattern.len() {
            for towel_len in self.min_towel_length..=(self.towels_by_length.len() - 1).min(i) {
                let prefix = &pattern[i - towel_len..i];

                if self.towels_by_length[towel_len]
                    .iter()
                    .any(|towel| towel == prefix)
                {
                    paths[i] += paths[i - towel_len];
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

            Ok(Onsen::new(towels, patterns))
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
        let expected_possible_arrangements: Vec<u64> = vec![2, 1, 4, 6, 0, 1, 2, 0];

        assert_eq!(
            expected_possible_arrangements,
            onsen.possible_arrangements_by_towel()
        );
    }
}
