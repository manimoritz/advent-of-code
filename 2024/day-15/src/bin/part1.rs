#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

// Similar to day 06.
// -> Adjust day 06 code to solve the current problem.
fn part1(input: &str) -> u64 {
    // Parse the input.

    // Matrix of u8 in {0 = nothing, 1 = obstacle, 2 = box, 3 = robot}.
    let mut matrix: Vec<Vec<char>> = Vec::new();
    // Position of the robot
    let mut current_position: (usize, usize) = (0, 0);
    // If we are currently parsing the input for the matrix
    let mut matrix_input = true;
    // Array with 2D velocities for the robot
    let mut directions: Vec<(i64, i64)> = Vec::new();

    for (i, l) in input.lines().enumerate() {
        if matrix_input {
            // Check if blank line was reached after which the moveset comes.
            if l.is_empty() {
                matrix_input = false;
                continue;
            }
            matrix.push(Vec::new());

            for (j, c) in l.chars().enumerate() {
                if c == '@' {
                    current_position = (i, j);
                }

                matrix[i].push(c);
            }
        } else {
            for c in l.chars() {
                let mut direction = (0, 0);

                match c {
                    '^' => {
                        direction = (-1, 0);
                    }
                    '>' => {
                        direction = (0, 1);
                    }
                    'v' => {
                        direction = (1, 0);
                    }
                    '<' => {
                        direction = (0, -1);
                    }
                    _ => {}
                }

                directions.push(direction);
            }
        }
    }

    for direction in directions {
        // Calculate where the robot would be next.
        let next_pos = (
            (current_position.0 as i64 + direction.0) as usize,
            (current_position.1 as i64 + direction.1) as usize,
        );

        // If an obstacle hinders the robot from moving or pushing boxes, do not move.
        if can_move_in_dir(&matrix, current_position, direction) {
            // Move the robot and all the boxes that are in the way.
            move_objects(&mut matrix, current_position, direction);
            current_position = (next_pos.0, next_pos.1);
        }
    }
    sum_up_boxes(&matrix)
}

fn can_move_in_dir(
    matrix: &[Vec<char>],
    current_position: (usize, usize),
    direction: (i64, i64),
) -> bool {
    let mut next_pos = current_position;
    // Continue through the direction until an obstacle has been reached.
    while matrix[next_pos.0][next_pos.1] != '#' {
        // If there is a free space in our path, the movement of the boxes or robot is possible.
        if matrix[next_pos.0][next_pos.1] == '.' {
            return true;
        }
        let added_pair = add_pair_and_pair((next_pos.0 as i64, next_pos.1 as i64), direction);
        next_pos = (added_pair.0 as usize, added_pair.1 as usize);
    }

    false
}

fn move_objects(matrix: &mut [Vec<char>], current_position: (usize, usize), direction: (i64, i64)) {
    let added_pair = add_pair_and_pair(
        (current_position.0 as i64, current_position.1 as i64),
        direction,
    );
    let mut next_pos = (added_pair.0 as usize, added_pair.1 as usize);
    let robot_next_pos = next_pos;

    // If the next position is box, we have to move some boxes.
    if matrix[next_pos.0][next_pos.1] == 'O' {
        // Continue through the direction until a free space has been reached.
        while matrix[next_pos.0][next_pos.1] != '.' {
            let added_pair = add_pair_and_pair((next_pos.0 as i64, next_pos.1 as i64), direction);
            next_pos = (added_pair.0 as usize, added_pair.1 as usize);
        }
        // When we have reached the position of the free space, make it into a box.
        matrix[next_pos.0][next_pos.1] = 'O';
    }
    // Move the robot by one
    matrix[current_position.0][current_position.1] = '.';
    matrix[robot_next_pos.0][robot_next_pos.1] = '@';
}

fn sum_up_boxes(matrix: &[Vec<char>]) -> u64 {
    let mut result = 0;
    for (i, vec) in matrix.iter().enumerate() {
        for (j, c) in vec.iter().enumerate() {
            if *c == 'O' {
                result += (100 * i) + j;
            }
        }
    }

    result as u64
}

fn add_pair_and_pair(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

fn _print_matrix(matrix: &[Vec<char>]) {
    // Iterate through each inner vector and print
    for inner_vec in matrix {
        println!("{:?}", inner_vec);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        let result = part1(test_case);
        assert_eq!(result, 2028u64);
    }

    #[test]
    fn test2() {
        let test_case = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let result = part1(test_case);
        assert_eq!(result, 10092u64);
    }
}
