use crate::Heading::{Down, Left, Right, Up};
use crate::Tile::{Empty, Obstruction};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let guard_map = {
            let mut string = String::new();
            File::open(path)?.read_to_string(&mut string)?;

            GuardMap::from_str(&string)?
        };

        println!("Visited tiles: {}", guard_map.visited_tiles()?);

        println!(
            "Positions of new obstacles that would cause a loop: {}",
            guard_map.looping_obstruction_positions()
        );

        Ok(())
    } else {
        Err("Usage: day06 INPUT_FILE_PATH".into())
    }
}

struct GuardMap {
    tiles: Vec<Tile>,
    width: usize,

    initial_position: (usize, usize),
}

impl GuardMap {
    pub fn visited_tiles(&self) -> Result<u32, Box<dyn Error>> {
        self.simulate_path(&self.tiles)
    }

    pub fn looping_obstruction_positions(&self) -> u32 {
        let initial_position_index =
            self.tile_index(self.initial_position.0, self.initial_position.1);

        let candidate_obstruction_indices: Vec<usize> = self
            .tiles
            .iter()
            .enumerate()
            .filter_map(|(index, tile)| {
                if index == initial_position_index {
                    None
                } else if matches!(tile, Empty) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        candidate_obstruction_indices
            .iter()
            .map(|&obstruction_index| {
                let mut modified_tiles = self.tiles.clone();
                modified_tiles[obstruction_index] = Obstruction;

                self.simulate_path(&modified_tiles)
            })
            .filter(|result| result.is_err())
            .count() as u32
    }

    fn simulate_path(&self, tiles: &[Tile]) -> Result<u32, Box<dyn Error>> {
        let mut position = self.initial_position;
        let mut heading = Up;
        let mut visited_tiles = vec![[false; 4]; tiles.len()];

        loop {
            // Are we in a loop?
            if visited_tiles[self.tile_index(position.0, position.1)][heading.index()] {
                return Err("Loop detected".into());
            }

            visited_tiles[self.tile_index(position.0, position.1)][heading.index()] = true;

            // Are we about to exit the map?
            if (heading == Up && position.1 == 0)
                || (heading == Down && position.1 == self.height() - 1)
                || (heading == Left && position.0 == 0)
                || (heading == Right && position.0 == self.width - 1)
            {
                break;
            }

            let next_position = match heading {
                Up => (position.0, position.1 - 1),
                Down => (position.0, position.1 + 1),
                Left => (position.0 - 1, position.1),
                Right => (position.0 + 1, position.1),
            };

            match tiles[self.tile_index(next_position.0, next_position.1)] {
                Empty => position = next_position,
                Obstruction => heading = heading.rotate_right(),
            };
        }

        Ok(visited_tiles
            .iter()
            .filter(|visited_from_directions| {
                visited_from_directions.iter().any(|&visited| visited)
            })
            .count() as u32)
    }

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn tile_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }
}

impl FromStr for GuardMap {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .ok_or("String must contain at least one line")?
            .len();

        let mut initial_position = None;

        let tiles: Vec<Tile> = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .enumerate()
            .map(|(i, c)| match c {
                '.' => Ok(Empty),
                '#' => Ok(Obstruction),
                '^' => {
                    let initial_x = i % width;
                    let initial_y = i / width;

                    initial_position = Some((initial_x, initial_y));

                    Ok(Empty)
                }
                _ => Err(format!("Unexpected tile: {}", c)),
            })
            .collect::<Result<_, _>>()?;

        if tiles.len() % width != 0 {
            return Err("Grid must be rectangular".into());
        }

        if let Some(initial_position) = initial_position {
            Ok(GuardMap {
                tiles,
                width,
                initial_position,
            })
        } else {
            Err("No initial position found".into())
        }
    }
}

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Obstruction,
}

#[derive(Eq, PartialEq)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

impl Heading {
    pub fn rotate_right(&self) -> Heading {
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Up => 0,
            Down => 1,
            Left => 2,
            Right => 3,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_MAP: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn test_visited_tiles() {
        assert_eq!(
            41,
            GuardMap::from_str(TEST_MAP)
                .unwrap()
                .visited_tiles()
                .unwrap()
        );
    }

    #[test]
    fn test_looping_obstruction_positions() {
        assert_eq!(
            6,
            GuardMap::from_str(TEST_MAP)
                .unwrap()
                .looping_obstruction_positions()
        );
    }
}
