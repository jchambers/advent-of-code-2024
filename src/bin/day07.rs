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
            calibration_equations
                .iter()
                .filter_map(|equation| {
                    if equation.is_possible(&[Add, Multiply]) {
                        Some(equation.test_value)
                    } else {
                        None
                    }
                })
                .sum::<u64>()
        );

        println!(
            "Sum of test values from valid equations with add/multiply/concat: {}",
            calibration_equations
                .iter()
                .filter_map(|equation| {
                    if equation.is_possible(&[Add, Multiply, Concat]) {
                        Some(equation.test_value)
                    } else {
                        None
                    }
                })
                .sum::<u64>()
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
    fn is_possible(&self, allowed_operators: &[Operator]) -> bool {
        if self.numbers.is_empty() {
            return false;
        }

        if self.numbers.len() == 1 {
            return self.numbers[0] as u64 == self.test_value;
        }

        OperatorSequence::new(self.numbers.len() - 1, allowed_operators)
            .any(|operators| Self::evaluate(&self.numbers, &operators) == self.test_value)
    }

    fn evaluate(numbers: &[u32], operators: &[Operator]) -> u64 {
        if numbers.len() != operators.len() + 1 {
            panic!("Mismatched operators/numbers lengths");
        }

        let mut result: u64 = numbers[0] as u64;

        for i in 0..operators.len() {
            match operators[i] {
                Add => result += numbers[i + 1] as u64,
                Multiply => result *= numbers[i + 1] as u64,
                Concat => {
                    for _ in 0..=numbers[i + 1].ilog10() {
                        result *= 10;
                    }

                    result += numbers[i + 1] as u64
                }
            }
        }

        result
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

struct OperatorSequence {
    len: usize,
    allowed_operators: Vec<Operator>,
    sequence: Vec<Operator>,
    stack: Vec<(usize, Operator)>,
}

impl OperatorSequence {
    pub fn new(len: usize, allowed_operators: &[Operator]) -> Self {
        Self {
            len,
            allowed_operators: allowed_operators.to_vec(),
            sequence: vec![allowed_operators[0]; len],
            stack: allowed_operators
                .iter()
                .map(|&operator| (0, operator))
                .collect(),
        }
    }
}

impl Iterator for OperatorSequence {
    type Item = Vec<Operator>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((level, operator)) = self.stack.pop() {
            self.sequence[level] = operator;

            if level == self.len - 1 {
                // We've hit the bottom of the "tree" and should yield a result
                return Some(self.sequence.clone());
            } else {
                // We're somewhere in the middle and should keep exploring
                self.allowed_operators
                    .iter()
                    .for_each(|&operator| self.stack.push((level + 1, operator)));
            }
        }

        None
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
