use std::error::Error;
use std::str::FromStr;
use std::{env, fs};

const BUTTON_A_TOKENS: i64 = 3;
const BUTTON_B_TOKENS: i64 = 1;

const UNIT_CORRECTION: i64 = 10000000000000;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        println!(
            "Min tokens to win all possible prizes: {}",
            ClawMachine::machines_from_str(fs::read_to_string(path)?.as_str())?
                .iter()
                .map(|machine| machine.min_tokens_to_win().unwrap_or(0))
                .sum::<u64>()
        );

        println!(
            "Min tokens to win all possible prizes with unit correction: {}",
            ClawMachine::machines_from_str_with_unit_correction(
                fs::read_to_string(path)?.as_str(),
                UNIT_CORRECTION
            )?
            .iter()
            .map(|machine| machine.min_tokens_to_win().unwrap_or(0))
            .sum::<u64>()
        );

        Ok(())
    } else {
        Err("Usage: day13 INPUT_FILE_PATH".into())
    }
}

type Vector2d = (i64, i64);

struct ClawMachine {
    buttons: [Vector2d; 2],
    prize: Vector2d,
}

impl ClawMachine {
    pub fn machines_from_str(s: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        Self::machines_from_str_with_unit_correction(s, 0)
    }

    pub fn machines_from_str_with_unit_correction(
        s: &str,
        unit_correction: i64,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        s.split("\n\n")
            .map(ClawMachine::from_str)
            .map(|result| {
                result.map(|machine| ClawMachine {
                    buttons: machine.buttons,
                    prize: (
                        machine.prize.0 + unit_correction,
                        machine.prize.1 + unit_correction,
                    ),
                })
            })
            .collect()
    }

    pub fn min_tokens_to_win(&self) -> Option<u64> {
        let mut min_tokens_to_win: Option<u64> = None;

        let (x_a, y_a) = self.buttons[0];
        let (x_b, y_b) = self.buttons[1];
        let (x_p, y_p) = self.prize;

        let b_presses = ((x_p * y_a) - (x_a * y_p)) / ((x_b * y_a) - (x_a * y_b));
        let a_presses = (x_p - (b_presses * x_b)) / x_a;

        if (a_presses * x_a) + (b_presses * x_b) == x_p
            && (a_presses * y_a) + (b_presses * y_b) == y_p
        {
            let tokens = (a_presses * BUTTON_A_TOKENS) + (b_presses * BUTTON_B_TOKENS);

            min_tokens_to_win = min_tokens_to_win
                .map(|t| t.min(tokens as u64))
                .or(Some(tokens as u64))
        }

        min_tokens_to_win
    }

    fn button_from_str(s: &str) -> Result<Vector2d, Box<dyn Error>> {
        if !s.starts_with("Button ") {
            return Err("Button line must start with 'Button: '".into());
        }

        // Button A: X+12, Y+25
        if let Some(label_end) = s.find(": ") {
            if let Some((x, y)) = s[label_end + 2..].split_once(", ") {
                let x = x
                    .strip_prefix("X+")
                    .ok_or("Could not parse X component")?
                    .parse()?;

                let y = y
                    .strip_prefix("Y+")
                    .ok_or("Could not parse Y component")?
                    .parse()?;

                Ok((x, y))
            } else {
                Err("Could not split x/y components".into())
            }
        } else {
            Err("Could not find label end".into())
        }
    }

    fn prize_from_str(s: &str) -> Result<Vector2d, Box<dyn Error>> {
        if let Some(components) = s.strip_prefix("Prize: ") {
            if let Some((x, y)) = components.split_once(", ") {
                let x = x
                    .strip_prefix("X=")
                    .ok_or("Could not parse X component")?
                    .parse()?;

                let y = y
                    .strip_prefix("Y=")
                    .ok_or("Could not parse Y component")?
                    .parse()?;

                Ok((x, y))
            } else {
                Err("Could not split x/y components".into())
            }
        } else {
            Err("Prize line must start with 'Prize: '".into())
        }
    }
}

impl FromStr for ClawMachine {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let &[button_a, button_b, prize] = s.lines().collect::<Vec<&str>>().as_slice() {
            Ok(Self {
                buttons: [
                    Self::button_from_str(button_a)?,
                    Self::button_from_str(button_b)?,
                ],

                prize: Self::prize_from_str(prize)?,
            })
        } else {
            Err("Could not parse machine string".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_MACHINES: &str = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    "};

    #[test]
    fn test_min_tokens_to_win() {
        let claw_machines = ClawMachine::machines_from_str(TEST_MACHINES).unwrap();

        assert_eq!(Some(280), claw_machines[0].min_tokens_to_win());
        assert_eq!(None, claw_machines[1].min_tokens_to_win());
        assert_eq!(Some(200), claw_machines[2].min_tokens_to_win());
        assert_eq!(None, claw_machines[3].min_tokens_to_win());
    }

    #[test]
    fn test_min_tokens_to_win_unit_correction() {
        let claw_machines =
            ClawMachine::machines_from_str_with_unit_correction(TEST_MACHINES, UNIT_CORRECTION)
                .unwrap();

        assert!(claw_machines[0].min_tokens_to_win().is_none());
        assert!(claw_machines[1].min_tokens_to_win().is_some());
        assert!(claw_machines[2].min_tokens_to_win().is_none());
        assert!(claw_machines[3].min_tokens_to_win().is_some());
    }
}
