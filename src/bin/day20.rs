use std::error::Error;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let racetrack = RaceTrack::from_str(fs::read_to_string(path)?.as_str())?;
        let cheats = racetrack.cheats();

        println!(
            "Cheats saving at least 100 picoseconds: {}",
            cheats.iter().filter(|&&savings| savings >= 100).count()
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
    pub fn cheats(&self) -> Vec<u32> {
        let main_path = self.distances_on_main_path();
        let mut cheats = Vec::new();

        // Look for horizontal cheats
        let height = self.tiles.len() / self.width;

        for y in 1..height - 1 {
            for x in 1..self.width - 2 {
                let start = (y * self.width) + x;

                if let [Some(a), None, Some(b)] = main_path[start..start + 3] {
                    cheats.push(a.abs_diff(b) - 2);
                }
            }
        }

        // …and look for vertical cheats

        for x in 1..self.width - 1 {
            for y in 1..height - 2 {
                let start = (y * self.width) + x;

                if let [Some(a), None, Some(b)] = [
                    main_path[start],
                    main_path[start + self.width],
                    main_path[start + (self.width * 2)],
                ] {
                    cheats.push(a.abs_diff(b) - 2);
                }
            }
        }

        cheats
    }

    fn distances_on_main_path(&self) -> Vec<Option<u32>> {
        let mut distances = vec![None; self.tiles.len()];
        distances[self.start] = Some(0);

        let mut position = self.start;
        let mut distance = 0;

        // The problem statement asserts that there's exactly one path through the maze, and we
        // additionally assert that the whole racetrack has a solid border of walls.
        loop {
            if position == self.end {
                break;
            }

            distance += 1;

            let next = *[
                position + 1,
                position - 1,
                position + self.width,
                position - self.width,
            ]
            .iter()
            .find(|&&neighbor| self.tiles[neighbor] == Tile::Track && distances[neighbor].is_none())
            .unwrap();

            distances[next] = Some(distance);
            position = next;
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

            start: start.ok_or("Start position not found")?,
            end: end.ok_or("End position not found")?,
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
        let cheats = racetrack.cheats();

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
}
