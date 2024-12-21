#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

// Naive approach:
// For every multiple of "Button A" pushes, try all multiples of "Button B" pushes.
// The maximum amount of pushes per button is 100.
// With this, find every combination of A & B pushes and take the one that costs the least.
fn part1(input: &str) -> i64 {
    let mut btn_as: Vec<(i64, i64)> = Vec::new();
    let mut btn_bs: Vec<(i64, i64)> = Vec::new();
    let mut prizes: Vec<(i64, i64)> = Vec::new();

    for (i, l) in input.lines().enumerate() {
        if i % 4 == 3 {
            continue;
        }

        let mut skipped_entries = 2;
        if i % 4 == 2 {
            skipped_entries = 1;
        }
        let coords: Vec<&str> = l.split_whitespace().skip(skipped_entries).collect();

        let (x, y): (i64, i64) = (
            coords[0][2..coords[0].len() - 1]
                .parse()
                .unwrap_or_else(|_| {
                    eprintln!(
                        "Something went wrong when parsing the first coordinate on line {}.",
                        i,
                    );
                    0
                }),
            coords[1][2..].parse().unwrap_or_else(|_| {
                eprintln!(
                    "Something went wrong when parsing the second coordinate on line {}.",
                    i,
                );
                0
            }),
        );

        // Button A
        if i % 4 == 0 {
            btn_as.push((x, y));
        }
        // Button B
        if i % 4 == 1 {
            btn_bs.push((x, y));
        } // Prize
        if i % 4 == 2 {
            prizes.push((x, y));
        }
    }

    let mut min_tokens_per_round: Vec<i64> = Vec::new();

    for i in 0..prizes.len() {
        let possibilities = find_all_btn_combos(btn_as[i], btn_bs[i], prizes[i]);
        let min_tokens = find_min_tokens(possibilities);
        min_tokens_per_round.push(min_tokens);
    }

    min_tokens_per_round.iter().sum()
}

fn find_all_btn_combos(btn_a: (i64, i64), btn_b: (i64, i64), prize: (i64, i64)) -> Vec<(i64, i64)> {
    let mut combos: Vec<(i64, i64)> = Vec::new();

    for i in 0..100 {
        for j in 0..100 {
            if add_pair_and_pair(
                mult_pair_and_scalar(btn_a, i),
                mult_pair_and_scalar(btn_b, j),
            ) == prize
            {
                combos.push((i, j));
            }
        }
    }

    combos
}

fn find_min_tokens(pairs: Vec<(i64, i64)>) -> i64 {
    if let Some(first_pair) = pairs.first() {
        let mut min = first_pair.0 * 3 + first_pair.1;

        for pair in pairs.iter() {
            let tokens = pair.0 * 3 + pair.1;
            if tokens < min {
                min = tokens;
            }
        }

        min
    } else {
        0
    }
}

fn add_pair_and_pair(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

fn mult_pair_and_scalar(a: (i64, i64), b: i64) -> (i64, i64) {
    (a.0 * b, a.1 * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let result = part1(test_case);
        assert_eq!(result, 480i64);
    }
}
