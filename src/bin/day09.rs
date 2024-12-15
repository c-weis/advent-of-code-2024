use std::cmp::min;

use itertools::Itertools;
use rusty_advent_2024::utils::lines_from_file;

#[derive(Clone, Copy, Debug)]
enum DataBlock {
    File { id: usize, size: usize },
    Free { size: usize },
}

#[derive(Debug)]
struct DiskReaderStatus {
    blocks: Vec<DataBlock>,
    checksum: u128,
    disk_position: u128,
    left_block_idx: usize,
    right_block_idx: usize,
    left_block_tmp: Option<DataBlock>,
    right_block_tmp: Option<DataBlock>,
}

impl DiskReaderStatus {
    fn from(blocks: Vec<DataBlock>) -> Self {
        DiskReaderStatus {
            checksum: 0,
            disk_position: 0,
            left_block_idx: 0,
            right_block_idx: &blocks.len() - 1,
            blocks,

            // use _tmp blocks to keep track of
            //  - partially filled Free Blocks on the left
            //  - partially moved File Blocks on the right
            left_block_tmp: None,
            right_block_tmp: None,
        }
    }

    fn left_block(&self) -> &DataBlock {
        match &self.left_block_tmp {
            Some(block) => block,
            _ => &self.blocks[self.left_block_idx],
        }
    }

    fn right_block(&self) -> &DataBlock {
        match &self.right_block_tmp {
            Some(block) => block,
            _ => &self.blocks[self.right_block_idx],
        }
    }

    fn advance_left_index(&mut self) {
        self.left_block_idx += 1;
        self.left_block_tmp = None;
    }

    fn advance_right_index(&mut self) {
        self.right_block_idx -= 1;
        self.right_block_tmp = None;
    }

    fn add_to_checksum(&mut self, id: &usize, size: &usize) {
        self.checksum += partial_checksum(id, &self.disk_position, size) as u128;
        self.disk_position += *size as u128;
    }

    fn simulate_moving_files(
        &mut self,
        &free_size: &usize,
        &right_id: &usize,
        &right_size: &usize,
    ) {
        let moved_files = min(free_size, right_size);
        self.add_to_checksum(&right_id, &moved_files);

        let new_free_size = free_size - moved_files;
        if new_free_size == 0 {
            self.advance_left_index();
        } else {
            self.left_block_tmp = Some(DataBlock::Free {
                size: new_free_size,
            });
        }

        let new_right_size = right_size - moved_files;
        if new_right_size == 0 {
            self.advance_right_index();
        } else {
            self.right_block_tmp = Some(DataBlock::File {
                id: right_id,
                size: new_right_size,
            });
        }
    }

    fn step(&mut self) {
        match (self.left_block().clone(), self.right_block().clone()) {
            // 1. file block on the left: add to checksum, advance left
            (DataBlock::File { id, size }, right_block) => {
                // special case: left block == right block
                // block might already have partially moved
                if self.left_block_idx == self.right_block_idx {
                    if let DataBlock::File {
                        id,
                        size: actual_size,
                    } = right_block
                    {
                        self.add_to_checksum(&id, &actual_size);
                    }
                } else {
                    self.add_to_checksum(&id, &size);
                }
                self.advance_left_index();
            }
            // 2. free block on the right: advance right
            (_, DataBlock::Free { size: _ }) => self.advance_right_index(),
            // 3. free block on the left: simulate moving files from right by adding to checksum
            (
                DataBlock::Free { size: free_size },
                DataBlock::File {
                    id: right_id,
                    size: right_size,
                },
            ) => self.simulate_moving_files(&free_size, &right_id, &right_size),
        }
    }

    fn total_checksum(&mut self) -> u128 {
        while &self.right_block_idx >= &self.left_block_idx {
            self.step();
        }
        self.checksum
    }
}

fn partial_checksum(&id: &usize, &start_position: &u128, &size: &usize) -> u128 {
    id as u128 * (start_position..start_position + size as u128).sum::<u128>()
}

fn blocks_from_string(string: String) -> Vec<DataBlock> {
    string
        .split("")
        .filter_map(|character| -> Option<usize> { character.parse().ok() })
        .enumerate()
        .map(|(idx, size)| -> DataBlock {
            if idx % 2 == 0 {
                DataBlock::File { id: idx / 2, size }
            } else {
                DataBlock::Free { size }
            }
        })
        .collect_vec()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input09.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input09.txt"));
}

fn part1(path: &str) -> u128 {
    let string = lines_from_file(path)
        .map(|line| line.unwrap())
        .find_or_first(|_| true)
        .expect("No input found.");

    let blocks = blocks_from_string(string);

    let mut analyser = DiskReaderStatus::from(blocks);

    analyser.total_checksum()
}

fn part2(_path: &str) -> u128 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_checksum() {
        assert!(partial_checksum(&7, &10, &5) == 7 * (10 + 11 + 12 + 13 + 14))
    }

    #[test]
    fn test_tiny_disks() {
        // "2": 00 -> 00
        let mut analyser1 = DiskReaderStatus::from(blocks_from_string(String::from("2")));
        assert!(analyser1.total_checksum() == 0);

        // "232": 00...11 -> 0011...
        let mut analyser2 = DiskReaderStatus::from(blocks_from_string(String::from("232")));
        assert!(analyser2.total_checksum() == 5);

        // "12345": 0..111....22222 -> 022111222.....
        let mut analyser3 = DiskReaderStatus::from(blocks_from_string(String::from("12345")));
        assert!(
            analyser3.total_checksum()
                == (partial_checksum(&0, &0, &1)
                    + partial_checksum(&2, &1, &2)
                    + partial_checksum(&1, &3, &3)
                    + partial_checksum(&2, &6, &3)) as u128
        );

        // "3132": 000.111.. -> 000111...
        let mut analyser4 = DiskReaderStatus::from(blocks_from_string(String::from("3132")));
        assert!(analyser4.total_checksum() == 3 + 4 + 5);
    }

    #[test]
    fn test_part1() {
        assert!(part1("input/input09.txt.test1") == 1928);
    }

    #[test]
    fn test_part2() {
        assert!(part2("input/input09.txt.test1") == 0);
    }
}
