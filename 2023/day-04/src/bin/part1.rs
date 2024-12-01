fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let mut points = 0;
        let abb_line = line.split(":").collect::<Vec<&str>>()[1];
        let sides = abb_line.split("|").collect::<Vec<&str>>();
        let left_side_nums = sides[0]
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let right_side_nums = sides[1]
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        for l in left_side_nums {
            let right_side_nums_clone = right_side_nums.clone();
            for r in right_side_nums_clone {
                if l == r {
                    if points != 0 {
                        points *= 2;
                    } else {
                        points = 1;
                    }
                }
            }
        }
        result += points;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut test_case = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part1(test_case);
        assert_eq!(result, 13u32);
        test_case = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = part1(test_case);
        assert_eq!(result, 8u32);
        test_case = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let result = part1(test_case);
        assert_eq!(result, 2u32);
        test_case = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let result = part1(test_case);
        assert_eq!(result, 2u32);
        test_case = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let result = part1(test_case);
        assert_eq!(result, 1u32);
        test_case = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        let result = part1(test_case);
        assert_eq!(result, 0u32);
        test_case = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part1(test_case);
        assert_eq!(result, 0u32);
    }
}
