#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

fn main() {
    let input = include_str!("./input.txt");
    let num_of_blinks = 25;
    let output = part1(input, num_of_blinks);
    dbg!(output);
}

fn part1(input: &str, num_of_blinks: i32) -> usize {
    // Parse the input
    let stones: Result<Vec<u64>, _> = input.split_whitespace().map(|x| x.parse::<u64>()).collect();

    match stones {
        Ok(mut stones_vec) => {
            // Successfully parsed, use `stones_vec`
            for _i in 0..num_of_blinks {
                blink(&mut stones_vec);
            }

            stones_vec.len()
        }
        Err(e) => {
            // Handle parsing error
            eprintln!("Failed to parse input: {}", e);
            0
        }
    }
}

fn blink(stones: &mut Vec<u64>) {
    let mut i = 0;
    let mut len = stones.len();

    while i < len {
        // First rule
        if stones[i] == 0 {
            stones[i] = 1;
        // Second rule
        } else {
            let stone_as_string = stones[i].to_string();
            let string_len = stone_as_string.len();
            if string_len % 2 == 0 {
                let middle_index = stone_as_string.len() / 2;
                let (first_half, second_half) = stone_as_string.split_at(middle_index);
                match first_half.parse::<u64>() {
                    Ok(parsed_value) => stones[i] = parsed_value,
                    Err(_) => {
                        eprintln!("Error parsing string to u64: {}", first_half);
                    }
                }
                match second_half.parse::<u64>() {
                    Ok(parsed_value) => {
                        i += 1;
                        stones.insert(i, parsed_value);
                        len += 1;
                    }
                    Err(_) => {
                        eprintln!("Error parsing string to u64: {}", second_half);
                    }
                }
            // Default behavior
            } else {
                stones[i] *= 2024;
            }
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "125 17";
        let result = part1(test_case, 25);
        assert_eq!(result, 55312);
    }
}
