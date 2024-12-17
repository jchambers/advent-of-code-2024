use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error::Error;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let maze = ReindeerMaze::from_str(fs::read_to_string(path)?.as_str())?;

        println!("Lowest possible score: {}", maze.lowest_score().unwrap());

        Ok(())
    } else {
        Err("Usage: day16 INPUT_FILE_PATH".into())
    }
}

struct ReindeerMaze {
    tiles: Vec<Tile>,
    width: usize,

    start: usize,
    end: usize,
}

impl ReindeerMaze {
    pub fn lowest_score(&self) -> Result<u32, ()> {
        self.lowest_scores()[self.end]
            .iter()
            .filter_map(|&score| score)
            .min()
            .ok_or(())
    }

    fn lowest_scores(&self) -> Vec<[Option<u32>; 4]> {
        let mut priority_queue = BinaryHeap::new();
        let mut lowest_scores = vec![[None; 4]; self.tiles.len()];

        priority_queue.push(ReindeerState {
            index: self.start,
            heading: Direction::Right,
            score: 0,
        });

        while let Some(ReindeerState {
            index,
            heading,
            score,
        }) = priority_queue.pop()
        {
            if score > lowest_scores[index][heading as usize].unwrap_or(u32::MAX) {
                // We've already found a lower-cost way to get to this state
                continue;
            } else {
                // This is the new best way to get to this state
                lowest_scores[index][heading as usize] = Some(score);
            }

            let forward_index = self.next_index(index, heading);

            if self.tiles[forward_index] == Tile::Empty {
                priority_queue.push(ReindeerState {
                    index: forward_index,
                    heading,
                    score: score + 1,
                });
            }

            let turns = {
                let candidate_turns = match heading {
                    Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
                    Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
                };

                let mut turns = Vec::with_capacity(2);

                for candidate_turn in candidate_turns {
                    if self.tiles[self.next_index(index, candidate_turn)] == Tile::Empty {
                        turns.push(candidate_turn);
                    }
                }

                turns
            };

            for turn in turns {
                priority_queue.push(ReindeerState {
                    index,
                    heading: turn,
                    score: score + 1000,
                });
            }
        }

        lowest_scores
    }

    fn next_index(&self, index: usize, direction: Direction) -> usize {
        match direction {
            Direction::Up => index - self.width,
            Direction::Down => index + self.width,
            Direction::Left => index - 1,
            Direction::Right => index + 1,
        }
    }
}

impl FromStr for ReindeerMaze {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .ok_or("String must contain at least one line")?
            .len();

        let mut start = None;
        let mut end = None;

        let tiles = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .enumerate()
            .map(|(i, c)| match c {
                '.' => Ok(Tile::Empty),
                '#' => Ok(Tile::Wall),
                'S' => {
                    start = Some(i);
                    Ok(Tile::Empty)
                }
                'E' => {
                    end = Some(i);
                    Ok(Tile::Empty)
                }
                _ => Err("Unexpected tile"),
            })
            .collect::<Result<_, _>>()?;

        Ok(ReindeerMaze {
            tiles,
            width,

            start: start.ok_or("Start tile not found")?,
            end: end.ok_or("End tile not found")?,
        })
    }
}

#[derive(Eq, PartialEq)]
struct ReindeerState {
    index: usize,
    heading: Direction,
    score: u32,
}

impl Ord for ReindeerState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for a min-heap
        other
            .score
            .cmp(&self.score)
            .then_with(|| other.index.cmp(&self.index))
    }
}

impl PartialOrd for ReindeerState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for usize {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 3,
            Direction::Right => 4,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_MAZE_SMALL: &str = indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    "};

    const TEST_MAZE_LARGE: &str = indoc! {"
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
    "};

    #[test]
    fn test_lowest_score() {
        {
            let maze = ReindeerMaze::from_str(TEST_MAZE_SMALL).unwrap();
            assert_eq!(7036, maze.lowest_score().unwrap());
        }

        {
            let maze = ReindeerMaze::from_str(TEST_MAZE_LARGE).unwrap();
            assert_eq!(11048, maze.lowest_score().unwrap());
        }
    }
}
