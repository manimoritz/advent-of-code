fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut final_number: u128 = 0;
    let mut first_number: Option<char> = None;
    let mut last_number: Option<char> = None;
    for l in input.lines() {
        for c in l.chars() {
            if c.is_numeric() {
                if first_number.is_none() {
                    first_number = Some(c);
                }
                last_number = Some(c);
            }
        }
        final_number += (first_number.unwrap().to_string() + &last_number.unwrap().to_string()).parse::<u128>().unwrap();
        first_number = None;
    }
    final_number.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_case = 
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        let result = part1(test_case);
        assert_eq!(result, "142".to_string());
    }
}