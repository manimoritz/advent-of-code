#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i64 {
    let mut blocks: Vec<i64> = Vec::new();

    // Parse the input, alternating between files and free space
    for l in input.lines() {
        for (i, c) in l.chars().enumerate() {
            if let Some(parsed_char) = c.to_digit(10) {
                // Count how large the block is
                let block_len = parsed_char as usize;

                blocks.reserve(block_len);

                let block_id = if i % 2 == 0 {
                    // If it is a file block, mark the fields with the block id
                    i as i64 / 2
                } else {
                    // If it is a free block, mark the fields with "-1"
                    -1
                };

                // Populate the block list with the current block
                blocks.extend(std::iter::repeat_with(|| block_id).take(block_len));
            } else {
                eprintln!("Error in parsing input: char is not a number");
            }
        }
    }

    let mut free_space_left = count_free_space(&blocks);

    // While we can still allocate blocks
    while free_space_left != 0 {
        let start = find_free_space(&blocks);
        let end = find_end_of_last_block(&blocks);

        // Bring end of rightmost file block to leftmost free space
        blocks.swap(start, end);

        // See if there is still free space left of the last file block
        free_space_left = count_free_space(&blocks);
    }

    calculate_checksum(&blocks)
}

fn count_free_space(blocks: &[i64]) -> usize {
    blocks[0..=find_end_of_last_block(blocks)]
        .iter()
        .filter(|x| **x == -1)
        .count()
}

fn find_free_space(blocks: &[i64]) -> usize {
    blocks.iter().position(|&x| x == -1).unwrap_or(0)
}

fn find_end_of_last_block(blocks: &[i64]) -> usize {
    blocks.iter().rposition(|&x| x != -1).unwrap_or(0)
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
        let result = part1(test_case);
        assert_eq!(result, 1928i64);
    }

    #[test]
    fn calculate_checksum_test() {
        let test_case = [
            0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        ];
        let result = calculate_checksum(&test_case);
        assert_eq!(result, 1928i64);
    }
}
