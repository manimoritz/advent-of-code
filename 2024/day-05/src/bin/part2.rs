use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut second_input: Vec<&str> = Vec::new();
    let mut rules: Vec<Regex> = Vec::new();

    let mut second_input_reached: bool = false;

    for l in input.lines() {
        if l == "" {
            second_input_reached = true;
            continue;
        } else if second_input_reached {
            second_input.push(l);
        } else {
            let both_numbers = l.split('|').collect::<Vec<&str>>();
            // Parse each line as a regex, where if the second number appears,
            // the first one is not allowed to appear afterwords
            let regex_string = format!(r",?({}),(?:.*,)?({}),?", both_numbers[1], both_numbers[0]);
            let regex = Regex::new(&regex_string).unwrap();

            rules.push(regex);
        }
    }

    let mut result = 0;

    for page_order in second_input {
        let mut corrected_page_order = page_order.to_string();
        let mut valid_from_beginning = true;
        let mut repeat = true;

        while repeat {
            let mut valid = true;

            for rule in &rules {
                match rule.captures(&corrected_page_order) {
                    Some(cap) => {
                        valid_from_beginning = false;
                        valid = false;
                        corrected_page_order =
                            swap_numbers(&corrected_page_order, &cap[1], &cap[2]);
                        break;
                    }
                    None => {
                        continue;
                    }
                }
            }
            repeat = !valid;
        }
        if !valid_from_beginning {
            result += find_middle_number(&corrected_page_order);
        }
    }

    result
}

fn swap_numbers(line: &str, first_number: &str, second_number: &str) -> String {
    let mut new_line = line.replace(first_number, "tmp");
    new_line = new_line.replace(second_number, first_number);
    new_line = new_line.replace("tmp", second_number);
    new_line
}

fn find_middle_number(number_string: &str) -> i32 {
    let numbers = number_string
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<i32>>();
    numbers[numbers.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let result = part2(test_case);
        assert_eq!(result, 123i32);
    }
}
