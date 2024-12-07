use crate::Operator::{Add, Multiply};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let calibration_equations = BufReader::new(File::open(path)?)
            .lines()
            .map_while(Result::ok)
            .map(|line| CalibrationEquation::from_str(line.as_str()))
            .collect::<Result<Vec<CalibrationEquation>, _>>()?;

        let valid_calibration_sum: u64 = calibration_equations
            .iter()
            .filter_map(|equation| {
                if equation.is_possible() {
                    Some(equation.test_value)
                } else {
                    None
                }
            })
            .sum();

        println!(
            "Sum of test values from valid equations: {}",
            valid_calibration_sum
        );

        Ok(())
    } else {
        Err("Usage: day07 INPUT_FILE_PATH".into())
    }
}

struct CalibrationEquation {
    test_value: u64,
    numbers: Vec<u32>,
}

impl CalibrationEquation {
    fn is_possible(&self) -> bool {
        if self.numbers.is_empty() {
            return false;
        }

        if self.numbers.len() == 1 {
            return self.numbers[0] as u64 == self.test_value;
        }

        let mut operators = vec![Add; self.numbers.len() - 1];
        let mut stack = vec![(0, Add), (0, Multiply)];

        while let Some((level, operator)) = stack.pop() {
            operators[level] = operator;

            if level == self.numbers.len() - 2 {
                // We've hit the bottom of the tree and should evaluate
                let mut result: u64 = self.numbers[0] as u64;

                for i in 0..operators.len() {
                    result = match operators[i] {
                        Add => result + self.numbers[i + 1] as u64,
                        Multiply => result * self.numbers[i + 1] as u64,
                    }
                }

                if result == self.test_value {
                    return true;
                }
            } else {
                // Continue exploring
                stack.push((level + 1, Add));
                stack.push((level + 1, Multiply));
            }
        }

        false
    }
}

impl FromStr for CalibrationEquation {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((test_value, numbers)) = s.split_once(": ") {
            let numbers = numbers
                .split_whitespace()
                .map(|number| number.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()?;

            Ok(CalibrationEquation {
                test_value: test_value.parse()?,
                numbers,
            })
        } else {
            Err("Could not parse calibration equation".into())
        }
    }
}

#[derive(Copy, Clone)]
enum Operator {
    Add,
    Multiply,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_possible() {
        assert!(CalibrationEquation::from_str("190: 10 19")
            .unwrap()
            .is_possible());

        assert!(CalibrationEquation::from_str("3267: 81 40 27")
            .unwrap()
            .is_possible());

        assert!(!CalibrationEquation::from_str("83: 17 5")
            .unwrap()
            .is_possible());

        assert!(!CalibrationEquation::from_str("156: 15 6")
            .unwrap()
            .is_possible());

        assert!(!CalibrationEquation::from_str("7290: 6 8 6 15")
            .unwrap()
            .is_possible());

        assert!(!CalibrationEquation::from_str("161011: 16 10 13")
            .unwrap()
            .is_possible());

        assert!(!CalibrationEquation::from_str("192: 17 8 14")
            .unwrap()
            .is_possible());

        assert!(!CalibrationEquation::from_str("21037: 9 7 18 13")
            .unwrap()
            .is_possible());

        assert!(CalibrationEquation::from_str("292: 11 6 16 20")
            .unwrap()
            .is_possible());
    }
}
