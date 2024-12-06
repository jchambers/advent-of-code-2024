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
            guard_map.looping_obstruction_positions()?
        );

        Ok(())
    } else {
        Err("Usage: day06 INPUT_FILE_PATH".into())
    }
}

type PositionAndHeading = ((usize, usize), Heading);

struct GuardMap {
    tiles: Vec<Tile>,
    width: usize,

    initial_position: (usize, usize),
}

impl GuardMap {
    pub fn visited_tiles(&self) -> Result<u32, Box<dyn Error>> {
        self.simulate_path(&self.tiles, self.initial_position, Up)
            .map(|mut path| {
                path.sort_by_key(|(index, _)| *index);
                path.dedup_by_key(|(index, _)| *index);
                path.len() as u32
            })
    }

    pub fn looping_obstruction_positions(&self) -> Result<u32, Box<dyn Error>> {
        let original_path = self.simulate_path(&self.tiles, self.initial_position, Up)?;
        let mut placed_obstacle_positions = vec![None; self.tiles.len()];

        for (position, heading) in &original_path[0..original_path.len() - 1] {
            let next_position = Self::next_position(*position, *heading);
            let next_position_index = self.tile_index(next_position.0, next_position.1);

            // Don't try to put an obstacle on the guard's initial position
            if next_position == self.initial_position {
                continue;
            }

            // Regardless of whether it caused a loop, have we tried putting an obstacle on this
            // tile before? Checking here has the dual benefit of avoiding duplicate work and
            // avoiding invalid situations where we try to put an obstacle "behind" the guard after
            // she's started moving.
            if placed_obstacle_positions[next_position_index].is_some() {
                continue;
            }

            if matches!(self.tiles[next_position_index], Empty) {
                let mut modified_tiles = self.tiles.clone();
                modified_tiles[next_position_index] = Obstruction;

                placed_obstacle_positions[next_position_index] = Some(
                    self.simulate_path(&modified_tiles, *position, *heading)
                        .map_or(true, |_| false),
                );
            }
        }

        Ok(placed_obstacle_positions
            .iter()
            .filter(|maybe_caused_loop| matches!(maybe_caused_loop, Some(true)))
            .count() as u32)
    }

    fn simulate_path(
        &self,
        tiles: &[Tile],
        initial_position: (usize, usize),
        initial_heading: Heading,
    ) -> Result<Vec<PositionAndHeading>, Box<dyn Error>> {
        let mut position = initial_position;
        let mut heading = initial_heading;
        let mut turns = vec![[false; 4]; tiles.len()];
        let mut path = Vec::new();

        loop {
            path.push((position, heading));

            // Are we about to exit the map?
            if (heading == Up && position.1 == 0)
                || (heading == Down && position.1 == self.height() - 1)
                || (heading == Left && position.0 == 0)
                || (heading == Right && position.0 == self.width - 1)
            {
                break;
            }

            let next_position = Self::next_position(position, heading);

            if matches!(
                tiles[self.tile_index(next_position.0, next_position.1)],
                Empty
            ) {
                position = next_position;
            } else {
                // We've bumped into an obstacle; are we in a loop?
                let index = self.tile_index(position.0, position.1);

                if turns[index][heading.index()] {
                    return Err("Loop detected".into());
                }

                turns[index][heading.index()] = true;
                heading = heading.rotate_right()
            }
        }

        Ok(path)
    }

    fn next_position(position: (usize, usize), heading: Heading) -> (usize, usize) {
        match heading {
            Up => (position.0, position.1 - 1),
            Down => (position.0, position.1 + 1),
            Left => (position.0 - 1, position.1),
            Right => (position.0 + 1, position.1),
        }
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

#[derive(Copy, Clone, Eq, PartialEq)]
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
                .unwrap()
        );
    }
}
