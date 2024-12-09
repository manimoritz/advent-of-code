fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut matrix: Vec<Vec<(bool, bool)>> = Vec::new();
    let mut current_position: (usize, usize) = (0, 0);
    let mut direction: (i32, i32) = (0, 0);

    for (i, l) in input.lines().enumerate() {
        matrix.push(Vec::new());

        for (j, c) in l.chars().enumerate() {
            let mut obstacle = false;
            let mut visited = false;

            match c {
                '^' => {
                    direction = (-1, 0);
                    visited = true;
                    current_position = (i, j);
                }
                '>' => {
                    direction = (0, 1);
                    visited = true;
                    current_position = (i, j);
                }
                'v' => {
                    direction = (1, 0);
                    visited = true;
                    current_position = (i, j);
                }
                '<' => {
                    direction = (0, -1);
                    visited = true;
                    current_position = (i, j);
                }
                '#' => {
                    obstacle = true;
                }
                _ => {}
            }

            matrix[i].push((obstacle, visited));
        }
    }

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

    // Sum up all the visited squares plus 1 for the last square before leaving the frame
    let result = matrix
        .iter()
        .map(|row| row.iter().filter(|entry| entry.1).collect::<Vec<_>>().len())
        .sum::<usize>()
        + 1;
    result as i32
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
        let result = part1(test_case);
        assert_eq!(result, 41i32);
    }
}
