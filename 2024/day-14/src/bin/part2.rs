#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input, 103, 101);
    dbg!(output);
}

fn part2(input: &str, height: i64, width: i64) -> i64 {
    let seconds = width * height;

    let mut positions: Vec<(i64, i64)> = Vec::new();
    let mut velocities: Vec<(i64, i64)> = Vec::new();

    for l in input.lines() {
        let p_and_v: Vec<&str> = l.split_whitespace().collect();

        let p: Vec<i64> = p_and_v[0][2..]
            .split(',')
            .map(|c| {
                c.parse::<i64>().unwrap_or_else(|_| {
                    eprintln!("Something went wrong");
                    0
                })
            })
            .collect();

        let v: Vec<i64> = p_and_v[1][2..]
            .split(',')
            .map(|n| {
                n.parse::<i64>().unwrap_or_else(|_| {
                    eprintln!("Something went wrong");
                    0
                })
            })
            .collect();

        positions.push((p[0], p[1]));
        velocities.push((v[0], v[1]));
    }

    let mut best_quad_count = i64::MAX;
    let mut best_iteration = 0;

    for i in 0..seconds {
        let mut pos_copy = positions.clone();
        for j in 0..positions.len() {
            pos_copy[j] = (
                (pos_copy[j].0 + velocities[j].0 * i) % width,
                (pos_copy[j].1 + velocities[j].1 * i) % height,
            );
            if pos_copy[j].0 < 0 {
                pos_copy[j].0 += width;
            }
            if pos_copy[j].1 < 0 {
                pos_copy[j].1 += height;
            }
        }

        let mut quad_count = (0, 0, 0, 0);

        for pos in pos_copy.iter() {
            let w_mid = width / 2;
            let h_mid = height / 2;
            if pos.0 < w_mid && pos.1 < h_mid {
                quad_count.0 += 1;
            } else if pos.0 > w_mid && pos.1 < h_mid {
                quad_count.1 += 1;
            } else if pos.0 > w_mid && pos.1 > h_mid {
                quad_count.2 += 1;
            } else if pos.0 < w_mid && pos.1 > h_mid {
                quad_count.3 += 1;
            }
        }

        let current_quad_count = quad_count.0 * quad_count.1 * quad_count.2 * quad_count.3;

        if current_quad_count < best_quad_count {
            best_quad_count = current_quad_count;
            best_iteration = i;
        }
    }

    best_iteration
}
