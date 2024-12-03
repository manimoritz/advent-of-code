fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut result = 0i32;
    for l in input.lines() {
        let mut nums: Vec<i32> = Vec::new();
        let line_elems = l.split_whitespace().collect::<Vec<&str>>();
        for elem in line_elems.iter() {
            nums.push(elem.parse::<i32>().unwrap());
        }

        let new_list: Vec<(i32, i32)> = nums
            .iter()
            .zip(nums.iter().skip(1))
            .map(|(&a, &b)| (a, b))
            .collect();

        let valid = (new_list.len()
            == new_list
                .clone()
                .into_iter()
                .filter(|&(a, b)| a > b && a <= b + 3)
                .collect::<Vec<_>>()
                .len())
            || (new_list.len()
                == new_list
                    .clone()
                    .into_iter()
                    .filter(|&(a, b)| a < b && a + 3 >= b)
                    .collect::<Vec<_>>()
                    .len());

        if valid {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let result = part1(test_case);
        assert_eq!(result, 2i32);
    }
}
