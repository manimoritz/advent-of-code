fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(_input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("");
        assert_eq!(result, "4".to_string());
    }
}
