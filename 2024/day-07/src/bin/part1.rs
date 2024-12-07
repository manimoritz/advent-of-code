fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

// Task:
// Given a result and a list of numbers, check if the numbers yield the result with the
// insertion of (+) and (*) between them.
// Return the sum of all the results for which this condition applies
//
// Naive approach:
// Loop through every possible permutation of operator placements and check if at least one
// permutation yields the result.
// Save the results in a list and sum them up.
fn part1(input: &str) -> i64 {
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
    }
    for i in 0..result.len() / 2 {
        result[i * 2].push('+');
        result[i * 2 + 1].push('*');
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
            _ => {
                panic!();
            }
        }
    }
    return result;
}

#[allow(dead_code)]
// This is not needed because we don't use real mathematical rules
fn calculate(numbers: &Vec<i64>, operators: &Vec<char>) -> i64 {
    if numbers.len() == 0 {
        return 0;
    }
    if operators.len() == 0 {
        return numbers[0];
    }
    let mut remaining_numbers = numbers.clone();
    let remaining_operators = operators.clone();
    let current_multiplier = remaining_numbers.remove(0);
    get_result(
        0,
        current_multiplier,
        &remaining_numbers,
        &remaining_operators,
    )
}

// This is not needed because we don't use real mathematical rules
fn get_result(
    current_result: i64,
    current_multiplier: i64,
    remaining_numbers: &Vec<i64>,
    remaining_operators: &Vec<char>,
) -> i64 {
    if remaining_numbers.len() == 0 {
        return current_result + current_multiplier;
    }

    let mut remaining_numbers = remaining_numbers.clone();
    let mut remaining_operators = remaining_operators.clone();
    let current_operator = remaining_operators.remove(0);
    let current_number = remaining_numbers.remove(0);

    match current_operator {
        '+' => {
            return current_multiplier
                + get_result(
                    current_result,
                    current_number,
                    &remaining_numbers,
                    &remaining_operators,
                );
        }
        '*' => {
            return get_result(
                current_result,
                current_multiplier * current_number,
                &remaining_numbers,
                &remaining_operators,
            );
        }
        _ => {
            panic!();
        }
    }
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
        let result = part1(test_case);
        assert_eq!(result, 3749i64);
    }

    #[test]
    fn test2() {
        let mut result = calculate(&vec![5, 4, 2], &vec!['*', '+']);
        assert_eq!(result, 22i64);

        result = calculate(&vec![5, 4, 2], &vec!['*', '*']);
        assert_eq!(result, 40i64);

        result = calculate(&vec![5, 4, 2, 30, 8], &vec!['*', '*', '*', '+']);
        assert_eq!(result, 1208i64);

        result = calculate(&vec![5, 4, 2, 30, 8], &vec!['*', '*', '+', '*']);
        assert_eq!(result, 280i64);
    }
}
