use core::panic;

struct Node {
    value: String,
    left_node: String,
    right_node: String,
}

impl Node {
    fn new(value: String, left: String, right: String) -> Node {
        Node {
            value: value,
            left_node: left,
            right_node: right,
        }
    }
}

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Graph {
        Graph { nodes: vec![] }
    }
    fn search_node(&self, target: &str) -> usize {
        let nodes = &self.nodes;
        let len = nodes.len();
        for i in 0..len {
            if nodes[i].value == target {
                return i;
            }
        }
        panic!("The node {} is not in the graph", target)
    }
    fn add_node(&mut self, value: &str, left: &str, right: &str) {
        self.nodes.push(Node::new(
            value.to_string(),
            left.to_string(),
            right.to_string(),
        ));
    }
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut graph = Graph::new();
    let mut lines = input.lines();
    let direction = lines.next().unwrap();
    lines.next();
    for line in lines {
        graph.add_node(&line[0..3], &line[7..10], &line[12..15]);
    }
    // for node in graph.nodes {
    //     println!("{}", [node.value + " = (" + &node.left_node + ", " + &node.right_node + ")"].concat())
    // }
    let mut current = "AAA";
    let mut current_idx = graph.search_node("AAA");
    let direction_len = direction.len();
    let mut counter = 0;
    loop {
        if current == "ZZZ" {
            return counter;
        }
        let dir_idx = counter as usize % direction_len;
        if &direction[dir_idx..dir_idx + 1] == "L" {
            current = &graph.nodes[current_idx].left_node;
        } else if &direction[dir_idx..dir_idx + 1] == "R" {
            current = &graph.nodes[current_idx].right_node;
        }
        current_idx = graph.search_node(current);
        counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_case = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(test_case);
        assert_eq!(result, 2u32);
    }
    #[test]
    fn test2() {
        let test_case = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(test_case);
        assert_eq!(result, 6u32);
    }
}
