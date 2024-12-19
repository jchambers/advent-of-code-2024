use std::error::Error;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let onsen = Onsen::from_str(fs::read_to_string(path)?.as_str())?;

        println!("Possible patterns: {}", onsen.possible_patterns());

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
            .filter(|pattern| Self::is_possible_pattern(pattern, &self.towels))
            .count()
    }

    fn is_possible_pattern<T: AsRef<str>>(pattern: &str, towels: &[T]) -> bool {
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

        paths[paths.len() - 1] > 0
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
    fn test_is_possible_pattern() {
        let onsen = Onsen::from_str(TEST_ONSEN).unwrap();

        assert!(Onsen::is_possible_pattern(
            &onsen.patterns[0],
            &onsen.towels
        ));

        assert!(Onsen::is_possible_pattern(
            &onsen.patterns[1],
            &onsen.towels
        ));

        assert!(Onsen::is_possible_pattern(
            &onsen.patterns[2],
            &onsen.towels
        ));

        assert!(Onsen::is_possible_pattern(
            &onsen.patterns[3],
            &onsen.towels
        ));

        assert!(!Onsen::is_possible_pattern(
            &onsen.patterns[4],
            &onsen.towels
        ));

        assert!(Onsen::is_possible_pattern(
            &onsen.patterns[5],
            &onsen.towels
        ));

        assert!(Onsen::is_possible_pattern(
            &onsen.patterns[6],
            &onsen.towels
        ));

        assert!(!Onsen::is_possible_pattern(
            &onsen.patterns[7],
            &onsen.towels
        ));
    }
}
