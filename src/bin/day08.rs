use std::error::Error;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let antenna_map = AntennaMap::from_str(fs::read_to_string(path)?.as_str())?;

        println!("Antinodes: {}", antenna_map.distinct_antinodes());

        Ok(())
    } else {
        Err("Usage: day06 INPUT_FILE_PATH".into())
    }
}

struct AntennaMap {
    frequencies: Vec<Option<char>>,
    width: usize,
}

impl AntennaMap {
    pub fn distinct_antinodes(&self) -> usize {
        self.antinodes()
            .iter()
            .filter(|&&antinode| antinode)
            .count()
    }

    fn antinodes(&self) -> Vec<bool> {
        let mut explored_frequencies = Vec::new();
        let mut antinodes = vec![false; self.frequencies.len()];

        self.frequencies
            .iter()
            .filter_map(|&frequency| frequency)
            .for_each(|frequency| {
                if !explored_frequencies.contains(&frequency) {
                    explored_frequencies.push(frequency);

                    let antenna_indices: Vec<usize> = self
                        .frequencies
                        .iter()
                        .enumerate()
                        .filter_map(|(i, f)| if f == &Some(frequency) { Some(i) } else { None })
                        .collect();

                    for i in 0..antenna_indices.len() - 1 {
                        let start = self.position(antenna_indices[i]);

                        antenna_indices[i + 1..].iter().for_each(|&other| {
                            let delta = self.position(other) - start;

                            if let Some(antinode) = self.index(&(start - delta)) {
                                antinodes[antinode] = true;
                            }

                            if let Some(antinode) = self.index(&(start + (delta * 2))) {
                                antinodes[antinode] = true;
                            }
                        })
                    }
                }
            });

        antinodes
    }

    fn position(&self, index: usize) -> Vector {
        Vector {
            x: index as i32 % self.width as i32,
            y: index as i32 / self.width as i32,
        }
    }

    fn index(&self, position: &Vector) -> Option<usize> {
        if position.x < 0
            || position.x >= self.width as i32
            || position.y < 0
            || position.y >= self.height() as i32
        {
            None
        } else {
            Some(position.x as usize + (position.y as usize * self.width))
        }
    }

    fn height(&self) -> usize {
        self.frequencies.len() / self.width
    }
}

impl FromStr for AntennaMap {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .ok_or("String must contain at least one line")?
            .len();

        let frequencies: Vec<Option<char>> = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| match c {
                '.' => None,
                _ => Some(c),
            })
            .collect();

        if frequencies.len() % width != 0 {
            Err("Antenna map must be rectangular".into())
        } else {
            Ok(AntennaMap { frequencies, width })
        }
    }
}

#[derive(Copy, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_MAP: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn test_distinct_antinodes() {
        assert_eq!(
            14,
            AntennaMap::from_str(TEST_MAP).unwrap().distinct_antinodes()
        );
    }
}
