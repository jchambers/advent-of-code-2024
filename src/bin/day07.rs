use crate::Operator::{Add, Concat, Multiply};
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

        println!(
            "Sum of test values from valid equations with add/multiply: {}",
            valid_calibration_sum(&calibration_equations, &[Add, Multiply])
        );

        println!(
            "Sum of test values from valid equations with add/multiply/concat: {}",
            valid_calibration_sum(&calibration_equations, &[Add, Multiply, Concat])
        );

        Ok(())
    } else {
        Err("Usage: day07 INPUT_FILE_PATH".into())
    }
}

fn valid_calibration_sum(
    calibration_equations: &[CalibrationEquation],
    allowed_operators: &[Operator],
) -> u64 {
    calibration_equations
        .iter()
        .filter_map(|equation| {
            if equation.is_possible(allowed_operators) {
                Some(equation.test_value)
            } else {
                None
            }
        })
        .sum()
}

struct CalibrationEquation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl CalibrationEquation {
    fn is_possible(&self, allowed_operators: &[Operator]) -> bool {
        let mut stack = vec![(0, self.numbers[0])];

        while let Some((level, total)) = stack.pop() {
            // All operators embiggen the total; if we've already overshot the target, stop
            // exploring the branch.
            if total > self.test_value {
                continue;
            }

            if level == self.numbers.len() - 1 {
                if total == self.test_value {
                    return true;
                }
            } else {
                let next = self.numbers[level + 1];

                stack.extend(allowed_operators.iter().map(|operator| {
                    let next_total = match operator {
                        Add => total + next,
                        Multiply => total * next,
                        Concat => (total * 10u64.pow(next.ilog10() + 1)) + next,
                    };

                    (level + 1, next_total)
                }));
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
                .map(|number| number.parse::<u64>())
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

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_possible() {
        assert!(CalibrationEquation::from_str("190: 10 19")
            .unwrap()
            .is_possible(&[Add, Multiply]));

        assert!(CalibrationEquation::from_str("3267: 81 40 27")
            .unwrap()
            .is_possible(&[Add, Multiply]));

        assert!(!CalibrationEquation::from_str("83: 17 5")
            .unwrap()
            .is_possible(&[Add, Multiply]));

        assert!(!CalibrationEquation::from_str("156: 15 6")
            .unwrap()
            .is_possible(&[Add, Multiply]));

        assert!(!CalibrationEquation::from_str("7290: 6 8 6 15")
            .unwrap()
            .is_possible(&[Add, Multiply]));

        assert!(!CalibrationEquation::from_str("161011: 16 10 13")
            .unwrap()
            .is_possible(&[Add, Multiply]));

        assert!(!CalibrationEquation::from_str("192: 17 8 14")
            .unwrap()
            .is_possible(&[Add, Multiply]));

        assert!(!CalibrationEquation::from_str("21037: 9 7 18 13")
            .unwrap()
            .is_possible(&[Add, Multiply]));

        assert!(CalibrationEquation::from_str("292: 11 6 16 20")
            .unwrap()
            .is_possible(&[Add, Multiply]));
    }

    #[test]
    fn test_is_possible_with_concat() {
        assert!(CalibrationEquation::from_str("190: 10 19")
            .unwrap()
            .is_possible(&[Add, Multiply, Concat]));

        assert!(CalibrationEquation::from_str("3267: 81 40 27")
            .unwrap()
            .is_possible(&[Add, Multiply, Concat]));

        assert!(!CalibrationEquation::from_str("83: 17 5")
            .unwrap()
            .is_possible(&[Add, Multiply, Concat]));

        assert!(CalibrationEquation::from_str("156: 15 6")
            .unwrap()
            .is_possible(&[Add, Multiply, Concat]));

        assert!(CalibrationEquation::from_str("7290: 6 8 6 15")
            .unwrap()
            .is_possible(&[Add, Multiply, Concat]));

        assert!(!CalibrationEquation::from_str("161011: 16 10 13")
            .unwrap()
            .is_possible(&[Add, Multiply, Concat]));

        assert!(CalibrationEquation::from_str("192: 17 8 14")
            .unwrap()
            .is_possible(&[Add, Multiply, Concat]));

        assert!(!CalibrationEquation::from_str("21037: 9 7 18 13")
            .unwrap()
            .is_possible(&[Add, Multiply, Concat]));

        assert!(CalibrationEquation::from_str("292: 11 6 16 20")
            .unwrap()
            .is_possible(&[Add, Multiply, Concat]));
    }
}
