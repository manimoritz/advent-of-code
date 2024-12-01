use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    let mut num_counts = HashMap::new();

    for l in input.lines() {
        let numbers = l.split_whitespace().collect::<Vec<&str>>();

        let left_num = numbers[0].parse::<i32>().unwrap();

        left_list.push(left_num);
        right_list.push(numbers[1].parse::<i32>().unwrap());

        num_counts.insert(left_num, 0);
    }

    for num in right_list {
        if let Some(left_number) = num_counts.get(&num) {
            num_counts.insert(num, left_number + num);
        }
    }

    let mut total = 0;

    for num in left_list {
        if let Some(left_number) = num_counts.get(&num) {
            total += left_number;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = part2(test_case);
        assert_eq!(result, 31i32);
    }
}
