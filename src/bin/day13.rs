use std::error::Error;
use std::ops::{Add, Mul};
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let claw_machines = ClawGame::machines_from_str(fs::read_to_string(path)?.as_str())?;

        println!(
            "Min tokens to win all possible prizes: {}",
            claw_machines
                .iter()
                .map(|machine| machine.min_tokens_to_win().unwrap_or(0))
                .sum::<u32>()
        );

        Ok(())
    } else {
        Err("Usage: day13 INPUT_FILE_PATH".into())
    }
}

struct ClawGame {
    buttons: [Vector2d; 2],
    prize: Vector2d,
}

impl ClawGame {
    pub fn machines_from_str(s: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        s.split("\n\n").map(ClawGame::from_str).collect()
    }

    pub fn min_tokens_to_win(&self) -> Option<u32> {
        let mut min_tokens_to_win: Option<u32> = None;

        for a in 0..=self.prize.x / self.buttons[0].x {
            let b = (self.prize.x - (a * self.buttons[0].x)) / self.buttons[1].x;

            if &self.buttons[0] * a + &self.buttons[1] * b == self.prize {
                let tokens = (a * 3) + b;

                min_tokens_to_win = min_tokens_to_win
                    .map(|t| t.min(tokens as u32))
                    .or(Some(tokens as u32))
            }
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

                Ok(Vector2d { x, y })
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

                Ok(Vector2d { x, y })
            } else {
                Err("Could not split x/y components".into())
            }
        } else {
            Err("Prize line must start with 'Prize: '".into())
        }
    }
}

impl FromStr for ClawGame {
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

#[derive(Eq, PartialEq)]
struct Vector2d {
    x: i32,
    y: i32,
}

impl Mul<i32> for &Vector2d {
    type Output = Vector2d;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add<Vector2d> for Vector2d {
    type Output = Self;

    fn add(self, rhs: Vector2d) -> Self::Output {
        Vector2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
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
        let claw_machines = ClawGame::machines_from_str(TEST_MACHINES).unwrap();

        assert_eq!(Some(280), claw_machines[0].min_tokens_to_win());
        assert_eq!(None, claw_machines[1].min_tokens_to_win());
        assert_eq!(Some(200), claw_machines[2].min_tokens_to_win());
        assert_eq!(None, claw_machines[3].min_tokens_to_win());
    }
}
