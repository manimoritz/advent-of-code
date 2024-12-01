use std::cmp::max;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    for l in input.lines() {
        let mut min_bag = Bag {
            red: 0,
            green: 0,
            blue: 0,
        };
        let game = l.split(": ").collect::<Vec<&str>>();
        let rounds = game[1].split("; ").collect::<Vec<&str>>();
        for r in rounds {
            let cubes = r.split(", ").collect::<Vec<&str>>();
            for c in cubes {
                let num_color = c.split_whitespace().collect::<Vec<&str>>();
                let num = num_color[0].parse::<u32>().unwrap();
                match num_color[1] {
                    "red" => min_bag.red = max(min_bag.red, num),
                    "green" => min_bag.green = max(min_bag.green, num),
                    "blue" => min_bag.blue = max(min_bag.blue, num),
                    _ => unreachable!(),
                }
            }
        }
        let power = min_bag.red * min_bag.green * min_bag.blue;
        result += power;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_case = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part2(test_case);
        assert_eq!(result, 2286u32);
    }
}
