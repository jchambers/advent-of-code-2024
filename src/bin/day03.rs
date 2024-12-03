use std::cmp::min;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let corrupted_memory = {
            let mut string = String::new();
            File::open(path)?.read_to_string(&mut string)?;

            string
        };

        println!(
            "Sum of products: {}",
            multiplication_sum(&find_operations(&corrupted_memory))
        );

        println!(
            "Sum of products with stateful evaluation: {}",
            multiplication_sum_with_state(&find_operations(&corrupted_memory))
        );

        Ok(())
    } else {
        Err("Usage: day03 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Do,
    DoNot,
    Multiply(u32, u32),
}

enum ParserState {
    FindOperatorStart,
    ParseMultiplicand,
    ExpectComma,
    ExpectOperatorEnd,
}

fn find_operations(corrupted_memory: &str) -> Vec<Operation> {
    let mut state = ParserState::FindOperatorStart;
    let mut remainder = corrupted_memory;
    let mut multiplicands = [0u32; 2];
    let mut found_first_multiplicand = false;
    let mut operations = Vec::new();

    while !remainder.is_empty() {
        match state {
            ParserState::FindOperatorStart => {
                found_first_multiplicand = false;

                let mut next_operator: Option<(usize, &str)> = None;

                for candidate in ["do()", "don't()", "mul("] {
                    if let Some(token_start) = remainder.find(candidate) {
                        if next_operator.is_none() || next_operator.unwrap().0 > token_start {
                            next_operator = Some((token_start, candidate));
                        }
                    }
                }

                match next_operator {
                    Some((token_start, token)) => {
                        match token {
                            "do()" => operations.push(Operation::Do),
                            "don't()" => operations.push(Operation::DoNot),
                            "mul(" => {
                                state = ParserState::ParseMultiplicand;
                            }
                            _ => unreachable!(),
                        }

                        remainder = &remainder[token_start + token.len()..];
                    }
                    None => {
                        // We didn't find a next operator and are done
                        break;
                    }
                }
            }

            ParserState::ParseMultiplicand => {
                let mut parsed = None;
                let mut digits_consumed = 0;

                // Try to parse up to three digits
                for length in 1..=min(3, remainder.len()) {
                    if let Ok(multiplicand) = remainder[..length].parse() {
                        parsed = Some(multiplicand);
                        digits_consumed = length;
                    } else {
                        // Bail out as soon as we hit a bad digit
                        break;
                    }
                }

                if let Some(multiplicand) = parsed {
                    remainder = &remainder[digits_consumed..];

                    if found_first_multiplicand {
                        multiplicands[1] = multiplicand;
                        state = ParserState::ExpectOperatorEnd;
                    } else {
                        found_first_multiplicand = true;
                        multiplicands[0] = multiplicand;
                        state = ParserState::ExpectComma;
                    }
                } else {
                    // We didn't find a valid number; don't consume any digits and start looking for
                    // the next candidate.
                    state = ParserState::FindOperatorStart;
                }
            }

            ParserState::ExpectComma => {
                if remainder.starts_with(",") {
                    state = ParserState::ParseMultiplicand;
                    remainder = &remainder[1..];
                } else {
                    // The next character wasn't a comma, and so this isn't a valid operation;
                    // search for other candidates instead
                    state = ParserState::FindOperatorStart;
                }
            }

            ParserState::ExpectOperatorEnd => {
                if remainder.starts_with(")") {
                    operations.push(Operation::Multiply(multiplicands[0], multiplicands[1]));
                    remainder = &remainder[1..];
                }

                // Whether or not we found a valid operation, we're done with this candidate. On to
                // the next one!
                state = ParserState::FindOperatorStart;
            }
        }
    }

    operations
}

fn multiplication_sum(operations: &[Operation]) -> u32 {
    operations
        .iter()
        .map(|operation| match operation {
            Operation::Multiply(a, b) => a * b,
            _ => 0,
        })
        .sum::<u32>()
}

fn multiplication_sum_with_state(operations: &[Operation]) -> u32 {
    let mut evaluate_multiplication = true;
    let mut sum = 0;

    operations.iter().for_each(|operation| match operation {
        Operation::Do => evaluate_multiplication = true,
        Operation::DoNot => evaluate_multiplication = false,
        Operation::Multiply(a, b) => {
            if evaluate_multiplication {
                sum += a * b;
            }
        }
    });

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_multiplications() {
        assert_eq!(
            vec![
                Operation::Multiply(2, 4),
                Operation::Multiply(5, 5),
                Operation::Multiply(11, 8),
                Operation::Multiply(8, 5),
            ],
            find_operations(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )
        );

        assert_eq!(
            vec![Operation::Multiply(123, 456)],
            find_operations("mul(123,456)")
        );

        assert_eq!(
            vec![
                Operation::Multiply(2, 4),
                Operation::DoNot,
                Operation::Multiply(5, 5),
                Operation::Multiply(11, 8),
                Operation::Do,
                Operation::Multiply(8, 5),
            ],
            find_operations(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )
        )
    }

    #[test]
    fn test_multiplication_sum() {
        assert_eq!(
            161,
            multiplication_sum(&find_operations(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            ))
        );
    }

    #[test]
    fn test_multiplication_sum_with_state() {
        assert_eq!(
            48,
            multiplication_sum_with_state(&find_operations(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ))
        );
    }
}
