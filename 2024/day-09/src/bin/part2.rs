#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i64 {
    let mut blocks: Vec<i64> = Vec::new();

    let mut block_sizes: HashMap<i64, usize> = HashMap::new();

    // Parse the input, alternating between files and free space
    for l in input.lines() {
        for (i, c) in l.chars().enumerate() {
            if let Some(parsed_char) = c.to_digit(10) {
                // Count how large the block is
                let block_len = parsed_char as usize;

                blocks.reserve(block_len);

                let block_id = if i % 2 == 0 {
                    // If it is a file block, mark the fields with the block id
                    let num = i as i64 / 2;
                    block_sizes.insert(num, block_len);
                    num
                } else {
                    // If it is a free block, mark the fields with "-1"
                    -1
                };

                // Populate the block list with the current block
                blocks.extend(std::iter::repeat(block_id).take(block_len));
            } else {
                eprintln!("Error in parsing input: char is not a number");
            }
        }
    }

    let mut block_ids: Vec<i64> = block_sizes.keys().cloned().collect();
    block_ids.sort_by(|a, b| b.cmp(a));

    // While we can still allocate blocks
    for num in block_ids {
        if num == 0 {
            break;
        }

        if let Some(&block_len) = block_sizes.get(&num) {
            if let Some(start) = find_free_space_of_size(&blocks, block_len) {
                let end = find_block(&blocks, num);

                if start > end {
                    continue;
                }

                // Bring end of rightmost file block to leftmost free space
                for i in 0..block_len {
                    blocks.swap(start + i, end + i);
                }
            } else {
                continue;
            }
        } else {
            eprintln!("Error in HashMap access");
        }
    }

    calculate_checksum(&blocks)
}

fn find_free_space_of_size(blocks: &[i64], size: usize) -> Option<usize> {
    for (i, &el) in blocks.iter().enumerate() {
        if el == -1 && blocks[i..].iter().take(size).all(|&x| x == -1) {
            return Some(i);
        }
    }
    None
}

fn find_block(blocks: &[i64], block_id: i64) -> usize {
    blocks
        .iter()
        .position(|&x| x == block_id)
        .unwrap_or_else(|| {
            eprintln!("Error: Block {} not found!", block_id);
            0
        })
}

fn calculate_checksum(blocks: &[i64]) -> i64 {
    blocks.iter().enumerate().fold(
        0,
        |acc, (i, &el)| {
            if el == -1 {
                acc
            } else {
                acc + i as i64 * el
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "2333133121414131402";
        let result = part2(test_case);
        assert_eq!(result, 2858i64);
    }

    #[test]
    fn calculate_checksum_test() {
        let test_case = [
            0, 0, 9, 9, 2, 1, 1, 1, 7, 7, 7, -1, 4, 4, -1, 3, 3, 3, -1, -1, -1, -1, 5, 5, 5, 5, -1,
            6, 6, 6, 6, -1, -1, -1, -1, -1, 8, 8, 8, 8, -1, -1,
        ];
        let result = calculate_checksum(&test_case);
        assert_eq!(result, 2858i64);
    }
}
