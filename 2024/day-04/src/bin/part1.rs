fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut occurrences = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for l in input.lines() {
        matrix.push(l.chars().collect());
    }
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            occurrences += find_string_in_matrix(&"XMAS".chars().collect(), "any", &matrix, i, j);
        }
    }
    occurrences
}

fn find_string_in_matrix(
    old_string: &Vec<char>,
    direction: &str,
    matrix: &Vec<Vec<char>>,
    vertical_index: usize,
    horizontal_index: usize,
) -> i32 {
    let mut string = old_string.clone();
    let searching_char = string.remove(0);
    let current_char = matrix[vertical_index][horizontal_index];

    if current_char != searching_char {
        return 0;
    }

    let mut occurrences = 0;

    if string.is_empty() {
        return 1;
    }

    match direction {
        "right" => {
            occurrences += find_string_in_matrix(
                &string,
                "right",
                matrix,
                vertical_index,
                horizontal_index + 1,
            );
        }
        "right-down" => {
            occurrences += find_string_in_matrix(
                &string,
                "right-down",
                matrix,
                vertical_index + 1,
                horizontal_index + 1,
            );
        }
        "right-up" => {
            occurrences += find_string_in_matrix(
                &string,
                "right-up",
                matrix,
                vertical_index - 1,
                horizontal_index + 1,
            );
        }
        "left" => {
            occurrences += find_string_in_matrix(
                &string,
                "left",
                matrix,
                vertical_index,
                horizontal_index - 1,
            );
        }
        "left-down" => {
            occurrences += find_string_in_matrix(
                &string,
                "left-down",
                matrix,
                vertical_index + 1,
                horizontal_index - 1,
            );
        }
        "left-up" => {
            occurrences += find_string_in_matrix(
                &string,
                "left-up",
                matrix,
                vertical_index - 1,
                horizontal_index - 1,
            );
        }
        "up" => {
            occurrences +=
                find_string_in_matrix(&string, "up", matrix, vertical_index - 1, horizontal_index);
        }
        "down" => {
            occurrences += find_string_in_matrix(
                &string,
                "down",
                matrix,
                vertical_index + 1,
                horizontal_index,
            );
        }
        _ => {
            // Execute the function for all direction in which the whole string would fit
            if horizontal_index >= string.len() {
                occurrences += find_string_in_matrix(
                    &string,
                    "left",
                    matrix,
                    vertical_index,
                    horizontal_index - 1,
                );

                if vertical_index >= string.len() {
                    occurrences += find_string_in_matrix(
                        &string,
                        "left-up",
                        matrix,
                        vertical_index - 1,
                        horizontal_index - 1,
                    );
                }
                if vertical_index + string.len() < matrix.len() {
                    occurrences += find_string_in_matrix(
                        &string,
                        "left-down",
                        matrix,
                        vertical_index + 1,
                        horizontal_index - 1,
                    );
                }
            }
            if horizontal_index + string.len() < matrix[vertical_index].len() {
                occurrences += find_string_in_matrix(
                    &string,
                    "right",
                    matrix,
                    vertical_index,
                    horizontal_index + 1,
                );

                if vertical_index >= string.len() {
                    occurrences += find_string_in_matrix(
                        &string,
                        "right-up",
                        matrix,
                        vertical_index - 1,
                        horizontal_index + 1,
                    );
                }
                if vertical_index + string.len() < matrix.len() {
                    occurrences += find_string_in_matrix(
                        &string,
                        "right-down",
                        matrix,
                        vertical_index + 1,
                        horizontal_index + 1,
                    );
                }
            }
            if vertical_index >= string.len() {
                occurrences += find_string_in_matrix(
                    &string,
                    "up",
                    matrix,
                    vertical_index - 1,
                    horizontal_index,
                );
            }
            if vertical_index + string.len() < matrix.len() {
                occurrences += find_string_in_matrix(
                    &string,
                    "down",
                    matrix,
                    vertical_index + 1,
                    horizontal_index,
                );
            }
        }
    }
    occurrences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX";
        let result = part1(test_case);
        assert_eq!(result, 3i32);
    }

    #[test]
    fn test2() {
        let test_case = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = part1(test_case);
        assert_eq!(result, 18i32);
    }
}
