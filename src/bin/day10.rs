use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let trail_map = TrailMap::from_str(fs::read_to_string(path)?.as_str())?;

        println!("Map score: {}", trail_map.score());
        println!("Map rating: {}", trail_map.rating());

        Ok(())
    } else {
        Err("Usage: day10 INPUT_FILE_PATH".into())
    }
}

struct TrailMap {
    elevations: Vec<u8>,
    width: usize,
}

impl TrailMap {
    pub fn score(&self) -> u32 {
        let trailheads: Vec<usize> = self
            .elevations
            .iter()
            .enumerate()
            .filter_map(|(i, elevation)| if *elevation == 0 { Some(i) } else { None })
            .collect();

        let mut score = 0;

        for trailhead in trailheads {
            let mut stack = vec![trailhead];
            let mut summits = HashSet::new();

            while let Some(i) = stack.pop() {
                let elevation = self.elevations[i];

                if elevation == 9 {
                    // We've found a summit; stop exploring
                    summits.insert(i);
                    continue;
                }

                stack.extend(self.neighbors(i).iter().filter_map(|&i| {
                    if self.elevations[i] == elevation + 1 {
                        Some(i)
                    } else {
                        None
                    }
                }));
            }

            score += summits.len() as u32;
        }

        score
    }

    fn rating(&self) -> u32 {
        let mut stack: Vec<usize> = self
            .elevations
            .iter()
            .enumerate()
            .filter_map(|(i, elevation)| if *elevation == 0 { Some(i) } else { None })
            .collect();

        let mut rating = 0;

        while let Some(i) = stack.pop() {
            let elevation = self.elevations[i];

            if elevation == 9 {
                // We've found a summit; stop exploring
                rating += 1;
                continue;
            }

            stack.extend(self.neighbors(i).iter().filter_map(|&i| {
                if self.elevations[i] == elevation + 1 {
                    Some(i)
                } else {
                    None
                }
            }));
        }

        rating
    }

    fn height(&self) -> usize {
        self.elevations.len() / self.width
    }

    fn neighbors(&self, index: usize) -> Vec<usize> {
        let x = index % self.width;
        let y = index / self.width;

        let mut neighbors = Vec::with_capacity(4);

        if x > 0 {
            neighbors.push(index - 1);
        }

        if x < self.width - 1 {
            neighbors.push(index + 1);
        }

        if y > 0 {
            neighbors.push(index - self.width);
        }

        if y < self.height() - 1 {
            neighbors.push(index + self.width);
        }

        neighbors
    }
}

impl FromStr for TrailMap {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .ok_or("String must contain at least one line")?
            .len();

        let elevations: Vec<u8> = s
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|height| height as u8)
            .collect();

        if elevations.len() % width == 0 {
            Ok(TrailMap { elevations, width })
        } else {
            Err("Map must be rectangular".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_MAP: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[test]
    fn test_score() {
        assert_eq!(36, TrailMap::from_str(TEST_MAP).unwrap().score());
    }

    #[test]
    fn test_rating() {
        assert_eq!(81, TrailMap::from_str(TEST_MAP).unwrap().rating());
    }
}
