fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i64 {
    let mut possible_results = Vec::new();

    for l in input.lines() {
        let (result_string, numbers_string) = l.split_once(':').unwrap();

        let result = result_string.parse::<i64>().unwrap();
        let numbers: Vec<i64> = numbers_string
            .split_whitespace()
            .filter_map(|x| x.parse::<i64>().ok())
            .collect();

        let permutations = generate_permutations(numbers.len() - 1);

        possible_results.push(check_possibility(result, numbers, permutations));
    }

    return possible_results.iter().sum();
}

fn generate_permutations(length: usize) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    if length == 0 {
        result.push(Vec::new());
        return result;
    }
    let last_result = generate_permutations(length - 1);

    for i in 0..last_result.len() {
        result.push(last_result[i].clone());
        result.push(last_result[i].clone());
        result.push(last_result[i].clone());
    }
    for i in 0..result.len() / 3 {
        result[i * 3].push('+');
        result[i * 3 + 1].push('*');
        result[i * 3 + 2].push('|');
    }
    return result;
}

fn check_possibility(result: i64, numbers: Vec<i64>, permutations: Vec<Vec<char>>) -> i64 {
    for permutation in permutations {
        if result == calculate_dumb(&numbers, &permutation) {
            return result;
        }
    }
    return 0;
}

fn calculate_dumb(numbers: &Vec<i64>, operators: &Vec<char>) -> i64 {
    if numbers.len() == 0 {
        return 0;
    }
    let mut result = numbers[0];
    for i in 0..operators.len() {
        match operators[i] {
            '+' => {
                result += numbers[i + 1];
            }
            '*' => {
                result *= numbers[i + 1];
            }
            '|' => {
                result = (result.to_string() + &numbers[i + 1].to_string())
                    .parse::<i64>()
                    .unwrap();
            }
            _ => {
                panic!();
            }
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let result = part2(test_case);
        assert_eq!(result, 11387i64);
    }
}
