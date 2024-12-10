#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i64 {
    let mut height_map: Vec<Vec<u32>> = Vec::new();

    // Parse the input
    for l in input.lines() {
        height_map.push(
            l.chars()
                .map(|x| {
                    x.to_digit(10).unwrap_or_else(|| {
                        eprintln!("Not a valid digit: '{}'", x);
                        0u32
                    })
                })
                .collect(),
        );
    }

    let mut result = 0;

    // Find every trailhead in the matrix
    for (i, row) in height_map.iter().enumerate() {
        for (j, &el) in row.iter().enumerate() {
            if el != 0 {
                continue;
            }
            result += find_num_of_trails(&height_map, i, j);
        }
    }
    result
}

fn find_num_of_trails(height_map: &[Vec<u32>], i: usize, j: usize) -> i64 {
    let mut result = 0;
    let current_num = height_map[i][j];

    if current_num == 9 {
        return 1;
    }

    // Look up
    if 0 < i && height_map[i - 1][j] == current_num + 1 {
        result += find_num_of_trails(height_map, i - 1, j);
    }
    // Look right
    if j < height_map[i].len() - 1 && height_map[i][j + 1] == current_num + 1 {
        result += find_num_of_trails(height_map, i, j + 1);
    }
    // Look down
    if i < height_map.len() - 1 && height_map[i + 1][j] == current_num + 1 {
        result += find_num_of_trails(height_map, i + 1, j);
    }
    // Look left
    if 0 < j && height_map[i][j - 1] == current_num + 1 {
        result += find_num_of_trails(height_map, i, j - 1);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let result = part2(test_case);
        assert_eq!(result, 81i64);
    }
}
