fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

struct Bag {
    red: u32,
    green: u32,
    blue: u32
}

fn part1(input: &str) -> u32 {
    const CONSTRAINTS: Bag = Bag{red: 12, green: 13, blue: 14};
    let mut result = 0;
    for l in input.lines() {
        let mut is_valid = true;
        let game = l.split(": ").collect::<Vec<&str>>();
        let id = game[0].split_whitespace().collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let rounds = game[1].split("; ").collect::<Vec<&str>>();
        for r in rounds {
            let cubes = r.split(", ").collect::<Vec<&str>>();
            for c in cubes {
                let num_color = c.split_whitespace().collect::<Vec<&str>>();
                let num = num_color[0].parse::<u32>().unwrap();
                match num_color[1] {
                    "red" => if num > CONSTRAINTS.red {is_valid = false},
                    "green" => if num > CONSTRAINTS.green {is_valid = false},
                    "blue" => if num > CONSTRAINTS.blue {is_valid = false},
                    _ => unreachable!()
                }
            }
        }
        if is_valid {
            result += id;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_case = 
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part1(test_case);
        assert_eq!(result, 8u32);
    }
}