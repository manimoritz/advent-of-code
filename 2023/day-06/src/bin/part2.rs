fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u64 {
    let lines = input.lines().take(2).collect::<Vec<&str>>();
    let time = lines[0][5..]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let distance = lines[1][9..]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let mut count = 0;
    for i in 0..=time {
        if (i * (time - i)) > distance {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_case = "Time:      7  15   30
Distance:  9  40  200";
        let result = part2(test_case);
        assert_eq!(result, 71503u64);
    }
}
