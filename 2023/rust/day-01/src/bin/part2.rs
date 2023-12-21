fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let digits = [
        "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
        "1", "2", "3", "4", "5",
        "6", "7", "8", "9"
    ];
    let mut final_number: u128 = 0;

    for l in input.lines() {
        let mut first_number = ("0".to_string(), -1);
        let mut last_number = ("0".to_string(), -1);
        for s in digits {
            first_number = match l.find(s) {
                Some(position) if first_number.1 == -1i32 || position < first_number.1 as usize => (
                    match s {
                        "one" => "1".to_string(),
                        "two" => "2".to_string(),
                        "three" => "3".to_string(),
                        "four" => "4".to_string(),
                        "five" => "5".to_string(),
                        "six" => "6".to_string(),
                        "seven" => "7".to_string(),
                        "eight" => "8".to_string(),
                        "nine" => "9".to_string(),
                        _ => String::from(s)
                    },
                    position as i32),
                _ => first_number
            };
            last_number = match l.rfind(s) {
                Some(position) if last_number.1 == -1i32 || position > last_number.1 as usize => (
                    match s {
                        "one" => "1".to_string(),
                        "two" => "2".to_string(),
                        "three" => "3".to_string(),
                        "four" => "4".to_string(),
                        "five" => "5".to_string(),
                        "six" => "6".to_string(),
                        "seven" => "7".to_string(),
                        "eight" => "8".to_string(),
                        "nine" => "9".to_string(),
                        _ => String::from(s)
                    },
                    position as i32),
                _ => last_number
            };
        }
        final_number += (first_number.0 + &last_number.0).parse::<u128>().unwrap();
    }
    final_number.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_case = 
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";
        let result = part2(test_case);
        assert_eq!(result, "281".to_string());
    }
}