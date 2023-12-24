fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
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

fn part1(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut result = 0;
    for (i, l) in input.lines().enumerate() {
        grid.push(Vec::new());
        for c in l.chars() {
            grid[i].push(c);
        }
    }

    for (i, row) in grid.iter().enumerate() {
        let mut number = 0;
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

                number = number * 10 + (*c as i32) - ('0' as i32);

                if j == row.len() - 1 {
                    if is_counted {
                        result += number;
                        is_counted = false;
                    }
                }
            } else if number != 0 {
                if is_counted {
                    result += number;
                    is_counted = false;
                }
                number = 0;
            }
        }
    }
    result as u32
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_case = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let result = part1(test_case);
        assert_eq!(result, 4361u32);
    }
}
