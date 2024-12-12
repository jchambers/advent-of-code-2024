use std::error::Error;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let garden_map = GardenMap::from_str(fs::read_to_string(path)?.as_str())?;

        println!("Fencing cost: {}", garden_map.fencing_cost());

        Ok(())
    } else {
        Err("Usage: day12 INPUT_FILE_PATH".into())
    }
}

struct GardenMap {
    vegetables: Vec<char>,
    width: usize,
}

impl GardenMap {
    pub fn fencing_cost(&self) -> u32 {
        let mut mapped_regions = vec![false; self.vegetables.len()];
        let mut fencing_cost = 0;

        while let Some(start) = mapped_regions.iter().position(|mapped| !mapped) {
            let vegetable = self.vegetables[start];

            let mut stack = vec![start];
            let mut explored = mapped_regions.clone();

            let mut area = 0;
            let mut perimeter = 0;

            while let Some(i) = stack.pop() {
                if explored[i] {
                    continue;
                } else {
                    explored[i] = true;
                }

                if self.vegetables[i] == vegetable {
                    mapped_regions[i] = true;
                    area += 1;

                    let neighbors = self.neighbors(i);

                    perimeter += 4 - neighbors
                        .iter()
                        .filter(|&&n| self.vegetables[n] == vegetable)
                        .count() as u32;

                    stack.extend(neighbors.iter());
                }
            }

            fencing_cost += area * perimeter;
        }

        fencing_cost
    }

    fn height(&self) -> usize {
        self.vegetables.len() / self.width
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

impl FromStr for GardenMap {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .ok_or("String must contain at least one line")?
            .len();

        let vegetables: Vec<char> = s.chars().filter(|c| !c.is_whitespace()).collect();

        if vegetables.len() % width == 0 {
            Ok(GardenMap { vegetables, width })
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
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    #[test]
    fn test_fencing_cost() {
        let garden_map = GardenMap::from_str(TEST_MAP).unwrap();
        assert_eq!(1930, garden_map.fencing_cost());
    }
}
