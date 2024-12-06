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

        println!("Visited tiles: {}", guard_map.visited_tiles());

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
    pub fn visited_tiles(&self) -> u32 {
        let mut position = self.initial_position;
        let mut heading = Up;
        let mut visited_tiles = vec![false; self.tiles.len()];

        loop {
            visited_tiles[self.tile_index(position.0, position.1)] = true;

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

            match self.tiles[self.tile_index(next_position.0, next_position.1)] {
                Empty => position = next_position,
                Obstruction => heading = heading.rotate_right(),
            };
        }

        visited_tiles.iter().filter(|&&visited| visited).count() as u32
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
        assert_eq!(41, GuardMap::from_str(TEST_MAP).unwrap().visited_tiles());
    }
}
