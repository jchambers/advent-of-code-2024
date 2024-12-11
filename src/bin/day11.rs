use std::error::Error;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let pebble_line = PebbleLine::from_str(fs::read_to_string(path)?.as_str())?;

        let pebble_count = {
            let mut evolved_line = pebble_line.clone();

            for _ in 0..25 {
                evolved_line = evolved_line.evolve();
            }

            evolved_line.pebbles.len()
        };

        println!("Pebbles after 25 blinks: {}", pebble_count);

        Ok(())
    } else {
        Err("Usage: day11 INPUT_FILE_PATH".into())
    }
}

#[derive(Clone)]
struct PebbleLine {
    pebbles: Vec<u64>,
}

impl PebbleLine {
    fn evolve(self) -> Self {
        let mut next = Vec::new();

        for pebble in self.pebbles {
            if pebble == 0 {
                next.push(1);
            } else if pebble.ilog10() % 2 != 0 {
                let mask = 10u64.pow((pebble.ilog10() + 1) / 2);

                next.push(pebble / mask);
                next.push(pebble % ((pebble / mask) * mask));
            } else {
                next.push(pebble * 2024);
            }
        }

        PebbleLine { pebbles: next }
    }
}

impl FromStr for PebbleLine {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pebbles = s
            .split_whitespace()
            .map(|p| p.parse::<u64>())
            .collect::<Result<_, _>>()?;

        Ok(PebbleLine { pebbles })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_PEBBLE_LINE: &str = "125 17";

    #[test]
    fn test_evolve() {
        let pebble_line = PebbleLine::from_str(TEST_PEBBLE_LINE).unwrap();

        let pebble_line = pebble_line.evolve();
        assert_eq!(vec![253000, 1, 7], pebble_line.pebbles);

        let pebble_line = pebble_line.evolve();
        assert_eq!(vec![253, 0, 2024, 14168], pebble_line.pebbles);

        let pebble_line = pebble_line.evolve();
        assert_eq!(vec![512072, 1, 20, 24, 28676032], pebble_line.pebbles);

        let pebble_line = pebble_line.evolve();
        assert_eq!(
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032],
            pebble_line.pebbles
        );

        let pebble_line = pebble_line.evolve();
        assert_eq!(
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32],
            pebble_line.pebbles
        );

        let pebble_line = pebble_line.evolve();
        assert_eq!(
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ],
            pebble_line.pebbles
        );
    }
}
