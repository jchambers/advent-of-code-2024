use std::error::Error;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let racetrack = RaceTrack::from_str(fs::read_to_string(path)?.as_str())?;

        println!(
            "2-picosecond cheats saving at least 100 picoseconds: {}",
            racetrack
                .cheats(2)
                .iter()
                .filter(|&&savings| savings >= 100)
                .count()
        );

        println!(
            "20-picosecond cheats saving at least 100 picoseconds: {}",
            racetrack
                .cheats(20)
                .iter()
                .filter(|&&savings| savings >= 100)
                .count()
        );

        Ok(())
    } else {
        Err("Usage: day20 INPUT_FILE_PATH".into())
    }
}

struct RaceTrack {
    tiles: Vec<Tile>,
    width: usize,

    start: usize,
    end: usize,
}

impl RaceTrack {
    pub fn cheats(&self, radius: usize) -> Vec<u32> {
        let times_along_path = self.times_along_path();
        let mut cheats = Vec::new();

        times_along_path
            .iter()
            .enumerate()
            .filter_map(|(i, elapsed_time)| elapsed_time.map(|t| (i, t)))
            .for_each(|(i, elapsed_time)| {
                self.neighbors(i, radius)
                    .iter()
                    .filter_map(|&neighbor| times_along_path[neighbor].map(|t| (neighbor, t)))
                    .for_each(|(neighbor, neighbor_time)| {
                        let distance = self.distance(i, neighbor);

                        if neighbor_time > elapsed_time + distance {
                            // This is actually a shortcut
                            cheats.push(neighbor_time - elapsed_time - distance);
                        }
                    });
            });

        cheats
    }

    fn neighbors(&self, index: usize, radius: usize) -> Vec<usize> {
        let height = self.tiles.len() / self.width;

        let (x, y) = (index % self.width, index / self.width);

        let x_min = x.saturating_sub(radius);
        let x_max = (x + radius).min(self.width - 1);
        let y_min = y.saturating_sub(radius);
        let y_max = (y + radius).min(height - 1);

        let mut neighbors = Vec::new();

        for x_neighbor in x_min..=x_max {
            for y_neighbor in y_min..=y_max {
                if x.abs_diff(x_neighbor) + y.abs_diff(y_neighbor) <= radius
                    && (x != x_neighbor || y != y_neighbor)
                {
                    neighbors.push((self.width * y_neighbor) + x_neighbor);
                }
            }
        }

        neighbors
    }

    fn distance(&self, a: usize, b: usize) -> u32 {
        let (x_a, y_a) = (a % self.width, a / self.width);
        let (x_b, y_b) = (b % self.width, b / self.width);

        (x_a.abs_diff(x_b) + y_a.abs_diff(y_b)) as u32
    }

    fn times_along_path(&self) -> Vec<Option<u32>> {
        let mut distances = vec![None; self.tiles.len()];
        distances[self.start] = Some(0);

        let mut index = self.start;
        let mut distance = 0;

        // The problem statement asserts that there's exactly one path through the maze, and we
        // additionally assert that the whole racetrack has a solid border of walls.
        loop {
            if index == self.end {
                break;
            }

            distance += 1;

            let next = *[index + 1, index - 1, index + self.width, index - self.width]
                .iter()
                .find(|&&neighbor| {
                    self.tiles[neighbor] == Tile::Track && distances[neighbor].is_none()
                })
                .unwrap();

            distances[next] = Some(distance);
            index = next;
        }

        distances
    }
}

impl FromStr for RaceTrack {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .ok_or("String must contain at least one line")?
            .len();

        let mut start = None;
        let mut end = None;

        let tiles: Vec<Tile> = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .enumerate()
            .map(|(i, c)| match c {
                '.' => Ok(Tile::Track),
                '#' => Ok(Tile::Wall),
                'S' => {
                    start = Some(i);
                    Ok(Tile::Track)
                }
                'E' => {
                    end = Some(i);
                    Ok(Tile::Track)
                }
                _ => Err("Unrecognized tile"),
            })
            .collect::<Result<_, _>>()?;

        if tiles.len() % width != 0 {
            return Err("Race track must be rectangular".into());
        }

        Ok(RaceTrack {
            tiles,
            width,

            start: start.ok_or("Start index not found")?,
            end: end.ok_or("End index not found")?,
        })
    }
}

#[derive(Eq, PartialEq)]
enum Tile {
    Track,
    Wall,
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_RACETRACK: &str = indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    "};

    #[test]
    fn test() {
        let racetrack = RaceTrack::from_str(TEST_RACETRACK).unwrap();

        {
            let cheats = racetrack.cheats(2);

            for (n, savings) in [
                (14, 2),
                (14, 4),
                (2, 6),
                (4, 8),
                (2, 10),
                (3, 12),
                (1, 20),
                (1, 36),
                (1, 38),
                (1, 40),
                (1, 64),
            ] {
                assert_eq!(n, cheats.iter().filter(|&&s| s == savings).count());
            }
        }

        {
            let cheats = racetrack.cheats(20);

            for (n, savings) in [
                (32, 50),
                (31, 52),
                (29, 54),
                (39, 56),
                (25, 58),
                (23, 60),
                (20, 62),
                (19, 64),
                (12, 66),
                (14, 68),
            ] {
                assert_eq!(n, cheats.iter().filter(|&&s| s == savings).count());
            }
        }
    }
}
