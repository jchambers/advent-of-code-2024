use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let reports = BufReader::new(File::open(path)?)
            .lines()
            .map_while(Result::ok)
            .map(|line| Report::from_str(line.as_str()))
            .collect::<Result<Vec<Report>, _>>()?;

        println!(
            "Safe reports: {}",
            reports.iter().filter(|report| report.is_safe()).count()
        );

        println!(
            "Safe reports with problem dampener: {}",
            reports
                .iter()
                .filter(|report| report.is_safe_with_problem_dampener())
                .count()
        );

        Ok(())
    } else {
        Err("Usage: day02 INPUT_FILE_PATH".into())
    }
}

struct Report {
    levels: Vec<u32>,
}

impl Report {
    pub fn is_safe(&self) -> bool {
        Report::is_level_sequence_safe(&self.levels)
    }

    pub fn is_safe_with_problem_dampener(&self) -> bool {
        self.is_safe()
            || (0..self.levels.len()).any(|i| {
                let mut modified_levels = self.levels.clone();
                modified_levels.remove(i);

                Report::is_level_sequence_safe(&modified_levels)
            })
    }

    fn is_level_sequence_safe(levels: &[u32]) -> bool {
        let is_monotonically_increasing = levels.windows(2).all(|pair| pair[0] <= pair[1]);
        let is_monotonically_decreasing = levels.windows(2).all(|pair| pair[0] >= pair[1]);
        let is_step_size_safe = levels
            .windows(2)
            .all(|pair| (1..=3).contains(&pair[0].abs_diff(pair[1])));

        (is_monotonically_increasing || is_monotonically_decreasing) && is_step_size_safe
    }
}

impl FromStr for Report {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_whitespace()
            .map(|level| level.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .map(|levels| Self { levels })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_safe() {
        assert!(Report::from_str("7 6 4 2 1").unwrap().is_safe());
        assert!(!Report::from_str("1 2 7 8 9").unwrap().is_safe());
        assert!(!Report::from_str("9 7 6 2 1").unwrap().is_safe());
        assert!(!Report::from_str("1 3 2 4 5").unwrap().is_safe());
        assert!(!Report::from_str("8 6 4 4 1").unwrap().is_safe());
        assert!(Report::from_str("1 3 6 7 9").unwrap().is_safe());
    }
    
    #[test]
    fn test_is_safe_with_problem_dampener() {
        assert!(Report::from_str("7 6 4 2 1").unwrap().is_safe_with_problem_dampener());
        assert!(!Report::from_str("1 2 7 8 9").unwrap().is_safe_with_problem_dampener());
        assert!(!Report::from_str("9 7 6 2 1").unwrap().is_safe_with_problem_dampener());
        assert!(Report::from_str("1 3 2 4 5").unwrap().is_safe_with_problem_dampener());
        assert!(Report::from_str("8 6 4 4 1").unwrap().is_safe_with_problem_dampener());
        assert!(Report::from_str("1 3 6 7 9").unwrap().is_safe_with_problem_dampener());
    }
}
