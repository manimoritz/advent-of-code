#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let num_of_blinks = 75;
    let output = part2(input, num_of_blinks);
    dbg!(output);
}

fn part2(input: &str, num_of_blinks: i32) -> u64 {
    // Parse the input
    let stones: Result<Vec<u64>, _> = input.split_whitespace().map(|x| x.parse::<u64>()).collect();

    match stones {
        Ok(stones_vec) => {
            let mut stones_with_count: HashMap<u64, u64> = HashMap::new();

            for &stone in stones_vec.iter() {
                if let Some(count) = stones_with_count.get(&stone) {
                    stones_with_count.insert(stone, count + 1);
                } else {
                    stones_with_count.insert(stone, 1);
                }
            }

            for _ in 0..num_of_blinks {
                stones_with_count = blink(&mut stones_with_count);
            }

            stones_with_count.values().sum()
        }
        Err(e) => {
            // Handle parsing error
            eprintln!("Failed to parse input: {}", e);
            0
        }
    }
}

fn blink(stones_with_count: &mut HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_hash = HashMap::new();

    for &key in stones_with_count.keys() {
        let count: u64 = *(stones_with_count.get(&key).unwrap_or(&0));

        // First rule
        if key == 0 {
            increment_by(&mut new_hash, 1, count);
        // Second rule
        } else {
            let stone_as_string = key.to_string();
            let string_len = stone_as_string.len();

            if string_len % 2 == 0 {
                let middle_index = stone_as_string.len() / 2;
                let (first_half, second_half) = stone_as_string.split_at(middle_index);
                if let Ok(first_value) = first_half.parse::<u64>() {
                    match second_half.parse::<u64>() {
                        Ok(second_value) => {
                            increment_by(&mut new_hash, first_value, count);
                            increment_by(&mut new_hash, second_value, count);
                        }
                        Err(_) => {
                            eprintln!("Error parsing string to u64: {}", second_half);
                        }
                    }
                } else {
                    eprintln!("Error parsing string to u64: {}", first_half);
                }
            // Default behavior
            } else {
                increment_by(&mut new_hash, key * 2024, count);
            }
        }
    }
    new_hash
}

fn increment_by(hash: &mut HashMap<u64, u64>, key: u64, value: u64) {
    *hash.entry(key).or_insert(0) += value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "0 7 198844 5687836 58 2478 25475 894";
        let result = part2(test_case, 25);
        assert_eq!(result, 216996u64);
    }
}
