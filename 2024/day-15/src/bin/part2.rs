#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u64 {
    // Parse the input.

    // Matrix of char in {'.' = nothing, '#' = obstacle, ['[','']'] = box, '@' = robot}.
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
                match c {
                    '.' => {
                        matrix[i].push('.');
                        matrix[i].push('.');
                    }
                    '#' => {
                        matrix[i].push('#');
                        matrix[i].push('#');
                    }
                    'O' => {
                        matrix[i].push('[');
                        matrix[i].push(']');
                    }
                    '@' => {
                        matrix[i].push('@');
                        matrix[i].push('.');
                        current_position = (i, j * 2);
                    }
                    _ => {}
                }
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
        // Move the robot and all the boxes that are in the way.
        current_position = move_objects(&mut matrix, current_position, direction);
    }
    sum_up_boxes(&matrix)
}

// For every box [] that we encounter, check for both spaces if they can be moved
// (and they have to be moved by the same distance).
fn move_objects(
    matrix: &mut [Vec<char>],
    current_position: (usize, usize),
    direction: (i64, i64),
) -> (usize, usize) {
    let old_matrix: Vec<Vec<char>> = matrix.to_vec();
    // Keep track of all items that will have to move.
    let mut objects_to_move = [current_position].to_vec();
    let mut can_move = true;
    // Make a copy to simulate a queue.
    let mut track_items = objects_to_move.clone();

    // For all objects we have to move to move forward, check if they can also move forward.
    while let Some(item_pos) = track_items.pop() {
        // Look ahead by one.
        let added_pair = add_pair_and_pair((item_pos.0 as i64, item_pos.1 as i64), direction);
        let next_pos = (added_pair.0 as usize, added_pair.1 as usize);
        let next_c = matrix[next_pos.0][next_pos.1];

        // We can only move forward if all objects in the way can be moved forward.
        if next_c == '#' {
            can_move = false;
            break;
        }
        // If the next character is the left side of a box, be sure to add the right side also.
        if next_c == '[' {
            let right_side = (next_pos.0, next_pos.1 + 1);

            // Only add elements that are not already in the list.
            if !objects_to_move.contains(&next_pos) {
                objects_to_move.push(next_pos);
                track_items.push(next_pos);
            }
            if !objects_to_move.contains(&right_side) {
                objects_to_move.push(right_side);
                track_items.push(right_side);
            }
        }
        // If the next character is the right side of a box, be sure to add the left side also.
        if next_c == ']' {
            let left_side = (next_pos.0, next_pos.1 - 1);

            // Only add elements that are not already in the list.
            if !objects_to_move.contains(&next_pos) {
                objects_to_move.push(next_pos);
                track_items.push(next_pos);
            }
            if !objects_to_move.contains(&left_side) {
                objects_to_move.push(left_side);
                track_items.push(left_side);
            }
        }
    }
    if !can_move {
        return current_position;
    }

    // Move each box by one.
    for &(y, x) in objects_to_move.iter().skip(1) {
        matrix[y][x] = '.'
    }
    for &(y, x) in objects_to_move.iter().skip(1) {
        let added_pair = add_pair_and_pair((y as i64, x as i64), direction);
        let (ny, nx) = (added_pair.0 as usize, added_pair.1 as usize);
        matrix[ny][nx] = old_matrix[y][x];
    }

    // Move the robot by one.
    matrix[current_position.0][current_position.1] = '.';
    let added_pair = add_pair_and_pair(
        (current_position.0 as i64, current_position.1 as i64),
        direction,
    );
    let next_pos = (added_pair.0 as usize, added_pair.1 as usize);
    matrix[next_pos.0][next_pos.1] = '@';

    next_pos
}

fn sum_up_boxes(matrix: &[Vec<char>]) -> u64 {
    let mut result = 0;
    for (i, vec) in matrix.iter().enumerate() {
        for (j, c) in vec.iter().enumerate() {
            if *c == '[' {
                result += (100 * i) + j;
            }
        }
    }

    result as u64
}

fn add_pair_and_pair(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

// Debug functions

fn _print_matrix(matrix: &[Vec<char>]) {
    // Iterate through each inner vector and print
    for vec in matrix {
        for c in vec {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn _print_dir(direction: &(i64, i64)) {
    println!(
        "{}",
        match direction {
            (-1, 0) => '^',
            (1, 0) => 'v',
            (0, 1) => '>',
            (0, -1) => '<',
            _ => '.',
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        let result = part2(test_case);
        assert_eq!(result, 618u64);
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
        let result = part2(test_case);
        assert_eq!(result, 9021u64);
    }
}
