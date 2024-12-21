#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u64 {
    // Instantiate a matrix where every entry is a (char, cluster_id, edges, already_in_cluster) tuple
    let mut matrix: Vec<Vec<(char, i32, u8, bool)>> = Vec::new();
    // Count for assigning cluster ids
    let mut counter = 0;
    // HashMap to keep track of how size and perimeter of each cluster
    let mut desc_list = HashMap::new();

    // Parse the input
    for row in input.lines() {
        matrix.push(
            row.chars()
                .map(|c| (c, -1, 0, false))
                .collect::<Vec<(char, i32, u8, bool)>>(),
        );
    }

    // Make a queue of elements that need to be assigned to the same cluster
    let mut queue: Vec<(usize, usize)> = Vec::new();

    for x in 0..matrix.len() {
        for y in 0..matrix[x].len() {
            if matrix[x][y].1 == -1 {
                queue.push((x, y));
            } else {
                continue;
            }

            let mut queue_has_elements = true;

            while queue_has_elements {
                let (i, j) = queue.pop().unwrap_or((0, 0));

                matrix[i][j].1 = counter;

                // Check top
                if 0 < i
                    && matrix[i][j].0 == matrix[i - 1][j].0
                    && !matrix[i - 1][j].3
                    && matrix[i - 1][j].1 == -1
                {
                    matrix[i - 1][j].3 = true;
                    queue.push((i - 1, j));
                }
                // Check right
                if j < matrix[i].len() - 1
                    && matrix[i][j].0 == matrix[i][j + 1].0
                    && !matrix[i][j + 1].3
                    && matrix[i][j + 1].1 == -1
                {
                    matrix[i][j + 1].3 = true;
                    queue.push((i, j + 1));
                }
                // Check bottom
                if i < matrix.len() - 1
                    && matrix[i][j].0 == matrix[i + 1][j].0
                    && !matrix[i + 1][j].3
                    && matrix[i + 1][j].1 == -1
                {
                    matrix[i + 1][j].3 = true;
                    queue.push((i + 1, j));
                }
                // Check left
                if 0 < j
                    && matrix[i][j].0 == matrix[i][j - 1].0
                    && !matrix[i][j - 1].3
                    && matrix[i][j - 1].1 == -1
                {
                    matrix[i][j - 1].3 = true;
                    queue.push((i, j - 1));
                }

                // Increment the size of the current_cluster
                desc_list
                    .entry(counter)
                    .and_modify(|(a, _b)| {
                        *a += 1;
                    })
                    .or_insert((1, 0));

                if queue.is_empty() {
                    queue_has_elements = false;
                }
            }
            counter += 1;
        }
    }
    // Go through the matrix again to get the perimeter of each cluster
    // We go from left to right and from top to bottom, so if we want
    // to count all edges that are in the same line as one singular edge,
    // we look if there is an edge above and to the left.
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let mut el_peri = 0;

            // Check top
            if i == 0 || (0 < i && matrix[i][j].0 != matrix[i - 1][j].0) {
                // Check if left is part of cluster
                if !(j == 0 || (0 < j && matrix[i][j].0 != matrix[i][j - 1].0)) {
                    // Check if top of left does not have an edge
                    if !(i == 0 || (0 < i && matrix[i][j].0 != matrix[i - 1][j - 1].0)) {
                        el_peri += 1;
                    }
                } else {
                    el_peri += 1;
                }
            }
            // Check right
            if j == matrix.len() - 1
                || (j < matrix[i].len() - 1 && matrix[i][j].0 != matrix[i][j + 1].0)
            {
                // Check if top is part of cluster
                if !(i == 0 || (0 < i && matrix[i][j].0 != matrix[i - 1][j].0)) {
                    // Check if right of top does not have an edge
                    if !(j == matrix.len() - 1
                        || (j < matrix[i].len() - 1 && matrix[i][j].0 != matrix[i - 1][j + 1].0))
                    {
                        el_peri += 1;
                    }
                } else {
                    el_peri += 1;
                }
            }
            // Check bottom
            if i == matrix.len() - 1
                || (i < matrix.len() - 1 && matrix[i][j].0 != matrix[i + 1][j].0)
            {
                // Check if left is part of cluster
                if !(j == 0 || (0 < j && matrix[i][j].0 != matrix[i][j - 1].0)) {
                    // Check if bottom of left does not have an edge
                    if !(i == matrix.len() - 1
                        || (i < matrix.len() - 1 && matrix[i][j].0 != matrix[i + 1][j - 1].0))
                    {
                        el_peri += 1;
                    }
                } else {
                    el_peri += 1;
                }
            }
            // Check left
            if j == 0 || (0 < j && matrix[i][j].0 != matrix[i][j - 1].0) {
                // Check if top is part of cluster
                if !(i == 0 || (0 < i && matrix[i][j].0 != matrix[i - 1][j].0)) {
                    // Check if left of top does not have an edge
                    if !(j == 0 || (0 < j && matrix[i][j].0 != matrix[i - 1][j - 1].0)) {
                        el_peri += 1;
                    }
                } else {
                    el_peri += 1;
                }
            }

            // Find the perimeter of the current_cluster
            desc_list
                .entry(matrix[i][j].1)
                .and_modify(|(_a, b)| {
                    *b += el_peri;
                })
                .or_insert((0, 0));
        }
    }

    let mut result = 0;
    for (_key, (size, perimeter)) in desc_list {
        result += size * perimeter;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "AAAA
BBCD
BBCC
EEEC";
        let result = part2(test_case);
        assert_eq!(result, 80u64);
    }

    #[test]
    fn test2() {
        let test_case = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let result = part2(test_case);
        assert_eq!(result, 236u64);
    }

    #[test]
    fn test3() {
        let test_case = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        let result = part2(test_case);
        assert_eq!(result, 368u64);
    }
}
