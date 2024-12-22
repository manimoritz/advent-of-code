#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input, 100, 103, 101);
    dbg!(output);
}

fn part1(input: &str, seconds: i64, height: i64, width: i64) -> i64 {
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

    for _ in 0..seconds {
        for j in 0..positions.len() {
            simulate_sec(&mut positions[j], velocities[j], height, width);
        }
    }

    let mut quad_count = (0, 0, 0, 0);

    for pos in positions.iter() {
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

    quad_count.0 * quad_count.1 * quad_count.2 * quad_count.3
}

fn simulate_sec(pos: &mut (i64, i64), vel: (i64, i64), height: i64, width: i64) {
    // Check if right or left reached
    if pos.0 + vel.0 >= width {
        pos.0 = pos.0 + vel.0 - width;
    } else if pos.0 + vel.0 < 0 {
        pos.0 = width + (pos.0 + vel.0);
    } else {
        pos.0 += vel.0;
    }
    // Check if bottom or top reached
    if pos.1 + vel.1 >= height {
        pos.1 = pos.1 + vel.1 - height;
    } else if pos.1 + vel.1 < 0 {
        pos.1 = height + (pos.1 + vel.1);
    } else {
        pos.1 += vel.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let result = part1(test_case, 100, 7, 11);
        assert_eq!(result, 12i64);
    }

    #[test]
    fn test2() {
        let mut test_pos = (2, 4);
        let test_vel = (2, -3);

        simulate_sec(&mut test_pos, test_vel, 7, 11);
        assert_eq!(test_pos, (4, 1));
        simulate_sec(&mut test_pos, test_vel, 7, 11);
        assert_eq!(test_pos, (6, 5));
        simulate_sec(&mut test_pos, test_vel, 7, 11);
        assert_eq!(test_pos, (8, 2));
        simulate_sec(&mut test_pos, test_vel, 7, 11);
        assert_eq!(test_pos, (10, 6));
        simulate_sec(&mut test_pos, test_vel, 7, 11);
        assert_eq!(test_pos, (1, 3));
    }
}
