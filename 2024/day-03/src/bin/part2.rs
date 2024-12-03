use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let do_mul = input
        .split("do()")
        .map(|a| a.split("don't()").collect::<Vec<&str>>()[0]);

    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut result = 0;

    for line in do_mul {
        for (_s, [a, b]) in regex.captures_iter(line).map(|c| c.extract()) {
            result += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part2(test_case);
        assert_eq!(result, 48i32);
    }
}
