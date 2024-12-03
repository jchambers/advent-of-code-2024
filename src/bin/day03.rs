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
            find_multiplications(&corrupted_memory)
                .iter()
                .map(|multiplication| multiplication.evaluate())
                .sum::<u32>()
        );

        Ok(())
    } else {
        Err("Usage: day03 INPUT_FILE_PATH".into())
    }
}

enum ParserState {
    FindOperatorStart,
    ParseMultiplicand,
    ExpectComma,
    ExpectOperatorEnd,
}

fn find_multiplications(corrupted_memory: &str) -> Vec<Multiplication> {
    let mut state = ParserState::FindOperatorStart;
    let mut remainder = corrupted_memory;
    let mut multiplicands = [0u32; 2];
    let mut found_first_multiplicand = false;
    let mut multiplications = Vec::new();

    while !remainder.is_empty() {
        match state {
            ParserState::FindOperatorStart => {
                found_first_multiplicand = false;

                if let Some(next) = remainder.find("mul(") {
                    state = ParserState::ParseMultiplicand;
                    remainder = &remainder[next + 4..];
                } else {
                    // If we can't find any more "start of operation" strings, we're done
                    break;
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
                    multiplications.push(Multiplication::new(multiplicands[0], multiplicands[1]));
                    remainder = &remainder[1..];
                }

                // Whether or not we found a valid operation, we're done with this candidate. On to
                // the next one!
                state = ParserState::FindOperatorStart;
            }
        }
    }

    multiplications
}

#[derive(Debug, Eq, PartialEq)]
struct Multiplication {
    multiplicands: [u32; 2],
}

impl Multiplication {
    fn new(a: u32, b: u32) -> Self {
        Self {
            multiplicands: [a, b],
        }
    }

    fn evaluate(&self) -> u32 {
        self.multiplicands[0] * self.multiplicands[1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_multiplications() {
        assert_eq!(
            vec![
                Multiplication::new(2, 4),
                Multiplication::new(5, 5),
                Multiplication::new(11, 8),
                Multiplication::new(8, 5),
            ],
            find_multiplications(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )
        );

        assert_eq!(
            vec![Multiplication::new(123, 456)],
            find_multiplications("mul(123,456)")
        );
    }
}
