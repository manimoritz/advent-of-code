fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

// Task:
// Given a matrix of dots and chars, find all antinodes.
// An antinode is a field which is on the same line as some chars a1 and a2 where a1==a2 and
// a2 reflected by a1 yields the position of the antinode.
// Return the number of squares in the matrix where such an antinode exists.
//
// Naive approach:
// Iterate through a matrix(value: char, is_antinode: bool) and for each char a1 found, iterate
// through the same matrix and search the same char a2. For every a2 found, get the position of a2
// relative to a1 (rel_pos(a2, a1) = pos(a2) - pos(a1)) and check if pos(antinode) = pos(a1) -
// rel_pos(a2, a1) is in the matrix. If yes, then set the field's is_antidode value to true.
fn part1(input: &str) -> i32 {
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

                // The antinode is outside the matrix
                if signed_i < rel_pos.0
                    || signed_i - rel_pos.0 >= matrix.len() as i32
                    || signed_j < rel_pos.1
                    || signed_j - rel_pos.1 >= matrix[i].len() as i32
                {
                    continue;
                }
                let antinode_pos = (signed_i - rel_pos.0, signed_j - rel_pos.1);
                matrix[antinode_pos.0 as usize][antinode_pos.1 as usize].1 = true;
            }
        }
    }

    let mut result = 0;

    // Count all the fields with antinodes
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j].1 {
                result += 1;
            }
        }
    }
    return result;
}

fn search_for_pairs(
    matrix: &Vec<Vec<(char, bool)>>,
    o_i: usize,
    o_j: usize,
) -> Vec<(usize, usize)> {
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    let original_char = matrix[o_i][o_j].0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if i == o_i && j == o_j {
                continue;
            }

            let antenna = matrix[i][j].0;

            if antenna == original_char {
                pairs.push((i, j));
            }
        }
    }
    return pairs;
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
        let result = part1(test_case);
        assert_eq!(result, 14i32);
    }
}
