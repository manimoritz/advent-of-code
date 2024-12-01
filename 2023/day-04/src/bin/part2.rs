fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    let mut cards = vec![];
    for line in input.lines() {
        cards.push((line.split(":").collect::<Vec<&str>>()[0], 1));
    }
    for line in input.lines() {
        let mut cards_after = 0;
        let card = line.split(":").collect::<Vec<&str>>()[0];
        let card_index = card.split_ascii_whitespace().collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap()
            - 1;
        let scratches = line.split(":").collect::<Vec<&str>>()[1];
        let sides = scratches.split("|").collect::<Vec<&str>>();
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
                    cards_after += 1;
                }
            }
        }
        for i in 1..=cards_after {
            if ((card_index + i) as usize) < cards.len() {
                cards[(card_index + i) as usize].1 += cards[(card_index) as usize].1;
                cards[(card_index + i) as usize].1;
            }
        }
    }
    for c in cards {
        result += c.1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_case = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part2(test_case);
        assert_eq!(result, 30u32);
    }
}
