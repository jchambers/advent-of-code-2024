use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let memory_region = MemoryRegion::new(
            71,
            BufReader::new(File::open(path)?)
                .lines()
                .map_while(Result::ok),
        )?;

        println!(
            "Shortest path at time 1024: {}",
            memory_region.shortest_path(1024)?
        );

        let blocking_coordinate = memory_region.blocking_coordinate();

        println!(
            "Coordinate that blocks path to exit: {},{}",
            blocking_coordinate.0, blocking_coordinate.1
        );

        Ok(())
    } else {
        Err("Usage: day18 INPUT_FILE_PATH".into())
    }
}

struct MemoryRegion {
    size: usize,
    falling_bytes: Vec<usize>,
}

impl MemoryRegion {
    pub fn new(
        size: usize,
        falling_bytes: impl IntoIterator<Item = String>,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            size,
            falling_bytes: falling_bytes
                .into_iter()
                .map(|line| {
                    if let Some((x, y)) = line.split_once(',') {
                        let x: usize = x.parse()?;
                        let y: usize = y.parse()?;

                        Ok((y * size) + x)
                    } else {
                        Err("Could not parse line".into())
                    }
                })
                .collect::<Result<Vec<usize>, Box<dyn Error>>>()?,
        })
    }

    pub fn shortest_path(&self, time: usize) -> Result<u32, Box<dyn Error>> {
        let mut queue = VecDeque::from([(0, 0)]);
        let mut explored = vec![false; self.size * self.size];
        let safe_coordinates = {
            let mut safe_coordinates = vec![true; self.size * self.size];

            self.falling_bytes[0..time]
                .iter()
                .for_each(|&i| safe_coordinates[i] = false);

            safe_coordinates
        };

        let exit_index = (self.size * self.size) - 1;

        while let Some((index, elapsed_time)) = queue.pop_front() {
            if index == exit_index {
                return Ok(elapsed_time as u32);
            }

            if explored[index] {
                continue;
            }

            explored[index] = true;

            queue.extend(
                self.safe_neighbors(index, &safe_coordinates)
                    .iter()
                    .filter(|&&neighbor| !explored[neighbor])
                    .map(|&neighbor| (neighbor, elapsed_time + 1)),
            );
        }

        Err("No path to exit".into())
    }

    pub fn blocking_coordinate(&self) -> (usize, usize) {
        let i = self.falling_bytes[self.last_time_to_exit()];

        (i % self.size, i / self.size)
    }

    fn last_time_to_exit(&self) -> usize {
        let mut left = 0;
        let mut right = self.falling_bytes.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;

            if self.shortest_path(mid).is_ok() {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        right
    }

    fn safe_neighbors(&self, index: usize, safe_coordinates: &[bool]) -> Vec<usize> {
        let x = index % self.size;
        let y = index / self.size;

        let mut safe_neighbors = Vec::new();

        if x > 0 && safe_coordinates[index - 1] {
            safe_neighbors.push(index - 1);
        }

        if x < self.size - 1 && safe_coordinates[index + 1] {
            safe_neighbors.push(index + 1);
        }

        if y > 0 && safe_coordinates[index - self.size] {
            safe_neighbors.push(index - self.size);
        }

        if y < self.size - 1 && safe_coordinates[index + self.size] {
            safe_neighbors.push(index + self.size);
        }

        safe_neighbors
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_BYTES: &str = indoc! {"
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    "};

    #[test]
    fn test_shortest_path() {
        let memory_region = MemoryRegion::new(7, TEST_BYTES.lines().map(String::from)).unwrap();
        assert_eq!(22, memory_region.shortest_path(12).unwrap());
    }

    #[test]
    fn test_blocking_coordinate() {
        let memory_region = MemoryRegion::new(7, TEST_BYTES.lines().map(String::from)).unwrap();
        assert_eq!((6, 1), memory_region.blocking_coordinate());
    }
}
