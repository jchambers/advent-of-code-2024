use std::error::Error;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let garden_map = GardenMap::from_str(fs::read_to_string(path)?.as_str())?;

        println!("Fencing cost: {}", garden_map.fencing_cost());

        println!(
            "Fencing cost with discount: {}",
            garden_map.fencing_cost_with_discount()
        );

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
        self.regions()
            .iter()
            .map(|region| {
                let vegetable = self.vegetables[*region.first().unwrap()];

                let perimeter: u32 = region
                    .iter()
                    .map(|&i| {
                        4 - self
                            .neighbors(i)
                            .iter()
                            .filter(|&&n| self.vegetables[n] == vegetable)
                            .count() as u32
                    })
                    .sum();

                region.len() as u32 * perimeter
            })
            .sum()
    }

    pub fn fencing_cost_with_discount(&self) -> u32 {
        self.regions()
            .iter()
            .map(|region| self.sides(region) * region.len() as u32)
            .sum()
    }

    fn sides(&self, region: &[usize]) -> u32 {
        let mut sides = 0;
        let (x_range, y_range) = self.bounding_box(region);

        // Find horizontal sides
        for y in y_range.clone() {
            let mut top_fences = Vec::with_capacity(self.width);
            let mut bottom_fences = Vec::with_capacity(self.width);

            for x in x_range.clone() {
                if region.contains(&self.index((x, y))) {
                    top_fences.push(y == 0 || !region.contains(&self.index((x, y - 1))));
                    bottom_fences
                        .push(y == self.height() - 1 || !region.contains(&self.index((x, y + 1))));
                } else {
                    top_fences.push(false);
                    bottom_fences.push(false);
                }
            }

            sides += Self::segments(&top_fences) + Self::segments(&bottom_fences);
        }

        // Find vertical sides
        for x in x_range.clone() {
            let mut left_fences = Vec::with_capacity(self.height());
            let mut right_fences = Vec::with_capacity(self.height());

            for y in y_range.clone() {
                if region.contains(&self.index((x, y))) {
                    left_fences.push(x == 0 || !region.contains(&self.index((x - 1, y))));
                    right_fences
                        .push(x == self.width - 1 || !region.contains(&self.index((x + 1, y))));
                } else {
                    left_fences.push(false);
                    right_fences.push(false);
                }
            }

            sides += Self::segments(&left_fences) + Self::segments(&right_fences);
        }

        sides
    }

    fn segments(fences: &[bool]) -> u32 {
        let mut segments = 0;

        let mut previous = false;

        for &fence in fences {
            if previous && !fence {
                segments += 1;
            }

            previous = fence;
        }

        if previous {
            segments += 1;
        }

        segments
    }

    fn bounding_box(&self, region: &[usize]) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
        let mut x_min = usize::MAX;
        let mut x_max = usize::MIN;
        let mut y_min = usize::MAX;
        let mut y_max = usize::MIN;

        region.iter().map(|&i| self.position(i)).for_each(|(x, y)| {
            x_min = x_min.min(x);
            x_max = x_max.max(x);
            y_min = y_min.min(y);
            y_max = y_max.max(y);
        });

        (x_min..=x_max, y_min..=y_max)
    }

    fn regions(&self) -> Vec<Vec<usize>> {
        let mut mapped_regions = vec![false; self.vegetables.len()];
        let mut regions = Vec::new();

        while let Some(start) = mapped_regions.iter().position(|mapped| !mapped) {
            let vegetable = self.vegetables[start];

            let mut stack = vec![start];
            let mut explored = mapped_regions.clone();

            let mut region = Vec::new();

            while let Some(i) = stack.pop() {
                if explored[i] {
                    continue;
                } else {
                    explored[i] = true;
                }

                if self.vegetables[i] == vegetable {
                    mapped_regions[i] = true;
                    region.push(i);

                    stack.extend(self.neighbors(i).iter());
                }
            }

            regions.push(region);
        }

        regions
    }

    fn height(&self) -> usize {
        self.vegetables.len() / self.width
    }

    fn position(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn index(&self, position: (usize, usize)) -> usize {
        (self.width * position.1) + position.0
    }

    fn neighbors(&self, index: usize) -> Vec<usize> {
        let (x, y) = self.position(index);

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

    #[test]
    fn test_fencing_cost_with_discount() {
        let garden_map = GardenMap::from_str(TEST_MAP).unwrap();
        assert_eq!(1206, garden_map.fencing_cost_with_discount());
    }
}
