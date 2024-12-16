use std::error::Error;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let warehouse = LanternfishWarehouse::from_str(fs::read_to_string(path)?.as_str())?;

        println!("Box GPS sum: {}", warehouse.gps_sum());

        Ok(())
    } else {
        Err("Usage: day15 INPUT_FILE_PATH".into())
    }
}

type Position = (usize, usize);

struct LanternfishWarehouse {
    tiles: Vec<Tile>,
    width: usize,

    moves: Vec<Direction>,

    robot_position: Position,
}

impl LanternfishWarehouse {
    pub fn gps_sum(&self) -> u32 {
        let mut tiles = self.tiles.clone();
        let mut robot_position = self.robot_position;

        self.moves.iter().for_each(|&direction| {
            if let Some(movable_boxes) = self.movable_boxes(&robot_position, direction, &tiles) {
                if movable_boxes > 0 {
                    tiles[self.index(&Self::advance_position(&robot_position, direction, 1))] =
                        Tile::Empty;

                    tiles[self.index(&Self::advance_position(
                        &robot_position,
                        direction,
                        movable_boxes + 1,
                    ))] = Tile::Box;
                }

                robot_position = Self::advance_position(&robot_position, direction, 1);
            }
        });

        tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile == &&Tile::Box)
            .map(|(i, _)| Self::gps(&self.position(i)))
            .sum()
    }

    fn gps(position: &Position) -> u32 {
        (100 * position.1 as u32) + position.0 as u32
    }

    fn movable_boxes(
        &self,
        robot_position: &Position,
        direction: Direction,
        tiles: &[Tile],
    ) -> Option<usize> {
        let mut movable_boxes = 0;
        let mut position = *robot_position;

        loop {
            position = Self::advance_position(&position, direction, 1);

            match tiles[self.index(&position)] {
                Tile::Empty => break Some(movable_boxes),
                Tile::Wall => break None,
                Tile::Box => movable_boxes += 1,
            }
        }
    }

    fn advance_position(position: &Position, direction: Direction, steps: usize) -> Position {
        match direction {
            Direction::Up => (position.0, position.1 - steps),
            Direction::Down => (position.0, position.1 + steps),
            Direction::Left => (position.0 - steps, position.1),
            Direction::Right => (position.0 + steps, position.1),
        }
    }

    fn index(&self, position: &Position) -> usize {
        (position.1 * self.width) + position.0
    }

    fn position(&self, index: usize) -> Position {
        (index % self.width, index / self.width)
    }
}

impl FromStr for LanternfishWarehouse {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((tiles, moves)) = s.split_once("\n\n") {
            let width = tiles
                .lines()
                .next()
                .ok_or("Map must contain at least one line")?
                .len();

            let mut robot_position = None;

            let tiles: Vec<Tile> = tiles
                .chars()
                .filter(|c| !c.is_whitespace())
                .enumerate()
                .map(|(i, c)| match c {
                    '#' => Ok(Tile::Wall),
                    '.' => Ok(Tile::Empty),
                    'O' => Ok(Tile::Box),
                    '@' => {
                        robot_position = Some((i % width, i / width));
                        Ok(Tile::Empty)
                    }
                    _ => Err(format!("Unrecognized tile: {}", c)),
                })
                .collect::<Result<_, _>>()?;

            if robot_position.is_none() {
                return Err("Robot position not found".into());
            }

            let moves = moves
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| match c {
                    '^' => Ok(Direction::Up),
                    'v' => Ok(Direction::Down),
                    '<' => Ok(Direction::Left),
                    '>' => Ok(Direction::Right),
                    _ => Err(format!("Unrecognized direction: {}", c)),
                })
                .collect::<Result<_, _>>()?;

            Ok(LanternfishWarehouse {
                tiles,
                width,
                moves,
                robot_position: robot_position.ok_or("Robot index not found")?,
            })
        } else {
            Err("Could not parse map and move list".into())
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Box,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_WAREHOUSE_SMALL: &str = indoc! {"
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<
    "};

    const TEST_WAREHOUSE_LARGE: &str = indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "};

    #[test]
    fn test() {
        let warehouse = LanternfishWarehouse::from_str(TEST_WAREHOUSE_SMALL).unwrap();
        assert_eq!(2028, warehouse.gps_sum());

        let warehouse = LanternfishWarehouse::from_str(TEST_WAREHOUSE_LARGE).unwrap();
        assert_eq!(10092, warehouse.gps_sum());
    }
}
