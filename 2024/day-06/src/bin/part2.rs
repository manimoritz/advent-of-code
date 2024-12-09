fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut original_matrix: Vec<Vec<(bool, bool, bool)>> = Vec::new();
    let mut original_current_position: (usize, usize) = (0, 0);
    let mut original_direction: (i32, i32) = (0, 0);

    for (i, l) in input.lines().enumerate() {
        original_matrix.push(Vec::new());

        for (j, c) in l.chars().enumerate() {
            let mut obstacle = false;
            let mut visited = false;

            match c {
                '^' => {
                    original_direction = (-1, 0);
                    visited = true;
                    original_current_position = (i, j);
                }
                '>' => {
                    original_direction = (0, 1);
                    visited = true;
                    original_current_position = (i, j);
                }
                'v' => {
                    original_direction = (1, 0);
                    visited = true;
                    original_current_position = (i, j);
                }
                '<' => {
                    original_direction = (0, -1);
                    visited = true;
                    original_current_position = (i, j);
                }
                '#' => {
                    obstacle = true;
                }
                _ => {}
            }

            original_matrix[i].push((obstacle, visited, false));
        }
    }

    let mut loop_counter = 0;

    for i in 0..original_matrix.len() {
        for j in 0..original_matrix[i].len() {
            let mut direction = original_direction;
            let mut current_position = original_current_position;
            let mut matrix = original_matrix.clone();
            if matrix[i][j].0 {
                continue;
            }
            matrix[i][j].0 = true;

            // Necessary for ensuring program does not count obstacles right after each other
            let mut corner_counter = 0;
            let mut next_step = (
                current_position.0 as i32 + direction.0,
                current_position.1 as i32 + direction.1,
            );
            // While the next square is not out of frame
            while next_step.0 >= 0
                && (next_step.0 as usize) < matrix.len()
                && next_step.1 >= 0
                && (next_step.1 as usize) < matrix[current_position.0].len()
            {
                // Set the current square as visited
                matrix[current_position.0][current_position.1].1 = true;

                // Check if the next square is an obstacle
                if matrix[next_step.0 as usize][next_step.1 as usize].0 {
                    // Check if the corner has already been visited
                    if matrix[current_position.0][current_position.1].2 {
                        if corner_counter == 3 {
                            loop_counter += 1;
                            break;
                        }
                        corner_counter += 1;
                    }
                    // Mark the square as a corner and rotate by 90 degrees
                    matrix[current_position.0][current_position.1].2 = true;
                    direction = (direction.1, -direction.0);
                } else {
                    // Update the position of the guard
                    current_position = (next_step.0 as usize, next_step.1 as usize);
                }

                // Update the next step of the guart
                next_step = (
                    current_position.0 as i32 + direction.0,
                    current_position.1 as i32 + direction.1,
                );
            }
        }
    }

    loop_counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let result = part2(test_case);
        assert_eq!(result, 6i32);
    }
}
