fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn grid_sym(grid: &Vec<Vec<char>>, y: i32, x: i32) -> bool {
    if 0 <= y && y < grid.len() as i32 {
        let y1 = y as usize;
        if 0 <= x && x < grid[y1].len() as i32 {
            let x1 = x as usize;
            return is_symbol(grid[y1][x1]);
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn search_in_grid(components: &Vec<(u32, Vec<(usize, usize)>)>, i: usize, j: usize) -> (u32, bool) {
    for (num, tups) in components {
        for (x, y) in tups {
            if i == *x && j == *y {
                return (*num, true);
            }
        }
    }
    (0, false)
}

fn part2(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut result = 0;
    let mut components: Vec<(u32, Vec<(usize, usize)>)> = Vec::new();
    for (i, l) in input.lines().enumerate() {
        grid.push(Vec::new());
        for c in l.chars() {
            grid[i].push(c);
        }
    }

    for (i, row) in grid.iter().enumerate() {
        let mut number = 0;
        let mut number_indices: Vec<(usize, usize)> = Vec::new();
        let mut is_counted = false;
        for (j, c) in row.iter().enumerate() {
            if is_number(*c) {
                is_counted |= grid_sym(&grid, i as i32 - 1, j as i32 - 1);
                is_counted |= grid_sym(&grid, i as i32, j as i32 - 1);
                is_counted |= grid_sym(&grid, i as i32 + 1, j as i32 - 1);

                is_counted |= grid_sym(&grid, i as i32 - 1, j as i32);
                //is_counted |= grid_sym(&grid,i as i32,j as i32);
                is_counted |= grid_sym(&grid, i as i32 + 1, j as i32);

                is_counted |= grid_sym(&grid, i as i32 - 1, j as i32 + 1);
                is_counted |= grid_sym(&grid, i as i32, j as i32 + 1);
                is_counted |= grid_sym(&grid, i as i32 + 1, j as i32 + 1);

                number_indices.push((i, j));
                number = number * 10 + (*c as u32) - ('0' as u32);

                if j == row.len() - 1 {
                    if is_counted {
                        components.push((number, number_indices.clone()));
                        is_counted = false;
                    }
                }
            } else if number != 0 {
                if is_counted {
                    components.push((number, number_indices.clone()));
                    is_counted = false;
                }
                number = 0;
            }
        }
    }

    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if is_star(*c) {
                let mut numbers: [(u32, bool); 8] = [(0, false); 8];
                let mut number_count = 0;
                if i > 0 {
                    if j > 0 {
                        numbers[0] = search_in_grid(&components, i - 1, j - 1);
                    }
                    numbers[1] = search_in_grid(&components, i - 1, j);
                    if j < row.len() - 1 {
                        numbers[2] = search_in_grid(&components, i - 1, j + 1);
                    }
                }
                if j > 0 {
                    numbers[3] = search_in_grid(&components, i, j - 1);
                }
                if j < row.len() - 1 {
                    numbers[4] = search_in_grid(&components, i, j + 1);
                }
                if i < grid.len() - 1 {
                    if j > 0 {
                        numbers[5] = search_in_grid(&components, i + 1, j - 1);
                    }
                    numbers[6] = search_in_grid(&components, i + 1, j);
                    if j < row.len() - 1 {
                        numbers[7] = search_in_grid(&components, i + 1, j + 1);
                    }
                }
                if numbers[6].1 {
                    numbers[5].1 = false;
                    numbers[7].1 = false;
                }
                if numbers[1].1 {
                    numbers[0].1 = false;
                    numbers[2].1 = false;
                }

                for (_, is_num) in numbers {
                    if is_num {
                        number_count += 1;
                    }
                }
                if number_count == 2 {
                    let mut num1 = 0;
                    let mut num2 = 0;
                    for (num, is_num) in numbers {
                        if is_num {
                            if num1 == 0 {
                                num1 = num;
                            } else {
                                num2 = num;
                            }
                        }
                    }
                    result += num1 * num2;
                }
            }
        }
    }
    result
}

fn is_number(input: char) -> bool {
    '0' <= input && input <= '9'
}

fn is_period(input: char) -> bool {
    input == '.'
}

fn is_symbol(input: char) -> bool {
    !(is_number(input) || is_period(input))
}

fn is_star(input: char) -> bool {
    input == '*'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_case = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let result = part2(test_case);
        assert_eq!(result, 467835u32);
    }
}
