fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let lines = input.lines().take(2).collect::<Vec<&str>>();
    let times = lines[0][5..]
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let distances = lines[1][9..]
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let numbers = times.len();
    let mut td_pairs = vec![];
    for i in 0..numbers {
        td_pairs.push((times[i], distances[i]));
    }
    let mut result = 1;
    for (time, distance) in td_pairs {
        let mut count = 0;
        for i in 0..=time {
            if (i * (time - i)) > distance {
                count += 1;
            }
        }
        result *= dbg!(count);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_case = "Time:      7  15   30
Distance:  9  40  200";
        let result = part1(test_case);
        assert_eq!(result, 288u32);
    }
}
