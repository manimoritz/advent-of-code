fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut occurrences = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for l in input.lines() {
        matrix.push(l.chars().collect());
    }
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            if matrix[i][j] == 'A' && find_x_in_matrix(&matrix, i, j) {
                occurrences += 1;
            }
        }
    }
    occurrences
}

fn find_x_in_matrix(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    check_top_left_to_bottom_right(matrix, i, j) && check_top_right_to_bottom_left(matrix, i, j)
}

fn check_top_left_to_bottom_right(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    (matrix[i - 1][j - 1] == 'M' && matrix[i + 1][j + 1] == 'S')
        || (matrix[i - 1][j - 1] == 'S' && matrix[i + 1][j + 1] == 'M')
}

fn check_top_right_to_bottom_left(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    (matrix[i - 1][j + 1] == 'M' && matrix[i + 1][j - 1] == 'S')
        || (matrix[i - 1][j + 1] == 'S' && matrix[i + 1][j - 1] == 'M')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        let result = part2(test_case);
        assert_eq!(result, 9i32);
    }
}
