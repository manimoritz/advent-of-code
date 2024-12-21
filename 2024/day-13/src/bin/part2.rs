#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

// Refined approach:
// for coords x and y
// while n * (coord x of Button B) < prize.x
// Check if (prize.x - (n * (coord x of Button B))) is divisible by (coord x of Button A).
// If we exit the while loop without having found anything, there is no way to win the prize.
//
// => Still too inefficient
//
// Best approach:
// Linear algebra.
// ax * s + bx * t = px
// ay * s + by * t = py
//
// axbys + bxbyt = pxby
// aybxs + bxbyt = pybx
//
// axbys - aybxs = pxby - pybx
//
// (axby - aybx) * s = pxby - pybx
//
// s = (pxby - pybx) / (axby - aybx) | (axby - aybx) != 0
//
// t = (px - axs) / bx
fn part2(input: &str) -> i64 {
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
            prizes.push((x + 10000000000000, y + 10000000000000));
        }
    }

    let mut min_tokens_per_round: Vec<i64> = Vec::new();

    for i in 0..prizes.len() {
        let possibility = find_btn_combo(btn_as[i], btn_bs[i], prizes[i]);
        let min_tokens = calculate_tokens(possibility);
        min_tokens_per_round.push(min_tokens);
    }

    min_tokens_per_round.iter().sum()
}

fn find_btn_combo(btn_a: (i64, i64), btn_b: (i64, i64), prize: (i64, i64)) -> (i64, i64) {
    // s = (pxby - pybx) / (axby - aybx) | (axby - aybx) != 0
    // t = (px - axs) / bx
    let ax = btn_a.0;
    let ay = btn_a.1;
    let bx = btn_b.0;
    let by = btn_b.1;
    let px = prize.0;
    let py = prize.1;

    // Check if axby = aybx.
    if (ax * by - ay * bx) == 0 {
        return (0, 0);
    }

    // Check if s is an integer. If not, there is no solution.
    if (px * by - py * bx) % (ax * by - ay * bx) != 0 {
        return (0, 0);
    }

    let s = (px * by - py * bx) / (ax * by - ay * bx);

    // Check if t is an integer. If not, there is no solution.
    if (px - ax * s) % bx != 0 {
        return (0, 0);
    }

    let t = (px - ax * s) / bx;

    (s, t)
}

fn calculate_tokens(pair: (i64, i64)) -> i64 {
    pair.0 * 3 + pair.1
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
        let result = part2(test_case);
        assert_eq!(result, 875318608908i64);
    }
}
