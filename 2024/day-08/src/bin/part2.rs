#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut matrix: Vec<Vec<(char, bool)>> = Vec::new();

    // Parse the input into a matrix
    for l in input.lines() {
        matrix.push(l.chars().map(|c| (c, false)).collect());
    }

    // Iterate through the matrix
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let antenna = matrix[i][j].0;

            if antenna == '.' {
                continue;
            }

            let pairs = search_for_pairs(&matrix, i, j);

            for (x, y) in pairs {
                let signed_i = i as i32;
                let signed_j = j as i32;
                let signed_x = x as i32;
                let signed_y = y as i32;
                let rel_pos = (signed_x - signed_i, signed_y - signed_j);

                // Find the greates common divisor of rel_pos to find the step size and direction
                let step = (rel_pos.0 / gcd(rel_pos), rel_pos.1 / gcd(rel_pos));

                // Search in both directions of the line
                let mut edge_reached = false;
                let mut factor = 0;

                while !edge_reached {
                    let antinode_pos = (signed_i - step.0 * factor, signed_j - step.1 * factor);
                    factor += 1;

                    // Check if the antinode_pos is outside the matrix
                    if antinode_pos.0 < 0
                        || antinode_pos.0 >= matrix.len() as i32
                        || antinode_pos.1 < 0
                        || antinode_pos.1 >= matrix[i].len() as i32
                    {
                        edge_reached = true;
                        continue;
                    }
                    matrix[antinode_pos.0 as usize][antinode_pos.1 as usize].1 = true;
                }

                edge_reached = false;
                factor = 0;

                while !edge_reached {
                    let antinode_pos = (signed_i + step.0 * factor, signed_j + step.1 * factor);
                    factor += 1;

                    // Check if the antinode_pos is outside the matrix
                    if antinode_pos.0 < 0
                        || antinode_pos.0 >= matrix.len() as i32
                        || antinode_pos.1 < 0
                        || antinode_pos.1 >= matrix[i].len() as i32
                    {
                        edge_reached = true;
                        continue;
                    }
                    matrix[antinode_pos.0 as usize][antinode_pos.1 as usize].1 = true;
                }
            }
        }
    }

    let mut result = 0;

    // Count all the fields with antinodes
    for row in matrix.iter() {
        for element in row {
            if element.1 {
                result += 1;
            }
        }
    }
    result
}

fn gcd((a, b): (i32, i32)) -> i32 {
    let mut x = a.abs();
    let mut y = b.abs();

    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}

fn search_for_pairs(matrix: &[Vec<(char, bool)>], o_i: usize, o_j: usize) -> Vec<(usize, usize)> {
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    let original_char = matrix[o_i][o_j].0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            if i == o_i && j == o_j {
                continue;
            }

            let antenna = element.0;

            if antenna == original_char {
                pairs.push((i, j));
            }
        }
    }
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let result = part2(test_case);
        assert_eq!(result, 34i32);
    }
}
