fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for l in input.lines() {
        let numbers = l.split_whitespace().collect::<Vec<&str>>();
        left_list.push(numbers[0].parse::<i32>().unwrap());
        right_list.push(numbers[1].parse::<i32>().unwrap());
    }
    left_list.sort();
    right_list.sort();
    let paired_list = left_list.iter().zip(right_list.iter());
    let distance_list = paired_list.into_iter().map(|(a, b)| (a - b).abs());
    let total_distance: i32 = distance_list.sum();
    total_distance
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
        let result = part1(test_case);
        assert_eq!(result, 11i32);
    }
}
