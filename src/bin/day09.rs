use std::error::Error;
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let disk_map = DiskMap::from_str(fs::read_to_string(path)?.as_str())?;

        println!("Defrag checksum: {}", disk_map.defrag_checksum());
        
        println!(
            "Whole-file defrag checksum: {}",
            disk_map.whole_file_defrag_checksum()
        );

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

        for entry in 0..self.layout.len() {
            let is_file_entry = entry % 2 == 0;

            for _ in 0..self.layout[entry] {
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

    fn whole_file_defrag_checksum(&self) -> u64 {
        let mut disk = {
            let total_blocks = self
                .layout
                .iter()
                .map(|&block_len| block_len as usize)
                .sum();

            let mut disk = vec![None; total_blocks];
            let mut write_index = 0usize;

            for entry in 0..self.layout.len() {
                let is_file_entry = entry % 2 == 0;

                if is_file_entry {
                    let file_id = entry / 2;
                    disk[write_index..write_index + self.layout[entry] as usize]
                        .fill(Some(file_id));
                }

                write_index += self.layout[entry] as usize;
            }

            disk
        };

        let mut undefragmented: &mut [Option<usize>] = &mut disk;

        while !undefragmented.is_empty() {
            if let Some(first_empty_block) = undefragmented.iter().position(|&x| x.is_none()) {
                undefragmented = &mut undefragmented[first_empty_block..];

                if let Some((file_id, file_start, file_len)) = Self::last_file(undefragmented) {
                    if let Some(empty_span_start) = Self::next_empty_span(undefragmented, file_len)
                    {
                        undefragmented[empty_span_start..empty_span_start + file_len]
                            .fill(Some(file_id));
                        undefragmented[file_start..file_start + file_len].fill(None);
                    }

                    // Whether or not we could find space for this file, we've taken our one shot
                    // at it and don't need to inspect it again
                    undefragmented = &mut undefragmented[..file_start];
                } else {
                    // The rest of the disk is empty; we're done!
                    break;
                }
            } else {
                // No more empty space; we're done!
                break;
            }
        }

        disk.iter()
            .map(|block| block.unwrap_or(0))
            .enumerate()
            .map(|(i, x)| (i * x) as u64)
            .sum()
    }

    fn next_empty_span(blocks: &[Option<usize>], len: usize) -> Option<usize> {
        let target = vec![None; len];

        blocks.windows(len).position(|window| window == target)
    }

    fn last_file(blocks: &[Option<usize>]) -> Option<(usize, usize, usize)> {
        blocks
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, block)| block.map(|b| (i, b)))
            .map(|(end, file_id)| {
                blocks
                    .iter()
                    .enumerate()
                    .rev()
                    .skip_while(|(_, block)| block.is_none())
                    .take_while(|(_, block)| block == &&Some(file_id))
                    .map(|(i, _)| i)
                    .last()
                    .map(|start| (file_id, start, end - start + 1))
                    .unwrap()
            })
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

    #[test]
    fn test_whole_file_defrag_checksum() {
        let disk_map = DiskMap::from_str("2333133121414131402").unwrap();
        assert_eq!(2858, disk_map.whole_file_defrag_checksum());
    }

    #[test]
    fn test_last_file() {
        let blocks = [None, None, Some(0), None, Some(7), Some(7), Some(7)];
        assert_eq!(Some((7, 4, 3)), DiskMap::last_file(&blocks));
    }
}
