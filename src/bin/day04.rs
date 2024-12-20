use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let word_search = {
            let mut string = String::new();
            File::open(path)?.read_to_string(&mut string)?;

            WordSearch::from_str(&string)?
        };

        println!("XMAS occurrences: {}", word_search.word_count("XMAS"));
        println!("Cross occurrences: {}", word_search.cross_count());

        Ok(())
    } else {
        Err("Usage: day04 INPUT_FILE_PATH".into())
    }
}

struct WordSearch {
    grid: Vec<char>,
    width: usize,
}

impl WordSearch {
    pub fn word_count(&self, word: &str) -> u32 {
        let mut word_count = 0;
        let word_reversed = word.chars().rev().collect::<String>();

        // Check for horizontal matches
        for col in 0..self.width - (word.len() - 1) {
            for row in 0..self.height() {
                let mut candidate = String::new();

                for i in 0..word.len() {
                    candidate.push(self.char_at(row, col + i));
                }

                if candidate == word || candidate == word_reversed {
                    word_count += 1;
                }
            }
        }

        // Check for vertical matches
        for col in 0..self.width {
            for row in 0..self.height() - (word.len() - 1) {
                let mut candidate = String::new();

                for i in 0..word.len() {
                    candidate.push(self.char_at(row + i, col));
                }

                if candidate == word || candidate == word_reversed {
                    word_count += 1;
                }
            }
        }

        // Check for \ matches
        for col in 0..self.width - (word.len() - 1) {
            for row in 0..self.height() - (word.len() - 1) {
                let mut candidate = String::new();

                for i in 0..word.len() {
                    candidate.push(self.char_at(row + i, col + i));
                }

                if candidate == word || candidate == word_reversed {
                    word_count += 1;
                }
            }
        }

        // Check for / matches
        for col in 0..self.width - (word.len() - 1) {
            for row in word.len() - 1..self.height() {
                let mut candidate = String::new();

                for i in 0..word.len() {
                    candidate.push(self.char_at(row - i, col + i));
                }

                if candidate == word || candidate == word_reversed {
                    word_count += 1;
                }
            }
        }

        word_count
    }

    fn cross_count(&self) -> u32 {
        let mut cross_count = 0;

        for col in 1..self.width - 1 {
            for row in 1..self.height() - 1 {
                if self.char_at(row, col) != 'A' {
                    continue;
                }

                let top_left = self.char_at(row - 1, col - 1);
                let top_right = self.char_at(row - 1, col + 1);
                let bottom_left = self.char_at(row + 1, col - 1);
                let bottom_right = self.char_at(row + 1, col + 1);

                if ((top_left == 'M' && bottom_right == 'S')
                    || (top_left == 'S' && bottom_right == 'M'))
                    && ((top_right == 'M' && bottom_left == 'S')
                        || (top_right == 'S' && bottom_left == 'M'))
                {
                    cross_count += 1;
                }
            }
        }

        cross_count
    }

    fn height(&self) -> usize {
        self.grid.len() / self.width
    }

    fn char_at(&self, row: usize, col: usize) -> char {
        self.grid[self.index(row, col)]
    }

    fn index(&self, row: usize, col: usize) -> usize {
        (row * self.width) + col
    }
}

impl FromStr for WordSearch {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .ok_or("String must contain at least one line")?
            .len();
        let grid: Vec<char> = s.chars().filter(|c| !c.is_whitespace()).collect();

        if grid.len() % width != 0 {
            return Err("Grid must be rectangular".into());
        }

        Ok(WordSearch { grid, width })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_GRID: &str = indoc! {"\
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    fn test_word_count() {
        assert_eq!(18, WordSearch::from_str(TEST_GRID).unwrap().word_count("XMAS"));
    }

    #[test]
    fn test_cross_count() {
        assert_eq!(9, WordSearch::from_str(TEST_GRID).unwrap().cross_count());
    }
}
