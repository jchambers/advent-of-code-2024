use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err("Usage: day14 INPUT_FILE OUTPUT_DIRECTORY".into());
    }

    let input_path = args.get(1).unwrap();
    let output_directory = args.get(2).unwrap();

    let robots = BufReader::new(File::open(input_path)?)
        .lines()
        .map_while(Result::ok)
        .map(|line| Robot::from_str(line.as_str()))
        .collect::<Result<Vec<_>, _>>()?;

    let lobby = Lobby {
        width: 101,
        height: 103,
        robots,
    };

    println!("Safety factor: {}", lobby.safety_factor(100));
    lobby.write_interesting_images(Path::new(output_directory), 256, 20)?;

    Ok(())
}
type Vector2d = (i32, i32);

struct Lobby {
    width: usize,
    height: usize,

    robots: Vec<Robot>,
}

impl Lobby {
    pub fn safety_factor(&self, seconds: i32) -> u32 {
        let mut quadrants = [0; 4];

        for robot in &self.robots {
            if let Some(quadrant) =
                self.quadrant(&robot.position_after_seconds(seconds, self.width, self.height))
            {
                quadrants[quadrant] += 1;
            }
        }

        quadrants.iter().product()
    }

    fn quadrant(&self, position: &Vector2d) -> Option<usize> {
        let half_width = (self.width / 2) as i32;
        let half_height = (self.height / 2) as i32;

        if position.0 == half_width || position.1 == half_height {
            None
        } else if position.0 < half_width && position.1 < half_height {
            Some(0)
        } else if position.0 > half_width && position.1 < half_height {
            Some(1)
        } else if position.0 < half_width && position.1 > half_height {
            Some(2)
        } else {
            Some(3)
        }
    }

    pub fn write_interesting_images(
        &self,
        output_directory: &Path,
        max_images: usize,
        row_density_threshold: usize,
    ) -> Result<(), Box<dyn Error>> {
        let mut images_written = 0;
        let mut time = 0;

        while images_written < max_images {
            let tiles = self.tiles(time);

            if self.is_interesting_image(&tiles, row_density_threshold) {
                let mut path_buf = output_directory.to_path_buf();
                path_buf.push(format!("t{:04}.txt", time));

                self.write_image(File::create(path_buf)?, &tiles)?;

                images_written += 1;
            }

            time += 1
        }

        Ok(())
    }

    fn tiles(&self, time: i32) -> Vec<usize> {
        let mut tiles = vec![0; self.width * self.height];

        self.robots
            .iter()
            .map(|robot| robot.position_after_seconds(time, self.width, self.height))
            .map(|position| (self.width * position.1 as usize) + position.0 as usize)
            .for_each(|i| tiles[i] += 1);

        tiles
    }

    fn is_interesting_image(&self, tiles: &[usize], row_density_threshold: usize) -> bool {
        (0..self.height).any(|y| {
            let start = y * self.width;
            let end = start + self.width;

            tiles[start..end].iter().sum::<usize>() >= row_density_threshold
        })
    }

    fn write_image(&self, mut file: File, tiles: &[usize]) -> Result<(), Box<dyn Error>> {
        for y in 0..self.height {
            let start = y * self.width;
            let end = start + self.width;

            let mut line: String = tiles[start..end]
                .iter()
                .map(|n| match n {
                    0 => ' ',
                    _ => '#',
                })
                .collect();

            line.push('\n');

            file.write_all(line.as_bytes())?;
        }

        Ok(())
    }
}

struct Robot {
    initial_position: Vector2d,
    velocity: Vector2d,
}

impl Robot {
    pub fn position_after_seconds(&self, seconds: i32, width: usize, height: usize) -> Vector2d {
        let unwrapped = (
            self.initial_position.0 + (self.velocity.0 * seconds),
            self.initial_position.1 + (self.velocity.1 * seconds),
        );

        (
            Self::wrap(unwrapped.0, width as i32),
            Self::wrap(unwrapped.1, height as i32),
        )
    }

    fn wrap(dividend: i32, divisor: i32) -> i32 {
        ((dividend % divisor) + divisor) % divisor
    }
}

impl FromStr for Robot {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((p, v)) = s.split_once(' ') {
            let initial_position;

            if let Some((x, y)) = p.strip_prefix("p=").and_then(|p| p.split_once(',')) {
                initial_position = (x.parse()?, y.parse()?);
            } else {
                return Err("Could not parse initial position".into());
            }

            let velocity;

            if let Some((x, y)) = v.strip_prefix("v=").and_then(|p| p.split_once(',')) {
                velocity = (x.parse()?, y.parse()?);
            } else {
                return Err("Could not parse velocity".into());
            }

            Ok(Robot {
                initial_position,
                velocity,
            })
        } else {
            Err("Could not parse robot definition".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_ROBOTS: &str = indoc! {"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    "};

    #[test]
    fn test_wrap() {
        assert_eq!(0, Robot::wrap(0, 100));
        assert_eq!(99, Robot::wrap(-1, 100));
        assert_eq!(99, Robot::wrap(-101, 100));
        assert_eq!(1, Robot::wrap(101, 100));
        assert_eq!(1, Robot::wrap(1001, 100));
    }

    #[test]
    fn test_safety_factor() {
        let robots = TEST_ROBOTS
            .lines()
            .map(Robot::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let lobby = Lobby {
            width: 11,
            height: 7,
            robots,
        };

        assert_eq!(12, lobby.safety_factor(100));
    }
}
