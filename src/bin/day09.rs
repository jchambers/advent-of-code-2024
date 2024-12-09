use std::error::Error;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let disk_map = DiskMap::from_str(fs::read_to_string(path)?.as_str())?;

        println!("Defrag checksum: {}", disk_map.defrag_checksum());

        Ok(())
    } else {
        Err("Usage: day09 INPUT_FILE_PATH".into())
    }
}

struct DiskMap {
    layout: Vec<u8>,
}

impl DiskMap {
    fn defrag_checksum(&self) -> u64 {
        let mut file_blocks = FileBlocks::new(&self.layout);
        let mut blocks_written = 0;
        let mut checksum = 0;

        for digit_index in 0..self.layout.len() {
            let is_file_entry = digit_index % 2 == 0;

            for _ in 0..self.layout[digit_index] {
                if let Some(file_id) = if is_file_entry {
                    file_blocks.next()
                } else {
                    file_blocks.next_back()
                } {
                    checksum += file_id as u64 * blocks_written;
                    blocks_written += 1;
                } else {
                    // We've exhausted the iterator; no need to keep looping
                    return checksum;
                }
            }
        }

        checksum
    }

    fn total_file_blocks(layout: &[u8]) -> usize {
        layout
            .iter()
            .step_by(2)
            .map(|&block_len| block_len as usize)
            .sum()
    }
}

impl FromStr for DiskMap {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let layout = s
            .chars()
            .filter_map(|c| c.to_digit(10).map(|digit| digit as u8))
            .collect();

        Ok(DiskMap { layout })
    }
}

struct FileBlocks<'a> {
    layout: &'a [u8],
    total_file_blocks: usize,
    total_blocks_written: usize,

    front_file_id: usize,
    front_blocks_written: u8,

    back_file_id: usize,
    back_blocks_written: u8,
}

impl<'a> FileBlocks<'a> {
    fn new(layout: &'a [u8]) -> Self {
        let total_file_blocks = DiskMap::total_file_blocks(layout);

        Self {
            layout,
            total_file_blocks,
            total_blocks_written: 0,

            front_file_id: 0,
            front_blocks_written: 0,

            back_file_id: layout.len() / 2,
            back_blocks_written: 0,
        }
    }
}

impl Iterator for FileBlocks<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.total_blocks_written == self.total_file_blocks {
            None
        } else {
            while self.front_blocks_written == self.layout[self.front_file_id * 2] {
                // We've written an entire file and need to move on to the next one
                self.front_file_id += 1;
                self.front_blocks_written = 0;
            }

            self.front_blocks_written += 1;
            self.total_blocks_written += 1;

            Some(self.front_file_id)
        }
    }
}

impl DoubleEndedIterator for FileBlocks<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.total_blocks_written == self.total_file_blocks {
            None
        } else {
            while self.back_blocks_written == self.layout[self.back_file_id * 2] {
                // We've written an entire file and need to move on to the next one
                self.back_file_id -= 1;
                self.back_blocks_written = 0;
            }

            self.back_blocks_written += 1;
            self.total_blocks_written += 1;

            Some(self.back_file_id)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_file_blocks() {
        let file_blocks = FileBlocks::new(&[1, 2, 3, 4, 5]);
        assert_eq!(
            vec![0, 1, 1, 1, 2, 2, 2, 2, 2],
            file_blocks.collect::<Vec<usize>>()
        );
    }

    #[test]
    fn test_file_blocks_rev() {
        let file_blocks = FileBlocks::new(&[1, 2, 3, 4, 5]).rev();
        assert_eq!(
            vec![2, 2, 2, 2, 2, 1, 1, 1, 0],
            file_blocks.collect::<Vec<usize>>()
        );
    }

    #[test]
    fn test_defrag_checksum() {
        let disk_map = DiskMap::from_str("2333133121414131402").unwrap();
        assert_eq!(1928, disk_map.defrag_checksum());
    }
}
