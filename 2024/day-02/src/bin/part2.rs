fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut result = 0i32;
    for l in input.lines() {
        let mut nums: Vec<i32> = Vec::new();
        let line_elems = l.split_whitespace().collect::<Vec<&str>>();
        for elem in line_elems.iter() {
            nums.push(elem.parse::<i32>().unwrap());
        }

        let paired_list = get_paired_list(&nums);

        let mut valid = false;

        if is_valid(&paired_list) {
            valid = true;
        } else {
            for i in 0..nums.len() {
                let mut permutation = nums.clone();
                permutation.remove(i);

                let perm_paired_list = get_paired_list(&permutation);

                if is_valid(&perm_paired_list) {
                    valid = true;
                    break;
                }
            }
        }

        if valid {
            result += 1;
        }
    }
    result
}

fn get_paired_list(list: &Vec<i32>) -> Vec<(i32, i32)> {
    let paired_list = list
        .iter()
        .zip(list.iter().skip(1))
        .map(|(&a, &b)| (a, b))
        .collect();

    paired_list
}

fn is_valid(list: &Vec<(i32, i32)>) -> bool {
    let valid = (list.len()
        == list
            .clone()
            .into_iter()
            .filter(|&(a, b)| a > b && a <= b + 3)
            .collect::<Vec<_>>()
            .len())
        || (list.len()
            == list
                .clone()
                .into_iter()
                .filter(|&(a, b)| a < b && a + 3 >= b)
                .collect::<Vec<_>>()
                .len());

    valid
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
        let result = part2(test_case);
        assert_eq!(result, 4i32);
    }
}
