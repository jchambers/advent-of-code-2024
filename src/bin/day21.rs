use std::error::Error;
use std::iter::repeat_n;

fn main() {}

type Position = (usize, usize);

struct Door {}

impl Door {
    pub fn complexity(code: &str, directional_keypads: usize) -> u32 {
        let numeric_keypad = NumericKeypad::default();
        let directional_keypad = DirectionalKeypad::default();

        let mut button_sequence = directional_keypad.button_sequence(code, &numeric_keypad);

        for _ in 1..directional_keypads {
            let symbols: String = button_sequence
                .iter()
                .map(|&button| char::from(button))
                .collect();

            button_sequence = directional_keypad.button_sequence(&symbols, &directional_keypad);
        }

        button_sequence.len() as u32
            * code
                .strip_suffix("A")
                .expect("Code must end with 'A'")
                .parse::<u32>()
                .expect("Code must be parsable as an integer")
    }
}

trait Keypad {
    fn positions(&self, symbols: &str) -> Vec<Position> {
        symbols
            .chars()
            .map(|c| self.position(c).expect("Keypad must have symbol"))
            .collect()
    }

    fn position(&self, symbol: char) -> Option<Position>;
}

#[derive(Default)]
struct NumericKeypad {}

impl Keypad for NumericKeypad {
    fn position(&self, symbol: char) -> Option<Position> {
        match symbol {
            '7' => Some((0, 0)),
            '8' => Some((1, 0)),
            '9' => Some((2, 0)),
            '4' => Some((0, 1)),
            '5' => Some((1, 1)),
            '6' => Some((2, 1)),
            '1' => Some((0, 2)),
            '2' => Some((1, 2)),
            '3' => Some((2, 2)),
            '0' => Some((1, 3)),
            'A' => Some((2, 3)),
            _ => None,
        }
    }
}

#[derive(Default)]
struct DirectionalKeypad {}

impl DirectionalKeypad {
    pub fn button_sequence(
        &self,
        symbols: &str,
        remote_keypad: &impl Keypad,
    ) -> Vec<DirectionalKeypadButton> {
        let mut position = remote_keypad
            .position('A')
            .expect("All keypads must have 'A' key");
        let mut path = Vec::new();

        for target_position in remote_keypad.positions(symbols) {
            // Avoid the "forbidden key." In both keypad layouts, there's no way to move over the
            // forbidden key by moving to the right, so always do that first if we can. If we're
            // moving left (or not moving horizontally at all), we can always avoid the forbidden
            // key by moving vertically first.
            if target_position.0 > position.0 {
                path.extend(repeat_n(
                    DirectionalKeypadButton::Right,
                    target_position.0 - position.0,
                ));
            }

            if target_position.1 > position.1 {
                path.extend(repeat_n(
                    DirectionalKeypadButton::Down,
                    target_position.1 - position.1,
                ));
            }

            if target_position.1 < position.1 {
                path.extend(repeat_n(
                    DirectionalKeypadButton::Up,
                    position.1 - target_position.1,
                ));
            }

            if target_position.0 < position.0 {
                path.extend(repeat_n(
                    DirectionalKeypadButton::Left,
                    position.0 - target_position.0,
                ));
            }

            path.push(DirectionalKeypadButton::Activate);

            position = target_position;
        }

        path
    }
}

impl Keypad for DirectionalKeypad {
    fn position(&self, symbol: char) -> Option<Position> {
        match symbol {
            '^' => Some((1, 0)),
            'A' => Some((2, 0)),
            '<' => Some((0, 1)),
            'v' => Some((1, 1)),
            '>' => Some((2, 1)),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum DirectionalKeypadButton {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl From<DirectionalKeypadButton> for char {
    fn from(direction: DirectionalKeypadButton) -> Self {
        match direction {
            DirectionalKeypadButton::Up => '^',
            DirectionalKeypadButton::Down => 'v',
            DirectionalKeypadButton::Left => '<',
            DirectionalKeypadButton::Right => '>',
            DirectionalKeypadButton::Activate => 'A',
        }
    }
}

#[cfg(test)]
impl TryFrom<char> for DirectionalKeypadButton {
    type Error = Box<dyn Error>;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(DirectionalKeypadButton::Up),
            'v' => Ok(DirectionalKeypadButton::Down),
            '<' => Ok(DirectionalKeypadButton::Left),
            '>' => Ok(DirectionalKeypadButton::Right),
            'A' => Ok(DirectionalKeypadButton::Activate),
            _ => Err(From::from(format!("Unexpected keypad button {}", c))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_directional_button_sequence() {
        let numeric_keypad = NumericKeypad {};
        let directional_keypad = DirectionalKeypad {};

        assert_eq!(
            directions_from_str("<A^A>^^AvvvA"),
            directional_keypad.button_sequence("029A", &numeric_keypad)
        );

        assert_eq!(
            directions_from_str("v<<A>>^A<A>AvA^<AA>Av<AAA>^A"),
            directional_keypad.button_sequence("<A^A>^^AvvvA", &directional_keypad)
        );
    }

    fn directions_from_str(s: &str) -> Vec<DirectionalKeypadButton> {
        s.chars()
            .map(|c| DirectionalKeypadButton::try_from(c).unwrap())
            .collect()
    }

    #[test]
    fn test_complexity() {
        assert_eq!(68 * 29, Door::complexity("029A", 3));
        assert_eq!(60 * 980, Door::complexity("980A", 3));
        assert_eq!(68 * 179, Door::complexity("179A", 3));
        assert_eq!(64 * 456, Door::complexity("456A", 3));
        assert_eq!(64 * 379, Door::complexity("379A", 3));
    }
}
